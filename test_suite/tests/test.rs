#![cfg_attr(any(feature = "const_unstable", const_unstable), feature(const_fn, const_vec_new))]
#![warn(unsafe_code)]
#![warn(rust_2018_idioms)]
#![warn(clippy::all)]
#![allow(dead_code)]
#![allow(clippy::let_and_return)]

mod features {
    use const_fn::const_fn;

    #[test]
    fn test_variables() {
        assert!(const_min("variables") == "variables");
        assert_eq!(const_let("variables"), "variables");
        assert_eq!(const_vec_new::<u8>(), Vec::new());
    }

    // min_const_fn (rust 1.31+)

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
        assert_eq!(const_let("min_const_fn"), "min_const_fn");
        assert_eq!(const_vec_new::<u8>(), Vec::new());
    }

    // const_let (rust 1.33+)

    #[const_fn(feature = "const_let")]
    const fn const_let<T>(x: T) -> T {
        let y = const_min(x);
        y
    }

    #[cfg(feature = "const_let")]
    const CONST_LET: &str = const_let("const_let");

    #[cfg(feature = "const_let")]
    #[test]
    fn test_const_let() {
        assert!(CONST_LET == "const_let");
        assert_eq!(const_vec_new::<u8>(), Vec::new());
    }

    // const_fn + const_vec_new (rust nightly)

    #[const_fn(feature = "const_unstable")]
    const fn const_vec_new<T>() -> Vec<T> {
        let vec = Vec::new();
        vec
    }

    #[cfg(feature = "const_unstable")]
    const CONST_UNSTABLE: Vec<u8> = const_vec_new();

    #[cfg(feature = "const_unstable")]
    #[test]
    fn test_const_unstable() {
        assert_eq!(CONST_UNSTABLE, Vec::new());
    }
}

mod build_script {
    use const_fn::const_fn;

    #[test]
    fn test_variables() {
        assert!(const_min("variables") == "variables");
        assert_eq!(const_let("variables"), "variables");
        assert_eq!(const_vec_new::<u8>(), Vec::new());
    }

    // min_const_fn (rust 1.31+)

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
        assert_eq!(const_let("min_const_fn"), "min_const_fn");
        assert_eq!(const_vec_new::<u8>(), Vec::new());
    }

    // const_let (rust 1.33+)

    #[const_fn(const_let)]
    fn const_let<T>(x: T) -> T {
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

    // const_fn + const_vec_new (rust nightly)

    #[const_fn(const_unstable)]
    fn const_vec_new<T>() -> Vec<T> {
        let vec = Vec::new();
        vec
    }

    #[cfg(const_unstable)]
    const CONST_UNSTABLE: Vec<u8> = const_vec_new();

    #[cfg(const_unstable)]
    #[test]
    fn test_const_unstable() {
        assert_eq!(CONST_UNSTABLE, Vec::new());
    }
}
