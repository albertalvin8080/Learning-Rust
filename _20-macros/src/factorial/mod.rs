#[macro_export]
macro_rules! factorial {
    (0 | 1) => {
        1
    };
    ($n:expr) => {{
        let mut fat: u32 = 1;
        for i in 1..=$n {
            fat *= i;
        }
        fat
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_factorial() {
        let n = factorial!(5);
        assert_eq!(120, n);
    }
    #[test]
    fn test_factorial_zero() {
        assert_eq!(1, factorial!(0));
    }
    #[test]
    fn test_factorial_one() {
        assert_eq!(1, factorial!(1));
    }
}