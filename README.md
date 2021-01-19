# Lerp

[![Build Status](https://travis-ci.org/coriolinus/lerp-rs.svg?branch=master)](https://travis-ci.org/coriolinus/lerp-rs)

Linear interpolation and iteration, automatically implemented over most
float-compatible types.

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

Of course, the real benefit is that it's derivation is broad enough that it also
covers types such as `num::Complex<T>`. If you have an array-processing library,
and the arrays are `T: Add<Output = T> + Mul<F: Float, Output = T>`, it'll just
work for them as well.

## Deriving `Lerp`

As well as working for individual float values, the crate also provides a derive
macro, available with the `derive` feature, which will be able to generate an
implementation automatically.

This derive implementation will lerp each field of the struct independently
and assumes a generic implementation of Lerp over `Float` types. If any
of the fields is generic only over one of the float values (f32, f64) that
can be specified by the `#[lerp(f32)]` or `#[lerp(f64)]` attributes respectively.

If you would like for the lerp implementation to ignore a field (or if it does
not derive lerp) you can use the `#[lerp(skip)]` attribute which will produce
the value, untouched from the left value.

Not all types are supported in this derive macro. See [the github issue] for
discussion and more information.

```toml
[dependencies]
lerp = { version = "0.4", features = ["derive"] }
```

```rust
use lerp::Lerp;

#[derive(Lerp, PartialEq, Debug)]
struct Data {
    a: f64,
    b: f64
}

assert_eq!(
    Data { a: 0.0, b: 1.0 }.lerp(Data { a: 1.0, b: 0.0 }, 0.5),
    Data { a: 0.5, b: 0.5 }
);
```

More derive examples can be seen in the [tests]

## Usage

```toml
[dependencies]
lerp = "0.4"
```

## Documentation

Auto-built from Travis: <https://coriolinus.github.io/lerp-rs/>

[the github issue]: https://github.com/coriolinus/lerp-rs/issues/6
[tests]: https://github.com/coriolinus/lerp-rs/tree/master/tests/derive.rs
