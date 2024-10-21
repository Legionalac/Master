#ifndef _MY_UNIQUE_PTR_
#define _MY_UNIQUE_PTR_

#include <iostream>
#include <memory>

template<class T>
class MyUniquePtr{
private:
    std::unique_ptr<T> p;
public:
    MyUniquePtr<T>() {}
    MyUniquePtr<T>(std::unique_ptr<T> other){
        p = other;
        other.reset();
    }

    MyUniquePtr<T>(T* other) : p(other) {}

    // These are already implicilty deleted, is there other way to stop copy?
    MyUniquePtr(MyUniquePtr& other) = delete;
    MyUniquePtr& operator=(MyUniquePtr& other) = delete;
    // --------------------------------------------------------

    MyUniquePtr(MyUniquePtr&& other) : p(std::move(other.p)) {}

    MyUniquePtr& operator=(MyUniquePtr&& other){
        p = std::move(other.p);
        return *this;
    }

    T operator*(){
        return *p;
    }

    T* get(){
        return p.get();
    }

    T* operator->(){
        return p.get();
    }

    ~MyUniquePtr<T>() {
        p.release();
        std::cout<< "Hi!";
    }
};

#endif
