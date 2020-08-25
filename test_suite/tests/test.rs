#![cfg_attr(const_unstable, feature(const_fn))]
#![warn(rust_2018_idioms, single_use_lifetimes)]

pub mod version {
    use const_fn::const_fn;

    #[test]
    fn test_variables() {
        assert!(const_min("variables") == "variables");
        assert_eq!(const_let("variables"), "variables");
        assert_eq!(const_vec_new::<u8>(), Vec::new());
        assert_eq!(A::const_unstable(const_vec_new::<u8>()), A(Vec::new()));
    }

    // min_const_fn (rust 1.31+)

    #[const_fn("1.31")]
    const fn const_min<T>(x: T) -> T {
        x
    }

    const CONST_MIN: &str = const_min("min_const_fn");

    #[test]
    fn test_const_min() {
        assert!(CONST_MIN == "min_const_fn");
        assert_eq!(const_let("min_const_fn"), "min_const_fn");
        assert_eq!(const_vec_new::<u8>(), Vec::new());
    }

    // const_let (rust 1.33+)

    #[allow(clippy::let_and_return)]
    #[const_fn("1.33")]
    const fn const_let<T>(x: T) -> T {
        let y = const_min(x);
        y
    }

    #[rustversion::since(1.33)]
    const CONST_LET: &str = const_let("const_let");

    #[rustversion::since(1.33)]
    #[test]
    fn test_const_let() {
        assert!(CONST_LET == "const_let");
        assert_eq!(const_vec_new::<u8>(), Vec::new());
    }

    // const_vec_new (rust 1.39+)

    #[const_fn("1.39")]
    const fn const_vec_new<T>() -> Vec<T> {
        Vec::new()
    }

    #[rustversion::since(1.39)]
    const CONST_VEC_NEW: Vec<u8> = const_vec_new();

    #[rustversion::since(1.39)]
    #[test]
    fn test_const_vec_new() {
        assert_eq!(CONST_VEC_NEW, Vec::new());
    }

    // const_fn (rust nightly)

    #[derive(Debug, Eq, PartialEq)]
    pub struct A<T>(T);

    impl<T: IntoIterator> A<T> {
        #[const_fn(nightly)]
        const fn const_unstable(x: T) -> Self {
            A(x)
        }
    }

    #[rustversion::nightly]
    pub const CONST_UNSTABLE: A<Vec<u8>> = A::const_unstable(const_vec_new());
}

pub mod cfg {
    use const_fn::const_fn;

    #[test]
    fn test_variables() {
        assert!(const_min("variables") == "variables");
        assert_eq!(const_let("variables"), "variables");
        assert_eq!(const_vec_new::<u8>(), Vec::new());
        assert_eq!(A::const_unstable(const_vec_new::<u8>()), A(Vec::new()));
    }

    // min_const_fn (rust 1.31+)

    #[const_fn(cfg(has_min_const_fn))]
    const fn const_min<T>(x: T) -> T {
        x
    }

    pub const CONST_MIN: &str = const_min("min_const_fn");

    #[test]
    fn test_const_min() {
        assert!(CONST_MIN == "min_const_fn");
        assert_eq!(const_let("min_const_fn"), "min_const_fn");
        assert_eq!(const_vec_new::<u8>(), Vec::new());
    }

    // const_let (rust 1.33+)

    #[allow(clippy::let_and_return)]
    #[const_fn(cfg(has_const_let))]
    const fn const_let<T>(x: T) -> T {
        let y = const_min(x);
        y
    }

    #[rustversion::since(1.33)]
    const CONST_LET: &str = const_let("const_let");

    #[rustversion::since(1.33)]
    #[test]
    fn test_const_let() {
        assert!(CONST_LET == "const_let");
        assert_eq!(const_vec_new::<u8>(), Vec::new());
    }

    // const_vec_new (rust 1.39+)

    #[const_fn(cfg(has_const_vec_new))]
    const fn const_vec_new<T>() -> Vec<T> {
        Vec::new()
    }

    #[rustversion::since(1.39)]
    const CONST_VEC_NEW: Vec<u8> = const_vec_new();

    #[rustversion::since(1.39)]
    #[test]
    fn test_const_vec_new() {
        assert_eq!(CONST_VEC_NEW, Vec::new());
    }

    // const_fn (rust nightly)

    #[derive(Debug, Eq, PartialEq)]
    pub struct A<T>(T);

    impl<T: IntoIterator> A<T> {
        #[const_fn(cfg(const_unstable))]
        const fn const_unstable(x: T) -> Self {
            A(x)
        }
    }

    #[rustversion::nightly]
    pub const CONST_UNSTABLE: A<Vec<u8>> = A::const_unstable(const_vec_new());
}
