#![cfg_attr(const_unstable, feature(const_fn))]
#![warn(unsafe_code)]
#![warn(rust_2018_idioms)]
#![allow(dead_code)]

use const_fn::const_fn;

#[test]
fn test_variables() {
    assert!(const_min("variables") == "variables");
    assert_eq!(const_let("variables"), "variables");
    assert_eq!(const_vec_new::<u8>(), Vec::new());
}

// min_const_fn (rust 1.31+)

#[const_fn(min_const_fn)]
const fn const_min<T>(x: T) -> T {
    x
}

#[cfg(min_const_fn)]
const CONST_MIN: &str = const_min("min_const_fn");

#[cfg(min_const_fn)]
#[test]
fn test_const_min() {
    assert!(CONST_MIN == "min_const_fn");
    assert_eq!(const_let("min_const_fn"), "min_const_fn");
    assert_eq!(const_vec_new::<u8>(), Vec::new());
}

// const_let (rust 1.33+)

#[const_fn(const_let)]
const fn const_let<T>(x: T) -> T {
    let y = const_min(x);
    y
}

#[cfg(const_let)]
const CONST_LET: &str = const_let("const_let");

#[cfg(const_let)]
#[test]
fn test_const_let() {
    assert!(CONST_LET == "const_let");
    assert_eq!(const_vec_new::<u8>(), Vec::new());
}

// const_vec_new (rust 1.39+)

#[const_fn(const_vec_new)]
const fn const_vec_new<T>() -> Vec<T> {
    Vec::new()
}

#[cfg(const_vec_new)]
const CONST_VEC_NEW: Vec<u8> = const_vec_new();

// const_fn (rust nightly)

struct A<T> {
    x: T,
}

impl<T: IntoIterator> A<T> {
    #[const_fn(const_unstable)]
    const fn const_unstable(x: T) -> Self {
        Self { x }
    }
}

#[cfg(const_unstable)]
const CONST_UNSTABLE: A<Vec<u8>> = A::const_unstable(const_vec_new());
