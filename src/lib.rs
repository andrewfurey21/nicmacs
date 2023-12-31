macro_rules! strexpr {
    () => {};
    ($arg1:expr, $arg2:expr, eq) => {
        format!("{:?}\nlhs: {:?}\nrhs: {:?}", stringify!($arg1 == $arg2), $arg1, $arg2)
    };
}

#[macro_export]
macro_rules! panic_eq {
    ($arg1:expr, $arg2:expr) => {
        if $arg1 == $arg2 {
            panic!("{}", strexpr!($arg1, $arg2, eq))
        }
    };
}

#[cfg(test)]
mod panic_if_tests {
    #[test]
    #[should_panic]
    fn panic_if_a_equals_b_should_panic() {
        let a = 10;
        let b = 10;
        let c = 0;
        panic_eq!(a + c, b);
    }

    #[test]
    fn panic_if_a_equals_b() {
        let a = 10;
        let b = 0;
        panic_eq!(a, b);
    }
}
