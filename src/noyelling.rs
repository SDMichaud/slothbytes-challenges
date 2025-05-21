//! # No Yelling
//! 
//! ## Challenge Description
//! 
//! Create a function that transforms sentences ending with multiple question marks ? or exclamation marks ! into a sentence
//! only ending with one without changing punctuation in the middle of the sentences.
//! 
//! - Only change ending punctuation - keep the exclamation marks or question marks in the middle of the sentence the same (see third example).
//! - Don't worry about mixed punctuation (no cases that end in something like ?!??!).
//! - Keep sentences that do not have question/exclamation marks the same.

/// Returns the string with repeated '?' or '!' characters removed from last word only.
/// # Examples
///
/// ```
/// use slothbytes::no_yelling;
/// 
/// let s = "I just!!! can!!! not!!! believe!!! it!!!";
/// let expected = "I just!!! can!!! not!!! believe!!! it!";
/// assert_eq!(no_yelling(s), expected);
/// 
/// let s = "Oh my goodness!";
/// let expected = "Oh my goodness!";
/// assert_eq!(no_yelling(s), expected);
/// ```
pub fn no_yelling(s: &str) -> String {
    // check for empty string
    if s.len() == 0 {
        return String::new()
    }
    let words = s.split_whitespace().collect::<Vec<&str>>();
    let last_word = *words.last().expect("Expected at least one word.");
    let last_word_chars = last_word.chars().collect::<Vec<char>>();
    // if last word is a single character there is nothing to change
    if last_word_chars.len() <= 1 {
        return String::from(s);
    }
    let last_char = last_word_chars.last().expect("Expected at least one character.");
    // if the last word doesn't end in '?' or '!' there is nothing to change
    if *last_char != '?' && *last_char != '!' {
        return String::from(s);
    }
    let penultimate_char = &last_word_chars[last_word_chars.len()-2];
    // by this point, last_char is either '?' or '!'
    // penultimate (second-to-last) character must match last character if there is going to be repeated punctuation
    // if they don't match, there is no repeating punctuation and nothing to change
    if *penultimate_char != *last_char {
        return String::from(s);
    }
    let mut last_word_no_punctuation = last_word.split(*last_char).collect::<String>();
    last_word_no_punctuation.push(*last_char);
    if words.len() <= 1 {
        return last_word_no_punctuation;
    } else {
        let mut new_string = words[0..words.len()-1].join(" ");
        new_string.push(' ');
        new_string.push_str(&last_word_no_punctuation);
        return new_string;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_question() {
        let s = "What went wrong?????????";
        let expected = "What went wrong?";
        assert_eq!(no_yelling(s), expected);
    }

    #[test]
    fn single_exclimation() {
        let s = "Oh my goodness!!!";
        let expected = "Oh my goodness!";
        assert_eq!(no_yelling(s), expected);
    }

    #[test]
    fn multiple_exclimation() {
        let s = "I just!!! can!!! not!!! believe!!! it!!!";
        let expected = "I just!!! can!!! not!!! believe!!! it!";
        assert_eq!(no_yelling(s), expected);
    }

    #[test]
    fn no_change() {
        let s = "Oh my goodness!";
        let expected = "Oh my goodness!";
        assert_eq!(no_yelling(s), expected);
    }

    #[test]
    fn no_punctuation() {
        let s = "sup";
        let expected = "sup";
        assert_eq!(no_yelling(s), expected);
    }
}