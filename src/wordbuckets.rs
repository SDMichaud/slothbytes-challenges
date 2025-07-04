//! # Word Buckets
//! 
//! ## Challenge Description
//! 
//! Write a function that divides a phrase into word buckets, with each bucket containing n or fewer characters. Only include full words inside each bucket.
//! 
//! - Spaces count as one character.
//! - Trim beginning and end spaces for each word bucket (see final example).
//! - If buckets are too small to hold a single word, return an empty list: []
//! - The final goal isn't to return just the words with a length equal (or lower) to the given n, but to return the entire given phrase bucketized (if possible).
//! 

/// Returns the bucketized version of the string if possible. Otherwise, returns an empty vector.
/// # Examples
///
/// ```
/// use slothbytes::split_into_buckets;
/// 
/// let s = "fairy dust coated the air";
/// let n = 20;
/// let mut expected = Vec::new();
/// expected.push(String::from("fairy dust coated"));
/// expected.push(String::from("the air"));
/// assert_eq!(split_into_buckets(s, n), expected);
/// 
/// // Cannot bucketize one of the words, returns empty vector:
/// let s = "One word is too freaking long";
/// let n = 4;
/// let expected: Vec<String> = Vec::new();
/// assert_eq!(split_into_buckets(s, n), expected)
/// ```
pub fn split_into_buckets(s: &str, n: i32) -> Vec<String> {
    // check if input is empty string
    if s.len() == 0 {
        let empty_vec: Vec<String> = Vec::new();
        return empty_vec;
    }

    let words = s.split_whitespace();
    let n = n as usize;
    let mut output: Vec<String> = Vec::new();
    let mut current_bucket = String::new();

    for word in words {
        // If we have no words in the current bucket, add the current word (unless it's too long then return an empty vector)
        if current_bucket.len() == 0 {
            if word.len() <= n {
                current_bucket.push_str(word);
            } else {
                let empty_vec: Vec<String> = Vec::new();
                return empty_vec;
            }
        // If there are words in the bucket, see if we can add a space plus the current word
        //   if not, the current bucket is cloned into the ouput vector and the current word is checked to see if it can fit into a new bucket as above
        //   Note: Is there a DRY way of doing this?
        } else {
            if word.len() + current_bucket.len() + 1 <= n {
                current_bucket.push_str(" ");
                current_bucket.push_str(word);
            } else {
                output.push(current_bucket.clone());
                current_bucket.clear();
                if word.len() <= n {
                    current_bucket.push_str(word);
                } else {
                    let empty_vec: Vec<String> = Vec::new();
                    return empty_vec;
                }
            }
        }
    }
    output.push(current_bucket);
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multi_words_split_into_buckets() {
        let s = "she sells sea shells by the sea";
        let n = 10;
        let mut expected = Vec::new();
        expected.push(String::from("she sells"));
        expected.push(String::from("sea shells"));
        expected.push(String::from("by the sea"));
        assert_eq!(expected, split_into_buckets(s, n));
    }

    #[test]
    fn single_words_split_into_buckets() {
        let s = "the mouse jumped over the cheese";
        let n = 7;
        let mut expected = Vec::new();
        expected.push(String::from("the"));
        expected.push(String::from("mouse"));
        expected.push(String::from("jumped"));
        expected.push(String::from("over"));
        expected.push(String::from("the"));
        expected.push(String::from("cheese"));
        assert_eq!(expected, split_into_buckets(s, n));
    }

    #[test]
    fn two_chunks_split_into_buckets() {
        let s = "fairy dust coated the air";
        let n = 20;
        let mut expected = Vec::new();
        expected.push(String::from("fairy dust coated"));
        expected.push(String::from("the air"));
        assert_eq!(expected, split_into_buckets(s, n));
    }

    #[test]
    fn single_letters_split_into_buckets() {
        let s = "a b c d e";
        let n = 2;
        let mut expected = Vec::new();
        expected.push(String::from("a"));
        expected.push(String::from("b"));
        expected.push(String::from("c"));
        expected.push(String::from("d"));
        expected.push(String::from("e"));
        assert_eq!(expected, split_into_buckets(s, n));
    }
    
    #[test]
    fn empty_string_split_into_buckets() {
        let s = "";
        let n = 2;
        let expected: Vec<String> = Vec::new();
        assert_eq!(expected, split_into_buckets(s, n));
    }

    #[test]
    fn cant_split_into_buckets() {
        let s = "One word is too freaking long";
        let n = 4;
        let expected: Vec<String> = Vec::new();
        assert_eq!(expected, split_into_buckets(s, n));
    }
}