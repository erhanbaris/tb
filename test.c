
int main() {
    int a1 = 5;
    int b1 = 10;
    int test1 = a1+b1;

    int a2 = 5;
    int b2 = 10;
    int test2 = a2+b2;
    int actual = test1+test2;
    return actual;
}

// cargo run > test.s; gcc -m64 test.s -o test; ./test; echo $?
// gcc -S -O0 -fno-asynchronous-unwind-tables -fcf-protection=none test.c -o test_.s
