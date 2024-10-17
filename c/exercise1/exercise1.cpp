#include <iostream>

struct Node {
    Node() {}
    Node(int e, Node* n) : elem(e), next(n) {}
    int elem = 0;
    Node* next = nullptr;

    struct Iterator {
        Node* current;
        Iterator(Node* node) : current(node) {}

        int& operator*() {
            return current->elem;
        }

        Iterator& operator++() {
            current = current->next;
            return *this;
        }

        bool operator!=(const Iterator& other) const {
            return current != other.current;
        }
    };

    Iterator begin() { return Iterator(next); }
    Iterator end() { return Iterator(nullptr); }
};

void add(Node& n, int x) {
    n.next = new Node(x, n.next);
}

int main() {
    Node head;
    add(head, 1);
    add(head, 2);
    add(head, 3);
    add(head, 4);
    add(head, 5);

    for (Node* p = head.next; p != nullptr; p = p->next) {
        std::cout << p->elem << " ";
    }
    std::cout << std::endl;

    for (int x : head) {
        std::cout << x << " ";
    }
    std::cout << std::endl;

    for (int& x : head) {
        x = 7;
    }

    for (const int& x : head) {
        std::cout << x << " ";
    }
    std::cout << std::endl;

    return 0;
}
