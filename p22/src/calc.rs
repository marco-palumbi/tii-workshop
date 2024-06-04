/// this calculate celsius from farenheit
/// ```
/// use p22::calc::celsius2farenheit;
/// let f = celsius2farenheit(7);
/// assert_eq!(f, 44)
/// ```
pub fn celsius2farenheit(celsius: i32) -> i32 {
    ((celsius * 9) / 5) + 32
}

pub fn farenheit2celsius(farenhait: i32) -> i32 {
    (farenhait - 32) * 5 / 9
}

pub fn fibonacci_loop(n: u32) -> u32 {
    if n == 0 {
        0u32
    } else if n == 1 {
        1u32
    } else {
        let mut prev_1 = 0;
        let mut prev_2 = 1;
        for _ in 0..n - 1 {
            let current = prev_1 + prev_2;
            prev_1 = prev_2;
            prev_2 = current;
        }
        prev_2
    }
}

pub fn fibonacci_rec(n: u32) -> u32 {
    if n == 0 {
        0u32
    } else if n == 1 {
        1u32
    } else {
        fibonacci_rec(n - 2) + fibonacci_rec(n - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = celsius2farenheit(3);
        assert_eq!(result, 37);
    }
    #[test]
    fn also_this_works() {
        let result = celsius2farenheit(6);
        assert_eq!(result, 42);
    }
    #[test]
    fn test1_farenheit2celsius() {
        let result = farenheit2celsius(42);
        assert_eq!(result, 5)
    }
    #[test]
    fn test0_fibonacci_loop() {
        let result = fibonacci_loop(0);
        assert_eq!(result, 0)
    }
    #[test]
    fn test1_fibonacci_loop() {
        let result = fibonacci_loop(1);
        assert_eq!(result, 1)
    }
    #[test]
    fn test2_fibonacci_loop() {
        let result = fibonacci_loop(2);
        assert_eq!(result, 1)
    }
    #[test]
    fn test3_fibonacci_loop() {
        let result = fibonacci_loop(19);
        assert_eq!(result, 4181)
    }
    #[test]
    fn test0_fibonacci_rec() {
        let result = fibonacci_rec(0);
        assert_eq!(result, 0)
    }
    #[test]
    fn test1_fibonacci_rec() {
        let result = fibonacci_rec(1);
        assert_eq!(result, 1)
    }
    #[test]
    fn test2_fibonacci_rec() {
        let result = fibonacci_rec(2);
        assert_eq!(result, 1)
    }
    #[test]
    fn test3_fibonacci_rec() {
        let result = fibonacci_rec(19);
        assert_eq!(result, 4181)
    }
}
