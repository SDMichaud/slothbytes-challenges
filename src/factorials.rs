//! # Factorial of Factorials
//! 
//! ## Challenge Description
//! 
//! Create a function that takes an integer, n, and returns the factorial of factorials. See below examples for a better understanding:
//! ex: fact_of_fact(4) = 4! * 3! * 2! * 1! = 288

/// Calculates a factorial of factorials. Return value is i64 to support large values.
/// 
/// # Examples
/// 
/// ```
/// use slothbytes::fact_of_fact;
/// 
/// // fact_of_fact(4) = 4! * 3! * 2! * 1! = 288
/// let n: u32 = 4;
/// let expected: u64 = 288;
/// assert_eq!(fact_of_fact(n), expected);
/// 
/// ```
pub fn fact_of_fact(n: u32) -> u64 {
    if n<= 1 {
        return 1;
    }
    fact_of_fact(n-1) * factorial(n) as u64
}

fn factorial(n: u32) -> u64 {
    if n <= 1 {
        return 1;
    }
    factorial(n-1) * n as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n_of_4() {
        let n = 4;
        let expected = 288;
        assert_eq!(fact_of_fact(n), expected)
    }

    #[test]
    fn n_of_5() {
        let n = 5;
        let expected = 34560;
        assert_eq!(fact_of_fact(n), expected)
    }

    #[test]
    fn n_of_6() {
        let n = 6;
        let expected = 24883200;
        assert_eq!(fact_of_fact(n), expected)
    }
}