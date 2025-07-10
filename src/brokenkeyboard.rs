//! # Broken Keyboard
//! 
//! ## Challenge Description
//! 
//! Given what is supposed to be typed and what is actually typed, write a function that returns the broken key(s). The function looks like: 
//! findBrokenKeys(correct phrase, what you actually typed)

/// Finds the letters in correct that do not appear in typed
/// 
/// # Examples
/// 
/// ```
/// use slothbytes::find_broken_keys;
/// 
/// let correct = "starry night";
/// let typed = "starrq light";
/// let expected = vec!['y', 'n'];
/// assert_eq!(find_broken_keys(correct, typed), expected)
/// 
/// ```
pub fn find_broken_keys(correct: &str, typed: &str) -> Vec<char> {
    let unique_letters = get_unique_letters(correct);
    let mut broken_keys = Vec::new();
    for c in unique_letters {
        if !typed.contains(c) {
            broken_keys.push(c);
        }
    }
    broken_keys
}

fn get_unique_letters(s: &str) -> Vec<char> {
    let mut unique_letters = Vec::new();
    for c in s.chars() {
        if !unique_letters.contains(&c) {
            unique_letters.push(c);
        }
    }
    unique_letters
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_birthday() {
        let correct = "happy birthday";
        let typed = "hawwy birthday";
        let expected = vec!['p'];
        assert_eq!(find_broken_keys(correct, typed), expected)
    }

    #[test]
    fn starry_night() {
        let correct = "starry night";
        let typed = "starrq light";
        let expected = vec!['y', 'n'];
        assert_eq!(find_broken_keys(correct, typed), expected)
    }

    #[test]
    fn beethoven() {
        let correct = "beethoven";
        let typed = "affthoif5";
        let expected = vec!['b', 'e', 'v', 'n'];
        assert_eq!(find_broken_keys(correct, typed), expected)
    }
}