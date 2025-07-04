//! # Actual Memory
//! 
//! ## Challenge Description
//! 
//! Create a function that takes the memory size (ms) as an argument and returns the actual memory size.
//! 
//! - The actual storage loss on a USB device is 7% of the overall memory size!  
//! - If the actual memory size was greater than 1 GB, round your result to two decimal places.  
//! - If the memory size after adjustment is smaller then 1 GB, return the result in MB.  
//! - For the purposes of this challenge, there are 1000 MB in a Gigabyte.


/// Returns the actual memory of a USB drive, which is the original memory less 7%.  
/// GB values will be returned to 2 decimal places. MB values are rounded. Values less than 1GB will be returned in MB.  
/// This function assumes there are 1000MB per GB.
/// 
/// # Examples
/// 
/// ```
/// use slothbytes::actual_memory;
/// 
/// let ms = "32GB";
/// let expected = "29.76GB";
/// assert_eq!(actual_memory(ms), expected);
/// 
/// let ms = "512MB";
/// let expected = "476MB";
/// assert_eq!(actual_memory(ms), expected);
/// 
/// let ms = "1GB";
/// let expected = "930MB";
/// assert_eq!(actual_memory(ms), expected);
/// 
/// ```
pub fn actual_memory(ms: &str) -> String {
    let loss = 0.93; // 0.93 = 7% loss
    let suffix_len = ms.len()-2;
    let suffix: String = ms.chars().skip(suffix_len).take(2).collect();
    let number: i32 = ms.chars().take(suffix_len).collect::<String>().parse().expect("Expected to be able to parse input.");

    let actual_size = loss * number as f64;
    if suffix == "MB" { // If in MB, return MB without decimal
        return format!("{actual_size:.0}MB");
    } else if actual_size < 1.0 { // If result is less than 1 GB, convert to MB and return without decimal
        return format!("{:.0}MB", actual_size * 1000.0);
    } else { // Result is > 1GB, return to 2 decimal places
        return format!("{actual_size:.2}GB");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(non_snake_case)]
    fn test_32GB() {
        let ms = "32GB";
        let expected = "29.76GB";
        assert_eq!(actual_memory(ms), expected);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_2GB() {
        let ms = "2GB";
        let expected = "1.86GB";
        assert_eq!(actual_memory(ms), expected);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_512MB() {
        let ms = "512MB";
        let expected = "476MB";
        assert_eq!(actual_memory(ms), expected);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_1GB() {
        let ms = "1GB";
        let expected = "930MB";
        assert_eq!(actual_memory(ms), expected);
    }
}