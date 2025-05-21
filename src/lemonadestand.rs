//! # Lemonade Stand
//! 
//! ## Challenge Description
//! 
//! At a lemonade stand, each lemonade costs $5.
//! Customers are standing in a queue to buy from you, and order one at a time (in the order specified by bills).
//! Each customer will only buy one lemonade and pay with either a $5, $10, or $20 bill.
//! You need to give each customer the right amount of change so that they end up paying $5.
//! - For $5 bills: No change needed
//! - For $10 bills: You need to give $5 back
//! - For $20 bills: You need to give $15 back (best using one $10 and one $5, or three $5 bills)
//! 
//! Return `true` if and only if you can provide every customer with correct change.

/// Takes a vector of `bills` and returns true if and only if every customer can be provided with correct change.
/// 
/// # Examples
/// 
/// ```
/// let bills: Vec<i32> = vec![5, 5, 5, 10, 20];
/// assert!(slothbytes::lemonade(bills));
/// 
/// let bad_bills: Vec<i32> = vec![5, 5, 10, 10, 20];
/// assert!(!slothbytes::lemonade(bad_bills));
/// ```
pub fn lemonade(bills: Vec<i32>) -> bool {
    let mut change = Bills::new();
    for n in 0..bills.len() {
        match change.collect(bills[n]) {
            Ok(_) => continue,
            Err(_) => return false
        }
    }
    true
}

struct Bills {
    pub fives: i32,
    pub tens: i32,
    pub twenties: i32,
}

impl Bills {
    fn collect(&mut self, bill: i32) -> Result<(), &str> {
        if bill == 5 {
            self.fives += 1;
            Ok(())
        } else if bill == 10 {
            self.tens += 1;
            if self.fives >= 1 {
                self.fives -= 1;
                Ok(())
            } else {
                Err("Cannot make change for a 10.")
            }
        } else {
            self.twenties += 1;
            if self.tens >= 1 && self.fives >= 1 {
                self.tens -= 1;
                self.fives -= 1;
                Ok(())
            } else if self.fives >= 3 {
                self.fives -= 3;
                Ok(())
            } else {
                Err("Cannot make change for a 20.")
            }
        }
    }

    fn new() -> Self {
        Self { fives: 0, tens: 0, twenties: 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn have_change_for_20() {
        let payments = vec![5, 5, 5, 10, 20];
        assert_eq!(lemonade(payments), true);
    }

    #[test]
    fn no_change_for_20() {
        let payments = vec![5, 5, 10, 10, 20];
        assert_eq!(lemonade(payments), false);
    }

    #[test]
    fn no_change_for_10() {
        let payments = vec![10, 10];
        assert_eq!(lemonade(payments), false);
    }

    #[test]
    fn change_for_10() {
        let payments = vec![5, 5, 10];
        assert_eq!(lemonade(payments), true);
    }
 
    #[test]
    fn change_for_20_with_only_fives() {
        let payments = vec![5, 5, 5, 20];
        assert_eq!(lemonade(payments), true);
    }
}