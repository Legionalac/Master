#ifndef _MY_BIG_INT_
#define _MY_BIG_INT_

#include <cstdint>
#include <array>
#include <ostream>

struct MyBigInt {

private:
	static constexpr int size{70};
	std::array<int_least8_t, size> digits;
public:
    MyBigInt(int number) {
        int i = 0;
        while (number) {
            digits[i++] = number % 10;
            number /= 10;
        }
    }
    friend std::ostream& operator<<(std::ostream& os, MyBigInt const& data)
    {
        return os << "I'm in the class, msg=" << data.size << std::endl;
    };
};

#endif
