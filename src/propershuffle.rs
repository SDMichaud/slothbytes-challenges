//! # Proper Shuffle
//! 
//! ## Challenge Description
//! 
//! Given an array of 10 numbers, return whether or not the array is shuffled enough.
//! In this case, if 3 or more numbers appear consecutively (ascending or descending), return false 

enum Direction {
    Increasing,
    Decreasing,
    Neutral,
}

/// Returns false if there are at least three consecutive one point increases or decreases in a list. (ex. 1, 2, 3 or 6, 5, 4)
/// # Examples
///
/// ```
/// use slothbytes::is_shuffled_well;
/// 
/// // false: 1, 2, 3
/// let numbers = vec![10, 9, 1, 2, 3, 8, 7];
/// assert!(!is_shuffled_well(&numbers));
/// 
/// // true: increments and decrements are by more than 1
/// let numbers = vec![1, 3, 5, 7, 9, 2, 4, 6, 8, 10];
/// assert!(is_shuffled_well(&numbers));
/// ```
pub fn is_shuffled_well(numbers: &Vec<i32>) -> bool {
    if numbers.len() == 0 {
        return true
    }
    let max_consecutive_count = 2; // 9 followed by 8 equals 1 consecutive count. 9, 8, 7 would be 2 consecutive counts.
    let mut direction = Direction::Neutral;
    let mut prev_number = numbers.first().expect("Expected there to be at least one number in list.");
    let mut consecutive_count = 0;
    for i in 1..numbers.len() {
        match direction {
            Direction::Neutral => {
                if numbers[i] == prev_number + 1 {
                    direction = Direction::Increasing;
                    consecutive_count = 1;
                } else if numbers[i] == prev_number - 1 {
                    direction = Direction::Decreasing;
                    consecutive_count = 1;
                }
            }
            Direction::Increasing => {
                if numbers[i] == prev_number + 1 {
                    consecutive_count += 1;
                } else if numbers[i] == prev_number - 1 {
                    direction = Direction::Decreasing;
                    consecutive_count = 1;
                } else {
                    direction = Direction::Neutral;
                    consecutive_count = 0;
                }
            }
            Direction::Decreasing => {
                if numbers[i] == prev_number + 1 {
                    direction = Direction::Increasing;
                    consecutive_count = 1;
                } else if numbers[i] == prev_number - 1 {
                    consecutive_count += 1;
                } else {
                    direction = Direction::Neutral;
                    consecutive_count = 0;
                }
            }
        }
        if consecutive_count >= max_consecutive_count {
            return false;
        }
        prev_number = &numbers[i];
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn consecutive_increase() {
        let numbers = vec![10, 9, 1, 2, 3, 8, 7];
        assert!(!is_shuffled_well(&numbers));
    }

    #[test]
    fn consecutive_decrease() {
        let numbers = vec![1, 3, 7, 6, 5, 2];
        assert!(!is_shuffled_well(&numbers));
    }

    #[test]
    fn flip_flop() {
        let numbers = vec![1, 2, 1, 7, 8, 7, 5];
        assert!(is_shuffled_well(&numbers));
    }

    #[test]
    fn no_consecutive() {
        let numbers = vec![1, 5, 3, 8, 10, 2, 7, 6, 4, 9];
        assert!(is_shuffled_well(&numbers));
    }

    #[test]
    fn no_consecutive_by_twos() {
        let numbers = vec![1, 3, 5, 7, 9, 2, 4, 6, 8, 10];
        assert!(is_shuffled_well(&numbers));
    }

    #[test]
    fn empty_list() {
        let numbers: Vec<i32> = Vec::new();
        assert!(is_shuffled_well(&numbers));
    }
}