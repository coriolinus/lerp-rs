# Lerp

Linear interpolation and iteration, automatically implemented over most float-compatible types. [![Build Status](https://travis-ci.org/coriolinus/lerp-rs.svg?branch=master)](https://travis-ci.org/coriolinus/lerp-rs)

Just need to know what's halfway between three and five?

```rust
use lerp::Lerp;

assert_eq!(3.0.lerp(5.0, 0.5), 4.0);
```

Want to iterate across some points in that range?

```rust
// bring the trait into scope
use lerp::LerpIter;

// iterate and produce four items evenly spaced between 3.0 and 5.0
// note that the default, open iterator does not include both endpoints
// this makes chaining lerping iterators simpler
let items: Vec<_> = 3.0_f64.lerp_iter(5.0, 4).collect();
assert_eq!(vec![3.0, 3.5, 4.0, 4.5], items);

// closed iterators return both ends
assert_eq!(vec![3.0, 5.0], 3.0.lerp_iter_closed(5.0, 2).collect::<Vec<_>>());
```

Of course, the real benefit is that it's derivation is broad enough that it also covers types such as `num::Complex<T>`. If you have an array-processing library, and the arrays are `T: Copy + Add<Output = T> + Sub<Output = T> + Mul<F: Float, Output = T>`, it'll just work for them as well.

## Usage

```toml
[dependencies]
lerp = "0.1"
```

## Documentation

Auto-built from Travis: <https://coriolinus.github.io/lerp-rs/>
