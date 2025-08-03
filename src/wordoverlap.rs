//! # Word Overlapping
//! 
//! ## Challenge Description
//! 
//! Given two words, overlap them in such a way, morphing the last few letters of the first word with the first few letters of the second word.  
//! Return the shortest overlapped word possible

/// Attempts to overlap as many matching letters from the end of the first word with the start of the second word.
/// Returns the two words pushed together if overlap is not possible.
/// # Examples
///
/// ```
/// use slothbytes::overlap;
/// 
/// // overlap of 'den'
/// let a = "sweden";
/// let b = "denmark";
/// let expected = "swedenmark";
/// assert_eq!(overlap(a, b), expected);
/// 
/// // cannot overlap
/// let a = "honey";
/// let b = "milk";
/// let expected = "honeymilk";
/// assert_eq!(overlap(a, b), expected);
/// 
/// // complete overlap
/// let a = "dodge";
/// let b = "dodge";
/// let expected = "dodge";
/// assert_eq!(overlap(a, b), expected);
/// 
/// // 'to' does not overlap with the beginning of 'kyoto'
/// let a = "to";
/// let b = "kyoto";
/// let expected = "tokyoto";
/// ```
pub fn overlap(a: &str, b: &str) -> String {
    // checks for empty or matching strings.
    if a.len() == 0 {
        return b.to_string();
    }
    if b.len() == 0 {
        return a.to_string();
    }
    if a == b {
        return a.to_string();
    }
    
    // return words together if last letter of first word isn't found in second word.
    let firstword_lastletter = a.chars().last().unwrap();
    if !b.contains(firstword_lastletter) {
        return join_words(a, b);
    }

    // find how deep into second word the letter is found
    let b_char_array = b.chars().collect::<Vec<char>>();
    let mut match_index = 0;
    for (ii, ch) in b_char_array.iter().enumerate() {
        if *ch == firstword_lastletter {
            match_index = ii;
            break;
        }
    }

    // if the depth is greater than the lenght of the first word, no overlap is possible
    if match_index+1 > a.len() {
        return join_words(a, b);
    }

    // loops backwards through both words, checking each letter matches
    // the first word starts at the last letter
    // the second word starts at the matched index
    // if all letters do not much, return with no overlap
    let mut a_rev_char_array = a.chars().rev().collect::<Vec<char>>();
    for (first_index, second_index) in (0..match_index+1).rev().enumerate() {
        if a_rev_char_array[first_index] != b_char_array[second_index] {
            return join_words(a, b);
        }
    }

    // overlap is possible, remove the matching letters from the first word and then join it to the second
    a_rev_char_array.reverse();
    for _ in 0..match_index+1 {
        a_rev_char_array.pop();
    }
    let s = String::from_iter(a_rev_char_array.into_iter());
    join_words(&s, b)
}

fn join_words(a: &str, b: &str) -> String {
    let mut ret = a.to_string();
    ret.push_str(b);
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sweden_denmark() {
        let a = "sweden";
        let b = "denmark";
        let expected = "swedenmark";
        assert_eq!(overlap(a, b), expected);
    }

    #[test]
    fn honey_milk() {
        let a = "honey";
        let b = "milk";
        let expected = "honeymilk";
        assert_eq!(overlap(a, b), expected);
    }

    #[test]
    fn dodge_dodge() {
        let a = "dodge";
        let b = "dodge";
        let expected = "dodge";
        assert_eq!(overlap(a, b), expected);
    }

    #[test]
    fn town_nice() {
        let a = "town";
        let b = "nice";
        let expected = "townice";
        assert_eq!(overlap(a, b), expected);
    }

    #[test]
    fn empty_strings() {
        let a = "";
        let b = "";
        let expected = "";
        assert_eq!(overlap(a, b), expected);
    }

    #[test]
    fn a_empty() {
        let a = "";
        let b = "empty";
        let expected = "empty";
        assert_eq!(overlap(a, b), expected);
    }

    #[test]
    fn b_empty() {
        let a = "empty";
        let b = "";
        let expected = "empty";
        assert_eq!(overlap(a, b), expected);
    }

    #[test]
    fn to_tokyo() {
        let a = "to";
        let b = "tokyo";
        let expected = "tokyo";
        assert_eq!(overlap(a, b), expected);
    }

    #[test]
    fn kyoto_to() {
        let a = "kyoto";
        let b = "to";
        let expected = "kyoto";
        assert_eq!(overlap(a, b), expected);
    }

    #[test]
    fn to_kyoto() {
        let a = "to";
        let b = "kyoto";
        let expected = "tokyoto";
        assert_eq!(overlap(a, b), expected);
    }
}