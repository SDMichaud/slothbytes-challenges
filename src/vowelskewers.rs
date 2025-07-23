//! # Vowel Skewers
//! 
//! ## Challenge Description
//! 
//! An authentic vowel skewer is a skewer with a delicious and juicy mix of consonants and vowels. However, the way they are made must be just right:
//! - Skewers must begin and end with a consonant.
//! - Skewers must alternate between consonants and vowels.
//! - There must be an even spacing between each letter on the skewer, so that there is a consistent flavour throughout.  
//! 
//! Create a function which returns whether a given vowel skewer is authentic

#[derive(PartialEq, Debug)]
enum CharacterType {
    Consonant,
    Vowel,
    Skewer
}

/// Returns true if the string starts and ends with a consonant, has letters divided by equal sized skewers '-',
/// and alternates between consonants and vowels. Empty strings and strings containing characters outside the alphabet will return false.
/// # Examples
///
/// ```
/// use slothbytes::is_authentic_skewer;
/// 
/// // True
/// let s = "B--A--N--A--N--A--S";
/// assert!(is_authentic_skewer(s));
/// 
/// // False, ends on a vowel
/// let s = "B---A---B---A";
/// assert!(!is_authentic_skewer(s));
/// 
/// // False, starts and ends with vowels
/// let s = "A-X-E";
/// assert!(!is_authentic_skewer(s));
/// 
/// // False, varying skewer lengths
/// let s = "M--A---T-E-S";
/// assert!(!is_authentic_skewer(s));
/// ```
pub fn is_authentic_skewer(s: &str) -> bool {
    if s.len() == 0 {
        return false
    }
    let mut looking_for = CharacterType::Consonant;
    let mut prev_non_skewer_type = CharacterType::Consonant;
    let mut skewer_length = None; // will hold the number of skewers expected, set by the first string of skewers seen
    let mut skewer_count = 0;
    for ch in s.chars() {
        let ch_type_option = get_ch_type(ch);
        if ch_type_option.is_none() {
            return false
        }
        let ch_type = ch_type_option.unwrap();
        if ch_type != looking_for {
            // A valid string will have a first consonant, a string of skewers, and a vowel
            // At first, we don't know how many skewers to expect, so skewers will always expect another skewer to follow
            // We don't want to return false the first time we hit a vowel. We instead want to set the skewer length and continue as normal.
            if skewer_length.is_none() && skewer_count > 0 && prev_non_skewer_type == CharacterType::Consonant {
                looking_for = CharacterType::Vowel; // was expecting Skewer, need to alter expectations to Vowel.
                skewer_length = Some(skewer_count);
                skewer_count = 0
            } else {
                return false
            }
        }
        match looking_for {
            CharacterType::Consonant => {
                prev_non_skewer_type = CharacterType::Consonant;
                looking_for = CharacterType::Skewer;
            },
            CharacterType::Vowel => {
                prev_non_skewer_type = CharacterType::Vowel;
                looking_for = CharacterType::Skewer;
            },
            // keep track of how many skewers in a row
            // if we havent hit the expected number, the next character should be a skewer
            // otherwise the next character should be the opposite of the last non_skewer_type
            CharacterType::Skewer => {
                skewer_count += 1;
                if let Some(expected_len) = skewer_length {
                    if skewer_count == expected_len {
                        skewer_count = 0;
                        if prev_non_skewer_type == CharacterType::Consonant {
                            looking_for = CharacterType::Vowel;
                        } else {
                            looking_for = CharacterType::Consonant;
                        }
                    }
                }
            }
        }
    }
    if prev_non_skewer_type == CharacterType::Consonant {
        return true;
    } else {
        return false;
    }
}

fn get_ch_type(ch: char) -> Option<CharacterType> {
    if ch == '-' {
        return Some(CharacterType::Skewer);
    }
    if "aeiouAEIOU".contains(ch) {
        return  Some(CharacterType::Vowel);
    }
    if "bcdfghjklmnpqrstvwxzBCDFGHJKLMNPQRSTVWXZ".contains(ch) {
        return Some(CharacterType::Consonant);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bananas() {
        let s = "B--A--N--A--N--A--S";
        assert!(is_authentic_skewer(s));
    }

    #[test]
    fn babab() {
        let s = "B---A---B---A---B---A---B";
        assert!(is_authentic_skewer(s));
    }

    #[test]
    fn ends_on_vowel() {
        let s = "B---A---B---A---B---A";
        assert!(!is_authentic_skewer(s));
    }

    #[test]
    fn axe() {
        let s = "A--X--E";
        assert!(!is_authentic_skewer(s));
    }

    #[test]
    fn clap() {
        let s = "C-L-A-P";
        assert!(!is_authentic_skewer(s));
    }

    #[test]
    fn mates() {
        let s = "M--A---T-E-S";
        assert!(!is_authentic_skewer(s));
    }

    #[test]
    fn empty_string() {
        let s = "";
        assert!(!is_authentic_skewer(s));
    }

    #[test]
    fn no_skewer() {
        let s = "TEST";
        assert!(!is_authentic_skewer(s));
    }

    #[test]
    fn no_letters() {
        let s = "------";
        assert!(!is_authentic_skewer(s));
    }

    #[test]
    fn numbers() {
        let s = "B-A-1-B";
        assert!(!is_authentic_skewer(s));
    }
}