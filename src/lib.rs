//! Linear interpolation and extrapolation traits.
#![doc(html_root_url = "https://coriolinus.github.io/lerp-rs/")]

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use num_traits::{Float, One, Zero};
use std::iter;
use std::iter::{Chain, Once, Skip};
use std::ops::{Add, Mul};

pub use num_traits;

/// Types which are amenable to linear interpolation and extrapolation.
///
/// This is mainly intended to be useful for complex
/// numbers, vectors, and other types which may be multiplied by a
/// scalar while retaining their own type.
///
/// It's automatically implemented
/// for all `T: Add<Output = T> + Mul<F, Output = T>`.
pub trait Lerp<F> {
    /// Interpolate and extrapolate between `self` and `other` using `t` as the parameter.
    ///
    /// At `t == 0.0`, the result is equal to `self`.
    /// At `t == 1.0`, the result is equal to `other`.
    /// At all other points, the result is a mix of `self` and `other`, proportional to `t`.
    ///
    /// `t` is unbounded, so extrapolation and negative interpolation are no problem.
    ///
    /// # Examples
    ///
    /// Basic lerping on floating points:
    ///
    /// ```
    /// use lerp::Lerp;
    ///
    /// let four_32 = 3.0_f32.lerp(5.0, 0.5);
    /// assert_eq!(four_32, 4.0);
    /// let four_64 = 3.0_f64.lerp(5.0, 0.5);
    /// assert_eq!(four_64, 4.0);
    /// ```
    ///
    /// Extrapolation:
    ///
    /// ```
    /// # use lerp::Lerp;
    /// assert_eq!(3.0.lerp(4.0, 2.0), 5.0);
    /// ```
    ///
    /// Negative extrapolation:
    ///
    /// ```
    /// # use lerp::Lerp;
    /// assert_eq!(3.0.lerp(4.0, -1.0), 2.0);
    /// ```
    ///
    /// Reverse interpolation:
    ///
    /// ```
    /// # use lerp::Lerp;
    /// assert_eq!(5.0.lerp(3.0, 0.5), 4.0);
    /// ```
    fn lerp(self, other: Self, t: F) -> Self;

    /// Interpolate between `self` and `other` precisely per the `lerp` function, bounding `t`
    /// in the inclusive range [0..1].
    ///
    /// # Examples
    ///
    /// Bounding on numbers greater than one:
    ///
    /// ```
    /// # use lerp::Lerp;
    /// assert_eq!(3.0.lerp_bounded(4.0, 2.0), 4.0);
    /// ```
    ///
    /// Bounding on numbers less than zero:
    ///
    /// ```
    /// # use lerp::Lerp;
    /// assert_eq!(3.0.lerp_bounded(5.0, -2.0), 3.0);
    /// ```
    fn lerp_bounded(self, other: Self, t: F) -> Self
    where
        Self: Sized,
        F: PartialOrd + Copy + Zero + One,
    {
        let t = match t {
            t if t < F::zero() => F::zero(),
            t if t > F::one() => F::one(),
            t => t,
        };
        self.lerp(other, t)
    }
}

/// Types which can construct a lerping iterator from one point to another
/// over a set number of steps.
///
/// This is automatically implemented for all `T: Lerp<f64> + Sized`.
pub trait LerpIter {
    /// Create an iterator which lerps from `self` to `other`.
    ///
    /// The iterator is half-open: it includes `self`, but not `other`
    ///
    /// # Example
    ///
    /// ```
    /// use lerp::LerpIter;
    ///
    /// // lerp between 3 and 5, collecting two items
    /// let items: Vec<_> = 3.0_f64.lerp_iter(5.0, 4).collect();
    /// assert_eq!(vec![3.0, 3.5, 4.0, 4.5], items);
    /// ```
    fn lerp_iter(self, other: Self, steps: usize) -> LerpIterator<Self>
    where
        Self: Sized;

    /// Create an iterator which lerps from `self` to `other`.
    ///
    /// The iterator is closed: it returns both `self` and `other`.
    ///
    /// Note when `steps == 1`, `other` is returned instead of `self`.
    ///
    /// # Example
    ///
    /// ```
    /// use lerp::LerpIter;
    ///
    /// assert_eq!(vec![3.0, 5.0], 3.0_f64.lerp_iter_closed(5.0, 2).collect::<Vec<f64>>());
    /// ```
    fn lerp_iter_closed(
        self,
        other: Self,
        steps: usize,
    ) -> Skip<Chain<LerpIterator<Self>, Once<Self>>>
    where
        Self: Copy,
        LerpIterator<Self>: Iterator<Item = Self>,
    {
        // reduce the number of times we consume the sub-iterator,
        // because we unconditionally add an element to the end.
        if steps == 0 {
            LerpIterator::new(self, other, steps)
                .chain(iter::once(other))
                .skip(1)
        } else {
            LerpIterator::new(self, other, steps - 1)
                .chain(iter::once(other))
                .skip(0)
        }
    }
}

/// Default, generic implementation of Lerp.
///
/// Note that due to the implementation details, LerpIterator is only actually
/// an iterator for those types `T` which fit the constraint `Mul<f64, Output = T>`.
/// This means that though you can use the `lerp` method on f32s, it will not work to
/// iterate over the results of calling `lerp_iter` on an f32. Instead, up-cast
/// your f32 as an f64 before calling: `(example_f32 as f64).lerp_iter(...)`.
///
/// This default implementation is mainly intended to be useful for complex
/// numbers, vectors, and other types which may be multiplied by a
/// scalar while retaining their own type.
impl<T, F> Lerp<F> for T
where
    T: Add<Output = T> + Mul<F, Output = T>,
    F: Float,
{
    fn lerp(self, other: T, t: F) -> T {
        self * (F::one() - t) + other * t
    }
}

impl<T> LerpIter for T
where
    T: Lerp<f64> + Sized,
{
    fn lerp_iter(self, other: T, steps: usize) -> LerpIterator<T> {
        LerpIterator::new(self, other, steps)
    }
}

/// An iterator across a range defined by its endpoints and the number of intermediate steps.
pub struct LerpIterator<T> {
    begin: T,
    end: T,
    steps: usize,
    current_step: usize,
}

impl<T> LerpIterator<T> {
    fn new(begin: T, end: T, steps: usize) -> LerpIterator<T> {
        LerpIterator {
            begin,
            end,
            steps,
            current_step: 0,
        }
    }
}

impl<T> Iterator for LerpIterator<T>
where
    T: Lerp<f64> + Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.current_step >= self.steps {
            None
        } else {
            let t = self.current_step as f64 / self.steps as f64;
            self.current_step += 1;
            Some(self.begin.lerp(self.end, t))
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = if self.current_step >= self.steps {
            0
        } else {
            self.steps - self.current_step
        };
        (remaining, Some(remaining))
    }
}

impl<T> ExactSizeIterator for LerpIterator<T> where T: Lerp<f64> + Copy {}

#[cfg(feature = "derive")]
#[allow(unused_imports)]
#[macro_use]
extern crate lerp_derive;

#[cfg(feature = "derive")]
#[doc(hidden)]
pub use lerp_derive::*;

