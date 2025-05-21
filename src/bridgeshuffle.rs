//! # Bridge Shuffle
//! 
//! ## Challenge Description
//! 
//! Create a function to bridge shuffle two arrays.
//! To bridge shuffle, you interleave the elements from both arrays in an alternating fashion.
//! This can still work with two arrays of uneven length. We simply tack on the extra elements from the longer array.
//! 
//! - Elements in both arrays can be strings or integers.
//! - If two arrays are of unequal length, add the additional elements of the longer array to the end of the shuffled array.
//! - Always start your shuffle with the first element of Array 1.

/// Bridge Shuffles two arrays together by interleaving the elements in an alternating fashion.
/// 
/// # Examples
/// 
/// ```
/// let list_a: Vec<i32> = vec![1, 3, 5, 7];
/// let list_b: Vec<i32> = vec![2, 4, 6];
/// let expected: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7];
/// assert_eq!(expected, slothbytes::bridgeshuffle(list_a, list_b));
/// 
/// let list_a: Vec<&str> = vec!["A", "A", "A"];
/// let list_b: Vec<&str> = vec!["B", "B", "B"];
/// let expected: Vec<&str> = vec!["A", "B", "A", "B", "A", "B"];
/// assert_eq!(expected, slothbytes::bridgeshuffle(list_a, list_b));
/// 
/// ```
pub fn bridgeshuffle<T: Copy>(list_a: Vec<T>, list_b: Vec<T>) -> Vec<T> {
    let a_len = list_a.len();
    let b_len = list_b.len();
    let mut result: Vec<T> = Vec::new();
    if a_len == b_len {
        for n in 0..a_len {
            result.push(list_a[n]);
            result.push(list_b[n]);
        }
        result
    } else if a_len > b_len {
        for n in 0..b_len {
            result.push(list_a[n]);
            result.push(list_b[n]);
        }
        for i in b_len..a_len {
            result.push(list_a[i]);
        }
        result
    } else {
        for n in 0..a_len {
            result.push(list_a[n]);
            result.push(list_b[n]);
        }
        for i in a_len..b_len {
            result.push(list_b[i]);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equal_sized_char_lists() {
        let list_a = vec!["A", "A", "A"];
        let list_b = vec!["B", "B", "B"];
        let expected = vec!["A", "B", "A", "B", "A", "B"];
        let result = bridgeshuffle(list_a, list_b);
        assert_eq!(result, expected);
    }

    #[test]
    fn list_a_larger() {
        let list_a = vec!["C", "C", "C", "C"];
        let list_b = vec!["D"];
        let expected = vec!["C", "D", "C", "C", "C"];
        let result = bridgeshuffle(list_a, list_b);
        assert_eq!(result, expected);
    }

    #[test]
    fn list_b_larger() {
        let list_a = vec!["E"];
        let list_b = vec!["B", "B", "B"];
        let expected = vec!["E", "B", "B", "B"];
        let result = bridgeshuffle(list_a, list_b);
        assert_eq!(result, expected);
    }

    #[test]
    fn list_a_larger_i32() {
        let list_a = vec![1, 3, 5, 7];
        let list_b = vec![2, 4, 6];
        let expected = vec![1, 2, 3, 4, 5, 6, 7];
        let result = bridgeshuffle(list_a, list_b);
        assert_eq!(result, expected);
    }
}