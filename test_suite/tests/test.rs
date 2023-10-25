// SPDX-License-Identifier: Apache-2.0 OR MIT

#![cfg_attr(const_unstable, feature(const_extern_fn))]
#![warn(rust_2018_idioms, single_use_lifetimes)]
#![allow(clippy::missing_safety_doc, clippy::unused_async, improper_ctypes_definitions)] // this is test

pub mod signature {
    #![allow(dead_code)]

    use const_fn::const_fn;

    // const
    #[const_fn]
    fn const_non_const() {}
    #[const_fn]
    pub fn const_non_const_pub() {}
    #[const_fn]
    const fn const_const() {}
    #[const_fn]
    pub const fn const_const_pub() {}
    const _: () = const_non_const();
    const _: () = const_non_const_pub();
    const _: () = const_const();
    const _: () = const_const_pub();

    // const unsafe
    #[const_fn]
    unsafe fn const_unsafe_non_const() {}
    #[const_fn]
    pub unsafe fn const_unsafe_non_const_pub() {}
    #[const_fn]
    const unsafe fn const_unsafe_const() {}
    #[const_fn]
    pub const unsafe fn const_unsafe_const_pub() {}
    const _: () = unsafe { const_unsafe_non_const() };
    const _: () = unsafe { const_unsafe_non_const_pub() };
    const _: () = unsafe { const_unsafe_const() };
    const _: () = unsafe { const_unsafe_const_pub() };

    // const extern
    #[const_fn(cfg(const_unstable))]
    extern "C" fn const_extern_non_const() {}
    #[const_fn(cfg(const_unstable))]
    pub extern "C" fn const_extern_non_const_pub() {}
    #[const_fn(cfg(const_unstable))]
    const extern "C" fn const_extern_const() {}
    #[const_fn(cfg(const_unstable))]
    pub const extern "C" fn const_extern_const_pub() {}
    #[cfg(const_unstable)]
    const _: () = const_extern_non_const();
    #[cfg(const_unstable)]
    const _: () = const_extern_non_const_pub();
    #[cfg(const_unstable)]
    const _: () = const_extern_const();
    #[cfg(const_unstable)]
    const _: () = const_extern_const_pub();

    // const unsafe extern
    #[const_fn(cfg(const_unstable))]
    unsafe extern "C" fn const_unsafe_extern_non_const() {}
    #[const_fn(cfg(const_unstable))]
    pub unsafe extern "C" fn const_unsafe_extern_non_const_pub() {}
    #[const_fn(cfg(const_unstable))]
    const unsafe extern "C" fn const_unsafe_extern_const() {}
    #[const_fn(cfg(const_unstable))]
    pub const unsafe extern "C" fn const_unsafe_extern_const_pub() {}
    #[cfg(const_unstable)]
    const _: () = unsafe { const_unsafe_extern_non_const() };
    #[cfg(const_unstable)]
    const _: () = unsafe { const_unsafe_extern_non_const_pub() };
    #[cfg(const_unstable)]
    const _: () = unsafe { const_unsafe_extern_const() };
    #[cfg(const_unstable)]
    const _: () = unsafe { const_unsafe_extern_const_pub() };

    // const async unsafe extern
    // functions cannot be both `const` and `async`, but rustc syntactically accepts this.
    #[const_fn(cfg(any()))]
    async unsafe extern "C" fn const_async_unsafe_extern_non_const() {}
    #[const_fn(cfg(any()))]
    pub async unsafe extern "C" fn const_async_unsafe_extern_non_const_pub() {}
    #[const_fn(cfg(any()))]
    const async unsafe extern "C" fn const_async_unsafe_extern_const() {}
    #[const_fn(cfg(any()))]
    pub const async unsafe extern "C" fn const_async_unsafe_extern_const_pub() {}
}

pub mod min_const_generics {
    #![allow(dead_code, unused_braces)]

    use const_fn::const_fn;

    struct S1<const C1: usize, const C2: usize>([(); C1], [(); C2]);
    trait T1 {
        type A1;
    }

    #[const_fn]
    const fn const_generics1() -> [(); { 1 + 1 }] {
        [(); 2]
    }
    #[const_fn]
    const fn const_generics2() -> S1<1, { 2 + 1 }> {
        S1([(); 1], [(); 3])
    }
    #[const_fn("1.61")]
    const fn const_generics4<T: T1<A1 = S1<1, { 2 + 1 }>>, const C: usize>() -> S1<C, { C }> {
        S1([(); C], [(); C])
    }
}

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

    // const_fn_trait_bound (1.61+)

    #[derive(Debug, PartialEq)]
    pub struct A<T>(T);

    impl<T: IntoIterator> A<T> {
        #[const_fn("1.61")]
        const fn const_fn_trait_bound(x: T) -> Self {
            A(x)
        }
    }
    #[rustversion::since(1.61)]
    const _: A<Vec<u8>> = A::const_fn_trait_bound(const_vec_new());

    #[test]
    fn test() {
        assert!(const_min("variables") == "variables");
        assert_eq!(const_let("variables"), "variables");
        assert_eq!(const_vec_new::<u8>(), Vec::new());
        assert_eq!(const_match(1), Some(1));
        assert_eq!(A::const_fn_trait_bound(const_vec_new::<u8>()), A(Vec::new()));
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

    // const_fn_trait_bound (1.61+)

    #[derive(Debug, PartialEq)]
    pub struct A<T>(T);

    impl<T: IntoIterator> A<T> {
        #[const_fn(cfg(rustc_1_61))]
        const fn const_fn_trait_bound(x: T) -> Self {
            A(x)
        }
    }
    #[rustversion::since(1.61)]
    const _: A<Vec<u8>> = A::const_fn_trait_bound(const_vec_new());

    #[test]
    fn test() {
        assert!(const_min("variables") == "variables");
        assert_eq!(const_let("variables"), "variables");
        assert_eq!(const_vec_new::<u8>(), Vec::new());
        assert_eq!(const_match(1), Some(1));
        assert_eq!(A::const_fn_trait_bound(const_vec_new::<u8>()), A(Vec::new()));
    }
}

pub mod macros {
    #![allow(dead_code)]

    use const_fn::const_fn;

    macro_rules! args {
        ($args:expr) => {
            #[const_fn($args)]
            const fn args<T>(x: T) -> T {
                x
            }
        };
    }

    args!("1.31");
    const _: () = args(());
}
