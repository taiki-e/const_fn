#![cfg_attr(const_unstable, feature(const_fn))]
#![warn(rust_2018_idioms, single_use_lifetimes)]

pub mod version {
    use const_fn::const_fn;

    // min_const_fn (1.31+)

    #[const_fn("1.31")]
    const fn const_min<T>(x: T) -> T {
        x
    }
    const _CONST_MIN: &str = const_min("min_const_fn");

    // const_let (1.33+)

    #[allow(clippy::let_and_return)]
    #[const_fn("1.33")]
    const fn const_let<T>(x: T) -> T {
        let y = const_min(x);
        y
    }
    #[rustversion::since(1.33)]
    const _CONST_LET: &str = const_let("const_let");

    // const_vec_new (1.39+)

    #[const_fn("1.39")]
    const fn const_vec_new<T>() -> Vec<T> {
        Vec::new()
    }
    #[rustversion::since(1.39)]
    const _: Vec<u8> = const_vec_new();

    // const_match, const_loop (1.46+)

    #[const_fn("1.46")]
    const fn const_match(x: u8) -> Option<u8> {
        match x {
            0 => None,
            x => Some(x),
        }
    }
    #[rustversion::since(1.46)]
    const _: Option<u8> = const_match(1);

    // const_fn (nightly)

    #[derive(Debug, Eq, PartialEq)]
    pub struct A<T>(T);

    impl<T: IntoIterator> A<T> {
        #[const_fn(nightly)]
        const fn const_unstable(x: T) -> Self {
            A(x)
        }
    }
    #[rustversion::nightly]
    const _: A<Vec<u8>> = A::const_unstable(const_vec_new());

    #[test]
    fn test() {
        assert!(const_min("variables") == "variables");
        assert_eq!(const_let("variables"), "variables");
        assert_eq!(const_vec_new::<u8>(), Vec::new());
        assert_eq!(const_match(1), Some(1));
        assert_eq!(A::const_unstable(const_vec_new::<u8>()), A(Vec::new()));
    }
}

pub mod cfg {
    use const_fn::const_fn;

    // min_const_fn (1.31+)

    #[const_fn(cfg(rustc_1_31))]
    const fn const_min<T>(x: T) -> T {
        x
    }
    const _CONST_MIN: &str = const_min("min_const_fn");

    // const_let (1.33+)

    #[allow(clippy::let_and_return)]
    #[const_fn(cfg(rustc_1_33))]
    const fn const_let<T>(x: T) -> T {
        let y = const_min(x);
        y
    }
    #[rustversion::since(1.33)]
    const _CONST_LET: &str = const_let("const_let");

    // const_vec_new (1.39+)

    #[const_fn(cfg(rustc_1_39))]
    const fn const_vec_new<T>() -> Vec<T> {
        Vec::new()
    }
    #[rustversion::since(1.39)]
    const _: Vec<u8> = const_vec_new();

    // const_match, const_loop (1.46+)

    #[const_fn(cfg(rustc_1_46))]
    const fn const_match(x: u8) -> Option<u8> {
        match x {
            0 => None,
            x => Some(x),
        }
    }
    #[rustversion::since(1.46)]
    const _: Option<u8> = const_match(1);

    // const_fn (nightly)

    #[derive(Debug, Eq, PartialEq)]
    pub struct A<T>(T);

    impl<T: IntoIterator> A<T> {
        #[const_fn(cfg(const_unstable))]
        const fn const_unstable(x: T) -> Self {
            A(x)
        }
    }
    #[rustversion::nightly]
    const _: A<Vec<u8>> = A::const_unstable(const_vec_new());

    #[test]
    fn test() {
        assert!(const_min("variables") == "variables");
        assert_eq!(const_let("variables"), "variables");
        assert_eq!(const_vec_new::<u8>(), Vec::new());
        assert_eq!(const_match(1), Some(1));
        assert_eq!(A::const_unstable(const_vec_new::<u8>()), A(Vec::new()));
    }
}
