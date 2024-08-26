use core::str;
use std::{cell::RefCell, fmt::Debug, str::Utf8Error};

use log::{error, info};
use strum_macros::EnumDiscriminants;
use thiserror::Error;


#[derive(Debug, Default)]
pub struct Context {
    pub tokens: RefCell<Vec<TokenInfo>>,
}

#[derive(Debug)]
pub struct Parser<'a> {
    pub index: usize,
    pub line: usize,
    pub column: usize,
    pub end: usize,
    size: usize,
    pub context: Context,
    pub data: &'a [u8],
}

#[derive(Debug, PartialEq, Clone)]
#[derive(EnumDiscriminants)]
#[strum_discriminants(name(TokenType))]
pub enum Token {
    Keyword(String),
    Branch(String),
    Variable(String),
    String(String),
    Comment(String),
    Number(i64),
    Float(f64),
    Assign,
    Comma,
    OpenParenthesis,
    CloseParenthesis,
    Sharp,
    NewLine(usize),
    Space(usize),
    End,
}

pub fn print_error<T: Debug>(data: &'_ [u8], error: &T, line: usize, column: usize, end: usize) {
    let mut line_index = 0;
    let mut start_index = 0;
    let mut end_index = data.len()-1;
    let mut line_found = false;

    for (index, byte) in data.iter().enumerate() {
        if *byte == b'\n' {
            line_index += 1;

            if line_index == line {
                start_index = index+1;
                line_found = true;
                continue;
            }

            if line_found {
                end_index = index;
                break;
            }
        }
    }

    println!();
    error!("{:?}", &error);
    error!("Line: {}, column: {}", line + 1, column);
    error!("{}", str::from_utf8(&data[start_index..end_index]).unwrap());
    error!("{}{}", (0..column).map(|_| " ").collect::<String>(), (0..end-column).map(|_| "^").collect::<String>());
    println!();
}

#[derive(Debug)]
#[derive(Clone)]
pub struct TokenInfo {
    pub line: usize,
    pub column: usize,
    pub token: Token,
    pub end: usize,
}

#[derive(Debug, PartialEq, Error)]
pub enum ParseError {
    #[error("Out of scope")]
    OutOfScope,
    
    #[error("Unexpeted symbol")]
    UnexpectedSymbol,
    
    #[error("Unknown token")]
    UnknownToken,
    
    #[error("Invalid number format")]
    InvalidNumberFormat,
    
    #[error("Invalid variable")]
    InvalidVariable,
    
    #[error("Invalid comment format")]
    InvalidCommentFormat,
    
    #[error("Invalid keyword")]
    InvalidKeyword,
    
    #[error("Missing colon")]
    MissingColon,
    
    #[error("Invalid directive")]
    InvalidDirective,
    
    #[error("Invalid string")]
    InvalidString,

    #[error("Invalid text format ({0})")]
    Utf8Error(#[from] Utf8Error),    
}

impl<'a> Parser<'a> {
    pub fn new(data: &'a [u8], context: Context) -> Self {
        let size = data.len();

        Self {
            index: 0,
            line: 0,
            column: 0,
            end: 0,
            size,
            context,
            data,
        }
    }

    fn add_token(&mut self, token: Token) {
        self.context.tokens.borrow_mut().push(TokenInfo {
            line: self.line,
            column: self.column,
            end: self.end,
            token,
        });
    }

    fn inner_parse(&mut self) -> Result<(), ParseError> {
        while self.size > self.index {
            let mut total_lines = 0;
            let token = self.next()?;

            if let Token::NewLine(lines) = token {
                total_lines = lines;
            }

            self.add_token(token);

            if total_lines > 0 {
                self.end = 0;
                self.line += total_lines;
            }

            self.column = self.end;
        }

        self.add_token(Token::End);
        Ok(())
    }

    pub fn parse(&mut self) -> Result<(), ParseError> {
        match self.inner_parse() {
            Ok(_) => Ok(()),
            Err(error) => {
                print_error(self.data, &error, self.line, self.column, self.end);
                Err(error)
            }
        }
    }

    fn peek(&mut self) -> Result<u8, ParseError> {
        self.empty_check()?;
        Ok(self.data[self.index])
    }

    fn peek2(&mut self) -> Result<u8, ParseError> {
        self.empty_check2()?;
        Ok(self.data[self.index+1])
    }

    fn eat(&mut self) -> Result<u8, ParseError> {
        self.empty_check()?;
        self.index += 1;
        self.end += 1;
        Ok(self.data[self.index - 1])
    }

    fn eat_expected(&mut self, byte: u8, error: ParseError) -> Result<(), ParseError> {
        if self.eat()? != byte {
            return Err(error);
        }
        Ok(())
    }

