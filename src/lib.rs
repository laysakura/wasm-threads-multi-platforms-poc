#[no_mangle]
pub extern "C" fn fib(n: u32) -> u32 {
    if n <= 1 {
        n
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fib_0() {
        assert_eq!(fib(0), 0);
    }
    #[test]
    fn fib_1() {
        assert_eq!(fib(1), 1);
    }
    #[test]
    fn fib_10() {
        assert_eq!(fib(10), 55);
    }
}
