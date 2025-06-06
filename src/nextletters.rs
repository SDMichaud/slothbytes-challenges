//! # Next in the Alphabet
//! 
//! ## Challenge Description
//! 
//! Create a function which returns the next letters alphabetically in a given string. If the last letter is a "Z", change the rest of the letters accordingly.
//! - Tests will all be in CAPITALS.
//! - Empty inputs should return a capital "A" (as if it were in letter position 0!).
//! - Think about the letter "Z" like the number 9 and how it carries over to increment the next letter/digit over.
//! 

/// Returns the string incremented by 1. Eg. A->B, B->C, ..., Z->AA. Empty string returns "A".
/// # Examples
///
/// ```
/// use slothbytes::next_letters;
/// 
/// // A->B
/// let s = "A";
/// let expected = "B";
/// assert_eq!(next_letters(s), expected);
/// 
/// // C->D, rest of the string unchanged
/// let s = "ABC";
/// let expected = "ABD";
/// assert_eq!(next_letters(s), expected);
/// 
/// // Z->AA, like 9->10
/// let s = "Z";
/// let expected = "AA";
/// assert_eq!(next_letters(s), expected);
/// 
/// // Z increments as well as A, like 19->20
/// let s = "CAZ";
/// let expected = "CBA";
/// assert_eq!(next_letters(s), expected);
/// ```
pub fn next_letters(s: &str) -> String {
    if s.len() == 0 {
        return "A".into();
    }
    let last_char = s.chars().last().expect("Expected string to not be empty.");
    let mut ret: String = s.chars().take(s.len()-1).collect();
    if last_char != 'Z' {
        let next_char = char::from_u32(last_char as u32 + 1).expect("Expected to be able to increment char.");
        ret.push(next_char);
    } else {
        ret = next_letters(&ret);
        ret.push('A');
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_to_b() {
        let s = "A";
        let expected = "B";
        assert_eq!(next_letters(s), expected);
    }

    #[test]
    fn abc() {
        let s = "ABC";
        let expected = "ABD";
        assert_eq!(next_letters(s), expected);
    }

    #[test]
    fn z_rollover() {
        let s = "Z";
        let expected = "AA";
        assert_eq!(next_letters(s), expected);
    }

    #[test]
    fn multi_z_rollover() {
        let s = "ZZZ";
        let expected = "AAAA";
        assert_eq!(next_letters(s), expected);
    }

    #[test]
    fn caz_rollover() {
        let s = "CAZ";
        let expected = "CBA";
        assert_eq!(next_letters(s), expected);
    }

    #[test]
    fn empty_string() {
        let s = "";
        let expected = "A";
        assert_eq!(next_letters(s), expected);
    }
}