#![cfg_attr(
    any(feature = "const_unstable", const_unstable),
    feature(const_fn, const_let, const_vec_new)
)]
#![deny(warnings)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![cfg(test)]

#[macro_use]
extern crate const_fn;

mod features {
    #[test]
    fn test_variables() {
        assert!(const_min("variables") == "variables");
        assert_eq!(const_unstable("variables"), "variables");
        assert_eq!(const_vec_new::<u8>(), Vec::new());
    }

    #[const_fn(feature = "min_const_fn")]
    const fn const_min<T>(x: T) -> T {
        x
    }

    #[cfg(feature = "min_const_fn")]
    const CONST_MIN: &str = const_min("min_const_fn");

    #[cfg(feature = "min_const_fn")]
    #[test]
    fn test_const_min() {
        assert!(CONST_MIN == "min_const_fn");
        assert_eq!(const_unstable("min_const_fn"), "min_const_fn");
        assert_eq!(const_vec_new::<u8>(), Vec::new());
    }

    #[const_fn(feature = "const_unstable")]
    const fn const_unstable<T>(x: T) -> T {
        let y = const_min(x);
        y
    }

    #[cfg(feature = "const_unstable")]
    const CONST_UNSTABLE: &str = const_unstable("const_unstable");

    #[const_fn(feature = "const_unstable")]
    const fn const_vec_new<T>() -> Vec<T> {
        let vec = Vec::new();
        vec
    }

    #[cfg(feature = "const_unstable")]
    const CONST_UNSTABLE2: Vec<u8> = const_vec_new();

    #[cfg(feature = "const_unstable")]
    #[test]
    fn test_const_unstable() {
        assert_eq!(CONST_UNSTABLE, "const_unstable");
        assert_eq!(CONST_UNSTABLE2, Vec::new());
    }
}

mod build_script {
    #[test]
    fn test_variables() {
        assert!(const_min("variables") == "variables");
        assert_eq!(const_unstable("variables"), "variables");
        assert_eq!(const_vec_new::<u8>(), Vec::new());
    }

    #[const_fn(min_const_fn)]
    fn const_min<T>(x: T) -> T {
        x
    }

    #[cfg(min_const_fn)]
    const CONST_MIN: &str = const_min("min_const_fn");

    #[cfg(min_const_fn)]
    #[test]
    fn test_const_min() {
        assert!(CONST_MIN == "min_const_fn");
        assert_eq!(const_unstable("min_const_fn"), "min_const_fn");
        assert_eq!(const_vec_new::<u8>(), Vec::new());
    }

    #[const_fn(const_unstable)]
    fn const_unstable<T>(x: T) -> T {
        let y = const_min(x);
        y
    }

    #[cfg(const_unstable)]
    const CONST_UNSTABLE: &str = const_unstable("const_unstable");

    #[const_fn(const_unstable)]
    fn const_vec_new<T>() -> Vec<T> {
        let vec = Vec::new();
        vec
    }

    #[cfg(const_unstable)]
    const CONST_UNSTABLE2: Vec<u8> = const_vec_new();

    #[cfg(const_unstable)]
    #[test]
    fn test_const_unstable() {
        assert_eq!(CONST_UNSTABLE, "const_unstable");
        assert_eq!(CONST_UNSTABLE2, Vec::new());
    }
}
