//! # Next Happy Year
//! 
//! ## Challenge Description
//! 
//! Sloth needs your help to find out the next happy year.  
//! A Happy Year is the year with only distinct digits (no duplicates).  
//! Create a function that takes an integer year and returns the next happy year.  

/// Returns the next Happy Year, a year without repeating digits.
/// # Examples
///
/// ```
/// use slothbytes::happy_year;
/// 
/// assert_eq!(happy_year(1990), 2013);
/// ```
pub fn happy_year(year: i32) -> i32 {
    let mut next = year;
    loop {
        next += 1;
        if is_all_digits_unique(next) {
            return next;
        }
    }
}

/// Returns true if all digits in a number are unique.
fn is_all_digits_unique(i: i32) -> bool {
    let mut digits: Vec<i32> = Vec::new();
    let mut n = i;
    while n > 0 {
        let rem = n % 10;
        if digits.contains(&rem) {
            return false;
        }
        digits.push(rem);
        n = n / 10;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unique_digits() {
        let i = 1234567890;
        assert!(is_all_digits_unique(i));
    }

    #[test]
    fn duplicate_digits() {
        let i = 1992;
        assert!(!is_all_digits_unique(i));
    }

    #[test]
    fn happy_year_test() {
        assert_eq!(happy_year(2017), 2018);
        assert_eq!(happy_year(1990), 2013);
        assert_eq!(happy_year(2021), 2031);
    }
}