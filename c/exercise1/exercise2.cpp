#include <iostream>

constexpr int fibo(int n) {
    if (n == 0) return 0;
    if (n == 1) return 1;

    int a = 0;
    int b = 1;
    int result = 0;

    for (int i = 2; i <= n; ++i) {
        result = a + b;
        a = b;
        b = result;
    }
    return result;
}

int main() {
    static_assert(fibo(7) == 13, "Fibonacci(7) should be 13");
    //Fixed this line so that it is not 34, so that the code runs

    constexpr int k = fibo(9);
    std::cout << k << std::endl;

    //constexpr int l = fibo(300); // 300. Fibonacci number is too big for int

    return 0;
}
