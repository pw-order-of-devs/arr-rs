# arr-rs

[![crates.io](https://img.shields.io/crates/v/arr-rs?label=latest)](https://crates.io/crates/arr-rs)
![downloads](https://img.shields.io/crates/d/arr-rs.svg)
[![Documentation](https://docs.rs/arr-rs/badge.svg?version=latest)](https://docs.rs/arr-rs/latest)
![MIT or Apache 2.0 licensed](https://img.shields.io/crates/l/arr-rs.svg) \
[![CI](https://github.com/pw-order-of-devs/arr-rs/actions/workflows/default.yml/badge.svg)](https://github.com/pw-order-of-devs/arr-rs/actions/workflows/default.yml)
[![Dependency Status](https://deps.rs/crate/arr-rs/latest/status.svg)](https://deps.rs/crate/arr-rs/latest)

### Crate

Implementation of numpy-inspired multidimensional, generic arrays. \
Documentation of the crate is available [here](https://docs.rs/arr-rs)

### How to use

```toml
[dependencies]
arr-rs = "0.1.0"
```

### Examples

```rust
// import the crate
use arr_rs::prelude::*;

// create an array: (4 elements, 2 dimensions)
let arr = Array::<i32>::new(vec![1, 2, 3, 4], vec![2, 2]);
  
// create same array using macro:
let arr: Array::<i32> = array!([[1, 2], [3, 4]]);
  
// create random array with the same shape:
let arr = Array::<i32>::rand(vec![2, 2]);

// array supports display and pretty display
let arr: Array<f64> = array!([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
println!("{array}");
println!("{array:#}");

// perform some chained operations on array:
let res = arr
    .map(|item| item * 2)
    .filter(|item| item % 3 == 0)
    .ravel()
    .slice(0 .. 2);
```

### Benchmark

Benchmark results can be found [here](https://github.com/pw-order-of-devs/arr-rs/actions/workflows/benchmark.yml)

### License

This project is licensed under either of the following licenses, at your option:

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0])
- MIT license ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT])
