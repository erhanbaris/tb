use std::{cell::{Cell, RefCell}, fs::File, io::Read, path::PathBuf};

#[cfg(not(test))] 
use log::{info, warn}; // Use log crate when building application
 
#[cfg(test)]
use std::{println as info, println as warn}; // Workaround to use prinltn! for logs.
use thiserror::Error;

use crate::parser::{print_error, Context, Token, TokenType};

#[derive(Debug, Error)]
pub enum AstGeneratorError {
    #[error("Syntax issue")]
    SyntaxIssue {
        #[allow(dead_code)] line: usize,
        #[allow(dead_code)] column: usize,
        #[allow(dead_code)] end: usize,
        #[allow(dead_code)] message: String
    },
    
    #[error("Out of scope")]
    OutOfScope,
    
    #[error("Internal error")]
    InternalError,

    #[error("IO Error ({0})")]
    IOError(#[from] std::io::Error),

    #[error("'{0}' reference already defined)")]
    ReferenceAlreadyDefined(String)
}

impl AstGeneratorError {
    pub fn syntax_issue(context: &Context, token_index: usize, message: String) -> Self {
        let token_info = &context.tokens.borrow()[token_index];
        AstGeneratorError::SyntaxIssue { column: token_info.column, end: token_info.end, line: token_info.line, message  }
    }
}

#[derive(Debug)]
pub struct AstGenerator {
    pub index: Cell<usize>,
    pub(crate) size: Cell<usize>,
}

impl AstGenerator {
    pub fn new() -> Self {
        Self {
            index: Cell::new(0),
            size: Cell::new(0),
        }
    }
    
    fn empty_check(&self) -> Result<(), AstGeneratorError> {
        match self.index.get() >= self.size.get() {
            true => Err(AstGeneratorError::OutOfScope),
            false => Ok(()),
        }
    }

    fn eat(&self) -> Result<usize, AstGeneratorError> {
        self.empty_check()?;
        self.index.set(self.index.get() + 1);
        Ok(self.index.get() - 1)
    }

    fn peek(&self)-> Result<usize, AstGeneratorError> {
        self.empty_check()?;
        Ok(self.index.get())
    }
    
    fn eat_expected(&self, context: &Context, token_type: TokenType, error: AstGeneratorError) -> Result<(), AstGeneratorError> {
        let token_index = self.eat()?;
        let token = &context.tokens.borrow()[token_index];

        if TokenType::from(&token.token) != token_type {
            return Err(error);
        }
        Ok(())
    }

    fn eat_space(&self, context: &Context) -> Result<(), AstGeneratorError> {
        let token_index= self.eat()?;
        let token = &context.tokens.borrow()[token_index];
        match token.token {
            Token::Space(_) => Ok(()),
            _ => Err(AstGeneratorError::syntax_issue(context, token_index, "Expected space".to_string()))
        }
    }

    fn cleanup_space(&self, context: &Context) -> Result<(), AstGeneratorError> {
        if let Ok(token_index) = self.peek() {
            let token = &context.tokens.borrow()[token_index];
            if let Token::Space(_) = token.token {
                let _ = self.eat();
            }
        }
        Ok(())
    }
    
    fn eat_assign(&self, context: &Context) -> Result<(), AstGeneratorError> {
        let token_index= self.eat()?;
        let token = &context.tokens.borrow()[token_index];
        match token.token {
            Token::Assign => Ok(()),
            _ => Err(AstGeneratorError::syntax_issue(context, token_index, "Expected assign".to_string()))
        }
    }

    fn eat_text(&self, context: &Context) -> Result<String, AstGeneratorError> {
        let token_index= self.eat()?;
        let token = &context.tokens.borrow()[token_index];
        match &token.token {
            Token::Keyword(text) => Ok(text.clone()),
            _ => Err(AstGeneratorError::syntax_issue(context, token_index, "Expected text".to_string()))
        }
    }

    fn generate_branch(&self, context: &Context, token_index: usize, name: &str, branch_type: BranchType) -> Result<(), AstGeneratorError> {
        context.add_ast(token_index, Ast::Branch(name.to_owned(), branch_type));
        Ok(())
    }

