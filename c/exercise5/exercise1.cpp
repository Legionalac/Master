#include <chrono>
#include <iostream>
#include <thread>
#include <utility>

void func1(int number){
    std::this_thread::sleep_for(std::chrono::seconds(3));
    std::cout<< number << std::endl;
}

void func2(int number, std::thread t){
    t.join();
    std::cout<< number << std::endl;
}

int main(){
    std::thread t1(func1, 1);
    std::thread t2(func2, 2 , std::move(t1));
    std::thread t3(func2, 3 , std::move(t2));
    t3.join();
}
