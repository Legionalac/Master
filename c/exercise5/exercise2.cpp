#include <iostream>
#include <thread>
#include <mutex>
#include <chrono>

int var1 = 0, var2 = 0;  // Shared resources
std::mutex mtx1, mtx2;   // Separate mutexes for each variable

void thread1() {
    int values[][2] = {{1, 2}, {3, 4}, {5, 6}};
    int i = 0;

    while (true) {
        {
            std::lock_guard<std::mutex> lock1(mtx1);
            var1 = values[i][0];
        }
        {
            std::lock_guard<std::mutex> lock2(mtx2);
            var2 = values[i][1];
        }

        i = (i + 1) % 3;
        std::this_thread::sleep_for(std::chrono::milliseconds(100));  
    }
}

void thread2() {
    int last_sum = -1;  

    while (true) {
        int local_var1, local_var2;
        
        while (true) {
            {
                std::lock_guard<std::mutex> lock1(mtx1);
                local_var1 = var1;
            }
            {
                std::lock_guard<std::mutex> lock2(mtx2);
                local_var2 = var2;
            }

            int sum = local_var1 + local_var2;
            if ((sum == 3 || sum == 7 || sum == 11) && sum != last_sum) {
                std::cout << "Sum: " << sum << std::endl;
                last_sum = sum;  // Update last_sum to avoid duplicate prints
                break;
            }

            std::this_thread::sleep_for(std::chrono::milliseconds(10));
        }

        std::this_thread::sleep_for(std::chrono::milliseconds(100));
    }
}

int main() {
    std::thread t1(thread1);
    std::thread t2(thread2);

    t1.join();
    t2.join();

    return 0;
}