    fn generate_assign(&self, context: &Context, _: usize, name: &String) -> Result<(), AstGeneratorError> {
        self.cleanup_space(context)?;
        self.eat_assign(context)?;
        self.cleanup_space(context)?;

        let values = self.parse_list(context, |_| true)?;
        let has_reference = context.references.borrow_mut().insert(name.to_owned(), values).is_some();

        if has_reference {
            return Err(AstGeneratorError::ReferenceAlreadyDefined(name.to_owned()));
        }
        Ok(())
    }

    fn generate_code_block(&self, context: &Context, token_index: usize, positon: usize) -> Result<(), AstGeneratorError> {

        if INSTS_SIZE[positon] == 1 {
            context.add_ast(token_index,Ast::InstrImplied(positon));
        }

        else if BRANCH_INSTS.contains(&positon) {
            // Branch inst
            self.eat_space(context)?;
            let value = self.parse_instr_value(context)?;

            match value.value {
                InstrValue::Byte(_) => context.add_ast(token_index, Ast::Instr(positon, value)),
                InstrValue::Reference(_) => context.add_ast(token_index, Ast::Instr(positon, value)),
                InstrValue::LocalReference(_) => context.add_ast(token_index, Ast::Instr(positon, value)),
                _ => return Err(AstGeneratorError::syntax_issue(context, token_index, "Relative number or branch name expected".to_string()))
            }
        }

        else {
            self.eat_space(context)?;
            let value = self.parse_instr_value(context)?;
            context.add_ast(token_index, Ast::Instr(positon, value));
        }
        
        Ok(())
    }
    
    fn inline_generate(&self, context: &Context) -> Result<(), AstGeneratorError> {
        self.size.set(context.tokens.borrow().len());
        let mut token_index = 0;

        while self.size.get() > self.index.get() {
            {
                token_index = self.eat()?;
                let tokens = context.tokens.borrow();

                match &tokens.get(token_index).map(|item| &item.token) {
                    Some(Token::Keyword(keyword)) => self.generate_assign(context, token_index, keyword)?,
                    Some(Token::Comment(_)) => (),
                    Some(Token::Branch(name)) => self.generate_branch(context, token_index, name)?,
                    Some(Token::Number(_)) => return Err(AstGeneratorError::syntax_issue(context, token_index, "Number not expected".to_string())),
                    Some(Token::Float(_)) => return Err(AstGeneratorError::syntax_issue(context, token_index, "Number not expected".to_string())),
                    Some(Token::Variable(_)) => return Err(AstGeneratorError::syntax_issue(context, token_index, "Variable not expected".to_string())),
                    Some(Token::NewLine(_)) => (),
                    Some(Token::Space(_)) => (),
                    Some(Token::OpenParenthesis) => return Err(AstGeneratorError::syntax_issue(context, token_index, "'(' not expected".to_string())),
                    Some(Token::CloseParenthesis) => return Err(AstGeneratorError::syntax_issue(context, token_index, "')' not expected".to_string())),
                    Some(Token::Sharp) => return Err(AstGeneratorError::syntax_issue(context, token_index, "'#' not expected".to_string())),
                    Some(Token::Assign) => return Err(AstGeneratorError::syntax_issue(context, token_index, "'=' not expected".to_string())),
                    Some(Token::Comma) => return Err(AstGeneratorError::syntax_issue(context, token_index, "',' not expected".to_string())),
                    Some(Token::String(_)) => return Err(AstGeneratorError::syntax_issue(context, token_index, "String not expected".to_string())),
                    Some(Token::End) => break,
                    None => return Err(AstGeneratorError::InternalError)
                }
            }

            self.process_include(context, token_index)?;
        }

        Ok(())
    }
    
    pub fn generate(&self, context: Context) -> Result<Context, AstGeneratorError> {
        match self.inline_generate(&context) {
            Ok(_) => Ok(context),
            Err(error) => {
                let tokens = context.tokens.borrow();
                let token = &tokens[self.index.get() - 1];

                print_error(&context.data, &error, token.line, token.column, token.end);
                Err(error)
            }
        }
    }
}