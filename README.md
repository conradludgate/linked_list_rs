# A ππ₯π₯**blazingly**π₯π₯π fast π linked π list

Toπ₯beπblazinglyπfastπ₯thisπ₯crateπcontainsπ₯ubπforπextraπperfπ.

# Benchmarks

| List                           | Inserting 10000 elements | Walking through 200 elements
|--------------------------------|--------------------------|-----------------------------
| `std::collections::LinkedList` | 5h3min44s                | 8s
| C++ `std::list`                | 6h2min4s                 | 7s
| This list                      | 3ms (SIGILL)             | 2ms (SIGSEGV)


# Is this production ready?
Yes, although production might not exist anymore after deploying it.

# How fast is it really?
πππππππππππππππππππππππππ
