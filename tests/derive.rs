#![cfg(feature = "derive")]

use common::round;
use lerp::Lerp;
use std::fmt::Debug;

mod common;

#[test]
fn tuple() {
    #[derive(PartialEq, Debug, Lerp)]
    struct Data(f64, f64);

    assert_eq!(
        round(&Data(0.0, 1.0).lerp(Data(1.0, 0.0), 0.5)),
        round(&Data(0.5, 0.5))
    );
    assert_eq!(
        round(&Data(0.0, 1.0).lerp(Data(1.0, 0.0), 0.9)),
        round(&Data(0.9, 0.1))
    );
}

#[test]
fn tuple_combine_types() {
    #[derive(PartialEq, Debug, Lerp)]
    struct Data(f64, f32);

    assert_eq!(
        round(&Data(0.0, 1.0).lerp(Data(1.0, 0.0), 0.5)),
        round(&Data(0.5, 0.5))
    );
    assert_eq!(
        round(&Data(0.0, 1.0).lerp(Data(1.0, 0.0), 0.9)),
        round(&Data(0.9, 0.1))
    );
}

#[test]
fn named() {
    #[derive(PartialEq, Debug, Lerp)]
    struct Data {
        a: f32,
        b: f32,
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
fn named_combine_types() {
    #[derive(PartialEq, Debug, Lerp)]
    struct Data {
        a: f64,
        b: f32,
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
fn nested_combine_types() {
    #[derive(PartialEq, Debug, Lerp)]
    struct InternalData(f64, f32);

    #[derive(PartialEq, Debug, Lerp)]
    struct Data {
        a: InternalData,
        b: f32,
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

#[test]
fn named_skip() {
    #[derive(PartialEq, Debug, Lerp)]
    struct Data {
        #[lerp(skip)]
        a: String,
        #[lerp(ignore)]
        b: f32,
    }

    assert_eq!(
        round(&Data { a: "a".into(), b: 1.0 }.lerp(Data { a: "bb".into(), b: 0.0 }, 0.5)),
        round(&Data { a: "a".into(), b: 1.0 })
    );
    assert_eq!(
        round(&Data { a: "aa".into(), b: 1.0 }.lerp(Data { a: "b".into(), b: 0.0 }, 0.9)),
        round(&Data { a: "aa".into(), b: 1.0 })
    );
}

#[test]
fn nested_manual_impl() {
    #[derive(PartialEq, Debug)]
    struct InternalData(f32, f32);

    impl Lerp<f32> for InternalData {
        fn lerp(self, other: Self, t: f32) -> Self {
            Self(self.0.lerp(other.0, t), self.1.lerp(other.1, t))
        }
    }

    #[derive(PartialEq, Debug, Lerp)]
    struct Data {
        #[lerp(f32)]
        a: InternalData,
        b: f64,
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