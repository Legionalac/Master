#include <iostream>

template<typename T>
T sum(T x) {
    return x;
}
template<typename T, typename... Args>
T sum(T x, Args... args) {
    return x + sum(args...);
}

int main(){
    int a = sum(1, 2, 3, 5, 7, 8);
    std::cout<<a<<std::endl;
}
