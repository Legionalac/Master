#ifndef _MY_BIG_INT_
#define _MY_BIG_INT_

#include <cstdint>
#include <array>
#include <cstring>
#include <ostream>
#include <string>

struct MyBigInt {

private:
    static constexpr int size{70};
    std::array<int_least8_t, size> digits{0};
public:
     explicit MyBigInt(unsigned long long number) {
        int i = 0;
        while (number) {
            digits[i++] = number % 10;
            number /= 10;
        }
    }

    explicit MyBigInt(const char* number) {
        int size = std::strlen(number) - 1;
        int i = 0;
        while (size >= 0) {
            digits[i] = number[size] - '0';
            i++;
            size--;
        }
    }
    MyBigInt operator+(const MyBigInt& other){
        MyBigInt result {int(0)} ;
        int carry = 0;
        for (int i = 0; i < size; i++) {
            int sum = digits[i] + other.digits[i] + carry;
            result.digits[i] = sum % 10;
            carry = sum / 10;
        }

        return result;
    }

    friend MyBigInt operator+(const int first , MyBigInt const& second){
        // Sum only first digit and then sum carry
        MyBigInt result {int(0)} ;
        int carry = 0;
        int sum = second.digits[0] + first;
        result.digits[0] = sum % 10;
        carry = sum / 10;
        int i = 1;

        while (carry != 0) {
            sum = second.digits[i] + carry;
            result.digits[i] = sum % 10;
            carry = sum / 10;
            i++;
        }
        return result;
    }

    friend MyBigInt operator+( MyBigInt const& first , const int second){
        // Sum only first digit and then sum carry
        MyBigInt result = first ;
        int carry = 0;
        int sum = first.digits[0] + second;
        result.digits[0] = sum % 10;
        carry = sum / 10;
        int i = 1;

        while (carry != 0) {
            sum = first.digits[i] + carry;
            result.digits[i] = sum % 10;
            carry = sum / 10;
            i++;
        }
        return result;
    }
    friend bool operator!=(MyBigInt const& first, MyBigInt const& second){
        for (int i = 0; i < size; i++) {
            if(first.digits[i] != second.digits[i]){
                return true;
            }
        }
        return false;
    }
    friend std::ostream& operator<<(std::ostream& os, MyBigInt const& data)
    {
        std::string output = "";
        for(int i = size - 1; i >= 0; i--)
            if(data.digits[i] != 0)
                output += std::to_string(data.digits[i]);
        return os << output;
    };


};

MyBigInt operator""_mbi(const char* x){
    MyBigInt result{x};
    return result;
}

#endif
