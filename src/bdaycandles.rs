//! # Birthday Cake Candles
//! 
//! ## Challenge Description
//! 
//!  You are in charge of the cake for a child's birthday. It will have one candle for each year of their total age. They will only be able to blow out the tallest of the candles.
//!  Your task is to count how many candles are the tallest.


/// Counts the number of occurances of the highest number. Returns 0 for empty vectors.
/// 
/// # Examples
/// 
/// ```
/// use slothbytes::birthday_cake_candles;
/// 
/// let candles = vec![4,4,1,3];
/// let expected = 2;
/// assert_eq!(birthday_cake_candles(candles), expected);
/// 
/// // Empty vector
/// let candles: Vec<i32> = Vec::new();
/// let expected = 0;
/// assert_eq!(birthday_cake_candles(candles), expected);
/// 
/// ```
pub fn birthday_cake_candles(candles: Vec<i32>) -> i32 {
    if candles.len() == 0 {
        return 0;
    }
    let maximum = candles.iter().max().expect("Expected to find a maximum value in vector.");
    candles.iter().filter(|n| *n == maximum).count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_tallest() {
        let candles = vec![4,4,1,3];
        let expected = 2;
        assert_eq!(birthday_cake_candles(candles), expected);
    }

    #[test]
    fn all_same_height() {
        let candles = vec![1, 1, 1, 1];
        let expected = 4;
        assert_eq!(birthday_cake_candles(candles), expected);
    }

    #[test]
    fn empty_vector() {
        let candles: Vec<i32> = Vec::new();
        let expected = 0;
        assert_eq!(birthday_cake_candles(candles), expected);
    }
}