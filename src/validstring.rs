//! # Sherlock and the Valid String
//! 
//! ## Challenge Description
//! 
//! Sherlock considers a string to be valid if all characters of the string appear the same number of times.  
//! It is also valid if he can remove just one character at one index in the string s, and the remaining characters will occur the same number of times.  
//! Given a string, determine if it is valid. If so, return "YES", otherwise return "NO".

use std::collections::HashMap;

/// Returns "YES" if all letters in the string occur an equal number of times, 
/// or if removing a single letter would cause the remaining letters to occur an equal number of times. Otherwise, returns "NO".
/// # Examples
///
/// ```
/// use slothbytes::is_valid;
/// 
/// // Valid, all characters have a count of 1
/// let s = "abc";
/// let expected = "YES".to_string();
/// assert_eq!(is_valid(s), expected);
/// 
/// // Valid, removing a single 'c' would make all character counts 1
/// let s = "abcc";
/// let expected = "YES".to_string();
/// assert_eq!(is_valid(s), expected);
/// 
/// // Invalid, removing a single 'c' doesn't equalize character counts
/// let s = "abccc";
/// let expected = "NO".to_string();
/// assert_eq!(is_valid(s), expected);
/// 
/// // Invalid, no way to remove a single character to equalize counts
/// let s = "aabbcd";
/// let expected = "NO".to_string();
/// assert_eq!(is_valid(s), expected);
/// 
/// // Valid, not explicitly in the challange but  
/// // dropping the 'd' means all remaining characters have a count of 3
/// let s = "aaabbbcccd";
/// let expected = "YES".to_string();
/// assert_eq!(is_valid(s), expected);
/// 
/// // Valid, empty strings are considered valid
/// let s = "";
/// let expected = "YES".to_string();
/// assert_eq!(is_valid(s), expected);
/// ```
pub fn is_valid(s: &str) -> String {
    if s.len() == 0 {
        return "YES".to_string();
    }

    // Get the frequency of every letter in the string
    let mut letters = HashMap::new();
    for c in s.chars() {
        letters.entry(c).and_modify(|counter| *counter += 1).or_insert(1);
    }

    // Grab a frequency and check it against all others
    // for is_valid to be "YES", the frequency grabbed must either:
    // - match every other frequency
    // - match every other frequency, except 1
    // - not match any other frequency (it is the "except 1" frequency)
    // if we find a single frequency that's different than the rest, we have to check to see if:
    // - it is 1 off from the others
    // - it is a frequency of 1 (and can be removed so that all remaining frequenecies match)
    let mut comparison_freq = None; // the frequency we grab
    let mut diff_freq = None; // the frequency that doesn't match the one we grabbed
    let mut total_counter = 0; // total number of frequencies
    let mut diff_counter = 0; // total comparisons that fail
    let mut single_counter = 0; // total amount of single characters
    for freq in letters.into_values() {
        total_counter += 1;
        if freq == 1 {
            single_counter += 1;
        }
        match comparison_freq {
            None => comparison_freq = Some(freq),
            Some(comparison) => {
                if comparison != freq {
                    diff_counter += 1;
                    if diff_freq.is_none() {
                        diff_freq = Some(freq);
                    }
                }
            }
        }
    }
    // a diff count of 0 or 1 is fine
    // if diff count is > 1, the freq we grabbed must fail all comparisons
    // ex. frequencies of [2,2,1,1] will fail this check
    if diff_counter > 1 && diff_counter != total_counter - 1 {
        return "NO".to_string();
    }
    if diff_freq.is_some() {
        // if one frequency that doesn't match the rest, check if they are 1 off from each other
        let diff: i32 = comparison_freq.unwrap() - diff_freq.unwrap();
        if diff.abs() == 1 {
            return "YES".to_string();
        // if one frequency that doesn't match the rest, check if the frequency is 1
        // ex. for [3, 3, 3, 3, 1], the 1 can be dropped so it is all 3s
        // however for [1, 1, 1, 1, 3] we can't drop a 1
        } else if (comparison_freq.unwrap() == 1 || diff_freq.unwrap() == 1) && single_counter == 1 {
            return "YES".to_string();
        } else {
            return "NO".to_string();
        }
    }
    // getting to this point means no comparisons failed
    "YES".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn abc() {
        let s = "abc";
        let expected = "YES".to_string();
        assert_eq!(is_valid(s), expected);
    }

    #[test]
    fn abcc() {
        let s = "abcc";
        let expected = "YES".to_string();
        assert_eq!(is_valid(s), expected);
    }

    #[test]
    fn abccc() {
        let s = "abccc";
        let expected = "NO".to_string();
        assert_eq!(is_valid(s), expected);
    }

    #[test]
    fn aabbcd() {
        let s = "aabbcd";
        let expected = "NO".to_string();
        assert_eq!(is_valid(s), expected);
    }

    #[test]
    fn aabbccddeefghi() {
        let s = "aabbccddeefghi";
        let expected = "NO".to_string();
        assert_eq!(is_valid(s), expected);
    }

    #[test]
    fn abcdefghhgfedecba() {
        let s = "abcdefghhgfedecba";
        let expected = "YES".to_string();
        assert_eq!(is_valid(s), expected);
    }

    #[test]
    fn drop() {
        let s = "aaabbbcccd";
        let expected = "YES".to_string();
        assert_eq!(is_valid(s), expected);
    }

    #[test]
    fn empty_string() {
        let s = "";
        let expected = "YES".to_string();
        assert_eq!(is_valid(s), expected);
    }
}