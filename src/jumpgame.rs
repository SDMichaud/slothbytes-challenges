//! # Jump Game
//! 
//! ## Challenge Description
//! 
//! You are given an integer array nums. You are initially positioned at the array's first index, and each element in the array represents your maximum jump length at that position.
//! Return true if you can reach the last index, or false otherwise.
//! 
//! Example 1:
//! Input: nums = \[2,3,1,1,4\]
//! Output: true
//! Explanation: Jump 1 step from index 0 to 1, then 3 steps to the last index.
//! 
//! Example 2:
//! Input: nums = \[3,2,1,0,4\]
//! Output: false
//! Explanation: You will always arrive at index 3 no matter what. Its maximum jump length is 0, which makes it impossible to reach the last index.

/// Returns true if you can jump from index 0 to the last index of the vector. The current index is the maximum jump size.
/// 
/// # Examples
/// 
/// ```
/// use slothbytes::can_jump;
/// 
/// // Jump from indicies 0, 2, 7, 9
/// let nums = vec![5,5,5,0,0,1,0,2,0,0];
/// assert!(can_jump(&nums));
/// 
/// // No way to reach index 4
/// let nums = vec![3,2,1,0,0];
/// assert!(!can_jump(&nums));
/// ```
pub fn can_jump(nums: &[i32]) -> bool {
    let len = nums.len();
    if len == 0 {
        return false;
    }
    if len == 1 || nums[0] as usize >= len {
        return true;
    } else {
        for jump_size in (1..nums[0]+1).rev() {
            if can_jump(&nums[jump_size as usize..len]) {
                return true;
            }
        }
        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn true_jump() {
        let nums = vec![2,3,1,1,4];
        assert!(can_jump(&nums))
    }

    #[test]
    fn bigger_jump() {
        let nums = vec![5,5,5,0,0,1,0,2,0,0];
        assert!(can_jump(&nums))
    }

    #[test]
    fn false_jump() {
        let nums = vec![3,2,1,0,4];
        assert!(!can_jump(&nums))
    }

    #[test]
    fn empty_list() {
        let nums = Vec::new();
        assert!(!can_jump(&nums))
    }
}