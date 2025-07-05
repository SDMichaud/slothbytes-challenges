//! # How Many Digits between 0 and N
//! 
//! ## Challenge Description
//! 
//! Imagine you took all the numbers between 0 and n and concatenated them together into a long string.
//! How many digits are there between 0 and n? Write a function that can calculate this.
//! There are 0 digits between 0 and 1, there are 9 digits between 0 and 10 and there are 189 digits between 0 and 100.


/// Takes all the numbers between 0 and n and counts the total number of digits
/// 
/// # Examples
/// 
/// ```
/// use slothbytes::digits_between;
/// 
/// // 123456789 -> 9 digits total
/// let n = 10;
/// let expected = 9;
/// assert_eq!(digits_between(n), expected);
/// 
/// let n = 100;
/// let expected = 189;
/// assert_eq!(digits_between(n), expected);
/// 
/// ```
pub fn digits_between(n: i32) -> i32 {
    let numbers: Vec<i32> = (1..n).collect();
    let result = numbers.into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .concat()
        .len() as i32;
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n_is_1() {
        let n = 1;
        let expected = 0;
        assert_eq!(digits_between(n), expected);
    }

    #[test]
    fn n_is_10() {
        let n = 10;
        let expected = 9;
        assert_eq!(digits_between(n), expected);
    }

    #[test]
    fn n_is_100() {
        let n = 100;
        let expected = 189;
        assert_eq!(digits_between(n), expected);
    }

    #[test]
    fn n_is_2020() {
        let n = 2020;
        let expected = 6969;
        assert_eq!(digits_between(n), expected);
    }
}