    fn empty_check(&mut self) -> Result<(), ParseError> {
        match self.index >= self.size {
            true => Err(ParseError::OutOfScope),
            false => Ok(()),
        }
    }

    fn empty_check2(&mut self) -> Result<(), ParseError> {
        match self.index + 1 >= self.size {
            true => Err(ParseError::OutOfScope),
            false => Ok(()),
        }
    }

    fn dec(&mut self) -> Result<(), ParseError> {
        if self.index > 0 {
            self.index -= 1;
            self.end -= 1;
            Ok(())
        } else {
            Err(ParseError::OutOfScope)
        }
    }

    fn next(&mut self) -> Result<Token, ParseError> {
        let first = self.peek()?;

        match first {
            b'$' => self.parse_variable(),
            b'%' => self.parse_binary(),
            b'0'..=b'9' => self.parse_absolute_decimal(),
            b'#' => self.parse_sharp(),
            b'a'..=b'z' | b'A'..=b'Z' => self.parse_keyword(),
            b'"' => self.parse_string(),
            b';' => self.parse_comment(),
            b'=' => self.parse_assign(),
            b'(' => self.parse_open_parenthesis(),
            b')' => self.parse_close_parenthesis(),
            b',' => self.parse_comma(),
            b'\r' | b'\n' => self.parse_newline(),
            b' ' | b'\t' => self.parse_whitespace(),
            n => {
                println!("{}", n);
                Err(ParseError::UnknownToken)
            }
        }
    }

    fn parse_variable(&mut self) -> Result<Token, ParseError> {
        self.eat_expected(b'$', ParseError::InvalidVariable)?;

        let start = self.index;
        let mut valid = false;
        
        loop {
            match self.peek() {
                Ok(byte) => {
                    match byte {
                        b'0'..=b'9' => (),
                        b'a'..=b'z' => valid = true,
                        b'A'..=b'Z' => valid = true,
                        b'_' => (),
                        b' ' | b',' | b')' | b'=' | b'\t' => break,
                        b'\n' | b'\r' => break,
                        _ => return Err(ParseError::InvalidKeyword),
                    };
                    self.eat()?;
                }
                Err(ParseError::OutOfScope) => break,
                _ => return Err(ParseError::InvalidKeyword),
            };
        }

        if !valid {
            return Err(ParseError::InvalidVariable);
        }

        Ok(Token::Variable(str::from_utf8(&self.data[start..self.index])?.to_string()))
    }

    fn parse_absolute_decimal(&mut self) -> Result<Token, ParseError> {
        
        let mut decimal_number: i64 = 0;
        
        while let Ok(n) = self.peek() {
            let number = match n {
                n @ b'0'..=b'9' => n - b'0',
                b' ' | b'\r' | b'\t' | b'\n' | b',' => break,
                _ => return Err(ParseError::InvalidNumberFormat),
            };

            decimal_number = (decimal_number * 10) + number as i64;
            let _ = self.eat();
        }

        Ok(Token::Number(decimal_number))
    }

    fn parse_hex(&mut self) -> Result<Token, ParseError> {
        self.eat_expected(b'$', ParseError::InvalidNumberFormat)?;
    
        let mut hex_number: i64 = 0;
        let mut count: u8 = 0;
        
        while let Ok(n) = self.peek() {
            let number = match n {
                b'0'..=b'9' => n - b'0',
                b'A'..=b'F' => (n - b'A') + 10,
                b'a'..=b'f' => (n - b'a') + 10,
                b' ' | b'\r' | b'\t' | b'\n' | b',' => break,
                _ => return Err(ParseError::InvalidNumberFormat),
            };

            hex_number = hex_number << 4 | number as i64;
            count += 1;
            let _ = self.eat();
        }
        
        if count != 2 && count != 4 {
            return Err(ParseError::InvalidNumberFormat);
        }

        Ok(Token::Number(hex_number))

    }

    fn parse_binary(&mut self) -> Result<Token, ParseError> {
        self.eat_expected(b'%', ParseError::InvalidNumberFormat)?;

        let mut binary_number: i64 = 0b0000_0000_0000_0000;
        let mut count: u8 = 0;
        
        while let Ok(n) = self.peek() {
            let number: i64 = match n {
                b'0' => 0,
                b'1' => 1,
                b' ' | b'\r' | b'\t' | b'\n' | b',' => break,
                _ => return Err(ParseError::InvalidNumberFormat),
            };

            binary_number = binary_number << 1 | number;
            count += 1;
            let _ = self.eat();
        }
        
        if count != 8 && count != 16 {
            return Err(ParseError::InvalidNumberFormat);
        }
        
        Ok(Token::Number(binary_number))
    }

    fn parse_open_parenthesis(&mut self) -> Result<Token, ParseError> {
        self.eat_expected(b'(', ParseError::InvalidNumberFormat)?;
        Ok(Token::OpenParenthesis)
    }

