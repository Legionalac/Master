#include <iostream>
#include <vector>
#include <numeric>
#include <algorithm>

using std::cout;
using std::vector;

struct GenerateNumbers {
    int current;
    GenerateNumbers() : current(0) {}
    int operator()() {
        return current++;
    }
};

struct Square {
    int operator()(int x) const {
        return x * x;
    }
};

struct SumOfSquares {
    int operator()(int sum, int x) const {
        return sum + x * x;
    }
};

struct PrintEven {
    void operator()(int x) const {
        if (x % 2 == 0) {
            cout << x << " ";
        }
    }
};

template<typename T>

void printElements(const T& c) {
    for (const auto& x : c) {
        cout << x << " ";
    }
    cout << std::endl;
}

int main()
{
    vector<int> first;

    // 1. Set vector elements to 0 .. 99.
    std::generate_n(std::back_inserter(first), 100, GenerateNumbers());

    cout << "First:\n";
    printElements(first);

    vector<int> second{ first };  // Copy vector first to vector second

    // 2. Modify values of vector "first" by squaring them.
    std::transform(first.begin(), first.end(), first.begin(), Square());

    cout << "\n\nSquares:\n";
    printElements(first);

    int res{
        // 3. Calculate the sum of squared elements of "second".
        std::accumulate(second.begin(), second.end(), 0, SumOfSquares())
    };

    cout << "\n\nSum of squares: " << res << "\n";
    if (res != 328350)
        return 1; // wrong result

    cout << "\nEven numbers:\n";
    // 4. Print out only even numbers in vector "second".
    std::for_each(second.begin(), second.end(), PrintEven());

    return 0;
}
