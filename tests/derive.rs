#![cfg(feature = "derive")]

use lerp::Lerp;
use std::fmt::Debug;
use common::round;

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
