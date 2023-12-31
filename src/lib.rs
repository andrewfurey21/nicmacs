#[macro_export]
macro_rules! strexpr {
    ($expr:expr) => {
        stringify!($expr)
    };
}

#[macro_export]
macro_rules! panic_if {
    ($expr:expr) => {
        assert!($expr)
    };
}

#[cfg(test)]
mod panic_if_tests {
    #[test]
    fn panic_if_true() {
        panic_if!(true);
    }
}

#[cfg(test)]
mod strexpr_tests {
    #[test]
    fn strexpr_true() {
        let string = strexpr!(1+2+3);
        assert_eq!(String::from("1 + 2 + 3"), string);
    }
}