    fn parse_close_parenthesis(&mut self) -> Result<Token, ParseError> {
        self.eat_expected(b')', ParseError::InvalidNumberFormat)?;
        Ok(Token::CloseParenthesis)
    }

    fn parse_sharp(&mut self) -> Result<Token, ParseError> {
        self.eat_expected(b'#', ParseError::InvalidNumberFormat)?;
        Ok(Token::Sharp)
    }

    fn parse_keyword(&mut self) -> Result<Token, ParseError> {
        let start = self.index;

        let mut valid = false;
        let mut branch = false;

        loop {
            match self.peek() {
                Ok(byte) => {
                    match byte {
                        b'0'..=b'9' => (),
                        b'a'..=b'z' => valid = true,
                        b'A'..=b'Z' => valid = true,
                        b'_' => (),
                        b':' => {
                            branch = true;
                            self.eat()?;
                            break;
                        }
                        _ => break
                    };
                    self.eat()?;
                }
                Err(ParseError::OutOfScope) => break,
                _ => return Err(ParseError::InvalidKeyword),
            };
        }

        if !valid {
            return Err(ParseError::InvalidKeyword);
        }

        if branch {
            return Ok(Token::Branch(str::from_utf8(&self.data[start..self.index - 1])?.to_string()));
        }

        Ok(Token::Keyword(str::from_utf8(&self.data[start..self.index])?.to_string()))
    }

    fn parse_string(&mut self) -> Result<Token, ParseError> {
        self.eat_expected(b'"', ParseError::InvalidString)?;
        let start = self.index;

        loop {
            match self.peek() {
                Ok(byte) => {
                    match byte {
                        b'"' => break,
                        b'\\' => {
                            if self.peek2()? == b'"' { // It is inline \"
                                self.eat()?;
                            }
                        },
                        _ => ()
                    };
                    self.eat()?;
                }
                _ => return Err(ParseError::InvalidString),
            };
        }

        self.eat_expected(b'"', ParseError::InvalidString)?;
        Ok(Token::String(str::from_utf8(&self.data[start..self.index - 1])?.to_string()))
    }

    fn parse_comment(&mut self) -> Result<Token, ParseError> {
        let start = self.index;

        loop {
            match self.eat() {
                Ok(byte) => match byte {
                    b'\n' | b'\r' => {
                        self.dec()?;
                        break;
                    },
                    _ => continue,
                },
                Err(ParseError::OutOfScope) => break,
                _ => return Err(ParseError::InvalidCommentFormat),
            };
        }
        Ok(Token::Comment(str::from_utf8(&self.data[start..self.index - 1])?.to_string()))
    }

    fn parse_assign(&mut self) -> Result<Token, ParseError> {
        self.eat_expected(b'=', ParseError::UnexpectedSymbol)?;
        Ok(Token::Assign)
    }

    fn parse_comma(&mut self) -> Result<Token, ParseError> {
        self.eat_expected(b',', ParseError::UnexpectedSymbol)?;
        Ok(Token::Comma)
    }

    fn parse_newline(&mut self) -> Result<Token, ParseError> {
        let mut total_lines = 0;

        loop {
            match self.peek() {
                Ok(b'\r') => (),
                Ok(b'\n') => total_lines += 1,
                _ => break,
            };
            self.eat()?;
        }
        Ok(Token::NewLine(total_lines))
    }

    fn parse_whitespace(&mut self) -> Result<Token, ParseError> {
        let mut total_whitespaces = 0;

        while let Ok(b' ') | Ok(b'\t') = self.peek() {
            total_whitespaces += 1;
            self.eat()?;
        }

        Ok(Token::Space(total_whitespaces))
    }

    pub fn friendly_dump(&self) {
        let mut line = 0;

        info!("Tokens");
        print!("{:>5}. ", line);
        for ast in self.context.tokens.borrow().iter() {
            let type_name = match ast.token {
                Token::Keyword(_) => "KEYWORD",
                Token::Comment(_) => "COMMENT",
                Token::Branch(_) => "BRANCH",
                Token::Number(_) => "NUMBER",
                Token::Float(_) => "FLOAT",
                Token::OpenParenthesis => "(",
                Token::CloseParenthesis => ")",
                Token::Sharp => "#",
                Token::NewLine(_) => "NEWLINE",
                Token::Space(_) => "SPACE",
                Token::End => "END",
                Token::String(_) => "STRING",
                Token::Assign => "ASSIGN",
                Token::Comma => "COMMA",
                Token::Variable(_) => "VARIABLE",
            };

            if ast.line != line {
                println!();
                line = ast.line;
                print!("{:>5}. ", line);
            }

            print!("[{:>2}:{:<2} {:^10}] ", ast.column, ast.end, type_name);
        }
        println!();
    }
}
