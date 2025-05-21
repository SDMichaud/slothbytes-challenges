#![allow(unused_assignments)]
//! # Space Message
//! 
//! ## Challenge Description
//! 
//! You have received an encrypted message from space. Your task is to decrypt the message with the following simple rules:
//! - Message string will consist of capital letters, numbers, and brackets only.
//! - When there's a block of code inside the brackets, such as \[10AB\], it means you need to repeat the letters AB for 10 times.
//! - Message can be embedded in multiple layers of blocks.
//! - Final decrypted message will only consist of capital letters.
//! 
//! Create a function that takes encrypted message `str` and returns the decrypted message.
use std::iter::repeat;

/// Takes an encrypted message and returns a decrypted message.
/// # Examples
///
/// ```
/// let encrypted = "IF[2E]LG[5O]D";
/// let decrypted = slothbytes::spacemessage(encrypted);
/// assert_eq!("IFEELGOOOOOD", decrypted);
/// ```
pub fn spacemessage(message: &str) -> String {
    let mut is_skip = false;
    let mut compressed_strings = extract_between_brackets(message);
    compressed_strings.reverse();
    let mut space_message = String::new();
    for character in message.chars() {
        if is_skip {
            if character == ']' {
                is_skip = false;
            }
        } else {
            if character == '[' {
                is_skip = true;
                let compressed_string = compressed_strings.pop().expect("Expected an element.");
                space_message.push_str(&expand(&extract_string(&compressed_string), extract_number(&compressed_string)));
            } else {
                space_message.push(character);
            }
        }
    };
    space_message
}

fn expand(char: &str, count: i32) -> String {
    repeat(char).take(count as usize).collect::<String>()
}

fn extract_between_brackets(text: &str) -> Vec<String> {
    let mut between_brackets: Vec<String> = Vec::new();
    let mut start_pos: usize = 0;
    let mut end_pos: usize = 0;
    let mut global_pos: usize = 0;
    loop {
        let text = &text[global_pos..];
        match text.find('[') {
            Some(byte) => start_pos = byte,
            None => return between_brackets
        }
        end_pos = text.find(']').expect("Expected a closing bracket.");
        between_brackets.push(text[start_pos+1..end_pos].to_string());
        global_pos += end_pos+1;
        start_pos = end_pos+1;
    }
}

fn extract_number(text: &str) -> i32 {
    text.chars().take_while(|x| x.is_numeric()).collect::<String>().parse::<i32>().expect("Expected to parse number.")
}

fn extract_string(text: &str) -> String {
    let start_pos = text.find(char::is_alphabetic).expect("Expected to find a character.");
    text[start_pos..].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiple_brackets() {
        let test_string = "IF[2E]LG[5O]D";
        let expected = vec!["2E", "5O"];
        let result = extract_between_brackets(test_string);
        assert_eq!(result, expected);
    }

    #[test]
    fn single_brackets() {
        let test_string = "AB[3CD]";
        let expected = vec!["3CD"];
        let result = extract_between_brackets(test_string);
        assert_eq!(result, expected);
    }

    #[test]
    fn no_brackets() {
        let test_string = "ABC";
        let expected: Vec<String> = Vec::new();
        let result = extract_between_brackets(test_string);
        assert_eq!(result, expected);
    }

    #[test]
    fn single_digit() {
        let test_string = "3CD";
        let expected = 3;
        let result = extract_number(test_string);
        assert_eq!(result, expected);
    }

    #[test]
    fn multi_digit() {
        let test_string = "10O";
        let expected = 10;
        let result = extract_number(test_string);
        assert_eq!(result, expected);
    }

    #[test]
    fn single_char() {
        let test_string = "3C";
        let expected = "C";
        let result = extract_string(test_string);
        assert_eq!(result, expected);
    }

    #[test]
    fn multi_char() {
        let test_string = "3CD";
        let expected = "CD";
        let result = extract_string(test_string);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_space_message_no_brackets() {
        let test_string = "ABCD";
        let expected = "ABCD";
        let result = spacemessage(test_string);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_space_message_one_bracket_multi_char() {
        let test_string = "AB[3CD]";
        let expected = "ABCDCDCD";
        let result = spacemessage(test_string);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_space_message_multi_bracket() {
        let test_string = "IF[2E]LG[5O]D";
        let expected = "IFEELGOOOOOD";
        let result = spacemessage(test_string);
        assert_eq!(result, expected);
    }
}