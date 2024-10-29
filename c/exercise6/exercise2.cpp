#include <iostream>

template<typename T>
T min(T x) {
    return x;
}
template<typename T, typename... Args>
T min(T x, Args... args) {
    T min_r = min(args...);
    return (x < min_r) ? x : min_r;
}

int main(){
    int a = min(1, 2, 3, 5, 7, 8, -2);
    std::cout<<a<<std::endl;
}
