use lerp::{Lerp, num_traits::{Float, cast}};
use std::fmt::Debug;
use common::round;

mod common;

#[test]
fn manual() {
    #[derive(PartialEq, Debug)]
    struct Data {
        a: f64,
        b: f64,
    };

    impl<F: Float> Lerp<F> for Data {
        fn lerp(self, other: Self, t: F) -> Self {
            Self {
                a: self
                    .a
                    .lerp(other.a, cast::<_, f64>(t).unwrap()),
                b: self
                    .b
                    .lerp(other.b, cast::<_, f64>(t).unwrap()),
            }
        }
    }

    assert_eq!(
        round(&Data { a: 0.0, b: 1.0 }.lerp(Data { a: 1.0, b: 0.0 }, 0.5)),
        round(&Data { a: 0.5, b: 0.5 })
    );
    assert_eq!(
        round(&Data { a: 0.0, b: 1.0 }.lerp(Data { a: 1.0, b: 0.0 }, 0.9)),
        round(&Data { a: 0.9, b: 0.1 })
    );
}

#[test]
fn manual_mix() {
    #[derive(PartialEq, Debug)]
    struct Data {
        a: f64,
        b: f32,
    };

    impl<F: crate::Float> Lerp<F> for Data {
        fn lerp(self, other: Self, t: F) -> Self {
            Self {
                a: self
                    .a
                    .lerp(other.a, cast::<_, f64>(t).unwrap()),
                b: self
                    .b
                    .lerp(other.b, cast::<_, f32>(t).unwrap()),
            }
        }
    }

    assert_eq!(
        round(&Data { a: 0.0, b: 1.0 }.lerp(Data { a: 1.0, b: 0.0 }, 0.5)),
        round(&Data { a: 0.5, b: 0.5 })
    );
    assert_eq!(
        round(&Data { a: 0.0, b: 1.0 }.lerp(Data { a: 1.0, b: 0.0 }, 0.9)),
        round(&Data { a: 0.9, b: 0.1 })
    );
}

#[test]
fn manual_nested() {
    #[derive(PartialEq, Debug)]
    struct InternalData(f64, f32);

    impl<F: crate::Float> Lerp<F> for InternalData {
        fn lerp(self, other: Self, t: F) -> Self {
            Self(
                self.0
                    .lerp(other.0, cast::<_, f64>(t).unwrap()),
                self.1
                    .lerp(other.1, cast::<_, f32>(t).unwrap()),
            )
        }
    }

    #[derive(PartialEq, Debug)]
    struct Data {
        a: InternalData,
        b: f32,
    };

    impl<F: crate::Float> Lerp<F> for Data {
        fn lerp(self, other: Self, t: F) -> Self {
            Self {
                a: self.a.lerp(other.a, t),
                b: self
                    .b
                    .lerp(other.b, cast::<_, f32>(t).unwrap()),
            }
        }
    }

    assert_eq!(
        round(
            &Data {
                a: InternalData(0.0, 1.0),
                b: 1.0
            }
            .lerp(
                Data {
                    a: InternalData(1.0, 0.0),
                    b: 0.0
                },
                0.5
            )
        ),
        round(&Data {
            a: InternalData(0.5, 0.5),
            b: 0.5
        })
    );
    assert_eq!(
        round(
            &Data {
                a: InternalData(0.0, 1.0),
                b: 1.0
            }
            .lerp(
                Data {
                    a: InternalData(1.0, 0.0),
                    b: 0.0
                },
                0.9
            )
        ),
        round(&Data {
            a: InternalData(0.9, 0.1),
            b: 0.1
        })
    );
}
