#include "MyBigInt.hpp"

#include <iostream>

// Here write the following function:
// MyBigInt fibo(int n);


MyBigInt fibo(int n){


}


int main()
{
	MyBigInt k{ 7 };
	k = fibo(9);
	k = k + 5;
	std::cout << k << std::endl;
	k = 7 + fibo(11);
	std::cout << k << std::endl;
	k = 13_mbi;
	std::cout << k << std::endl;
	MyBigInt l = fibo(300);
	if (l != 222232244629420445529739893461909967206666939096499764990979600_mbi) {
		std::cout << "Error!" << std::endl;
		return 1;
	}
	std::cout << l << std::endl;
	return 0;
}
