# A 🚀🔥🔥**blazingly**🔥🔥🚀 fast 🚀 linked 🚀 list

To🔥be🚀blazingly🚀fast🔥this🔥crate🚀contains🔥ub🚀for🚀extra🚀perf🚀.

# Benchmarks

| List                           | Inserting 10000 elements | Walking through 200 elements
|--------------------------------|--------------------------|-----------------------------
| `std::collections::LinkedList` | 5h3min44s                | 8s
| C++ `std::list`                | 6h2min4s                 | 7s
| This list                      | 3ms (SIGILL)             | 1ms (SIGSEGV)


# Is this production ready?
Yes, although production might not exist anymore after deploying it.

# How fast is it really?
🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀

# MSRV

The current MSRV is 1.10.0 but that may be reduced in the future. Reducing the MSRV is not considered a breaking change and may happen in patch releases.
