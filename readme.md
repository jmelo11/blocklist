# SmallObjectPool

A rust implementation of a block list, a data structure described in the book "Modern Computational Finance: AAD and Parallel Simulations" by Antoine Savine.
The repo contains the following implementations:

- `PtrBased`: A trait that exposed methods to interact with data structures as if they were in C++ (i.e. using pointers and iterators).
- `ArrayLike`: A simple implementation of a list, using pointers.
- `LinkedList`: Linked-list, using pointers.
- `SmallObjectPool`: AKA "BlockList", a list of fixed-size blocks, using pointers.

## Usage

```rust
use smallobjectpool::SmallObjectPool;

fn main() {
    let mut sop = SmallObjectPool::<u32, 4>::new();
    for i in 0..8 {
        sop.push(i);
    }
}
```

## Performance

In terms of performance, the `SmallObjectPool` achieves close performance to a `Vec`, but still the latter is faster. In this case, the benefit of the `SmallObjectPool` will come from a smaller footprint in memory, as it doesn't need to allocate memory for each element after the capacity is reached.

| Operation             | Avg Time (ns or µs)             | Outliers                                |
|-----------------------|-----------------------------|-----------------------------------------|
| vec push              | 932.30 ns | None                                    |
| array direct insert   | 326.35 ns | 15 outliers (3 high mild, 12 high severe) |
| sop push              | 18.534 µs | 17 outliers (9 low mild, 5 high mild, 3 high severe) |
| linked list push      | 220.50 µs | 6 outliers (3 high mild, 3 high severe)  |
| vec high volume push  | 16.383 µs | None                                    |

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.
