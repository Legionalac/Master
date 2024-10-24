#include <iostream>
#include <vector>
#include <numeric>
#include <algorithm>

using std::cout;
using std::vector;

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
	std::generate_n(std::back_inserter(first), 100, [n = 0]() mutable { return n++; });

	cout << "First:\n";
	printElements(first);

	vector<int> second{ first };  // Copy vector first to vector second

	// 2. Modify values of vector "first" by squaring them.
	std::transform(first.begin(), first.end(), first.begin(), [](int x) { return x * x; });

	cout << "\n\nSquares:\n";
	printElements(first);

	int res{
		// 3. Calculate the sum of squared elements of "second".
		std::accumulate(second.begin(), second.end(), 0, [](int sum, int x) { return sum + x * x; })
	};

	cout << "\n\nSum of squares: " << res << "\n";
	if (res != 328350)
		return 1; // wrong result

	cout << "\nEven numbers:\n";
	// 4. Print out only even numbers in vector "second".
	std::for_each(second.begin(), second.end(), [](int x) { if (x % 2 == 0) cout << x << " "; });

	return 0;
}
