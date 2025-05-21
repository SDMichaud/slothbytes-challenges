//! # Sticky Keys Typing
//! 
//! ## Challenge Description
//! 
//! Someone is typing on the sticky keyboard. Occasionally a key gets stuck and more than intended number of characters of a particular letter is being added into the string.  
//! - The function input contains original and typed strings.
//! - Determine if the typed string has been made from the original.  
//! - Return true if it is and false if the typed string cannot have been made from the original

use std::collections::HashMap;

/// Determines if the typed string can be made from repeating letters in the original string.
/// # Examples
///
/// ```
/// use slothbytes::is_long_pressed;
/// 
/// let original = "alex";
/// let typed = "aaleex";
/// assert!(is_long_pressed(original, typed));
/// ```
pub fn is_long_pressed(original: &str, typed: &str) -> bool {
    let original_char_count = count_chars(original);
    let typed_char_count = count_chars(typed);
    if original_char_count.keys().len() != typed_char_count.keys().len() {
        return false;
    }
    for c in original_char_count.keys() {
        if typed_char_count.get(c).expect("Expected key to exist.") < original_char_count.get(c).expect("Expected key to exist.") {
            return false;
        }
    }
    true
}

/// Takes a string and returns a Hashmap where each character of the string is a key and it's value is the frequency that character occured in the string.
/// # Examples
/// 
/// 
/// let s = "Hello, World!";  
/// let count = count_chars(s);  
/// assert_eq!(*count.get("l").unwrap(), 3);  
fn count_chars(s: &str) -> HashMap<String, i32> {
    let mut char_count: HashMap<String, i32> = HashMap::new();
    for c in s.chars() {
        char_count.entry(c.to_string())
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }
    char_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn long_press(){
        let original = "alex";
        let typed = "aaleex";
        assert!(is_long_pressed(original, typed));
    }

    #[test]
    fn not_long_press(){
        let original = "saeed";
        let typed = "ssaaedd";
        assert!(!is_long_pressed(original, typed));
    }

    #[test]
    fn different_letters(){
        let original = "Tokyo";
        let typed = "TTokkyoh";
        assert!(!is_long_pressed(original, typed));
    }

    #[test]
    fn same_text(){
        let original = "laiden";
        let typed = "laiden";
        assert!(is_long_pressed(original, typed));
    }
}