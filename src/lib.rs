//! # Slothbytes
//! 
//! A library of code used to solve programming challenges issued by the [Sloth Bytes newsletter](https://slothbytes.beehiiv.com)!
pub mod bridgeshuffle;
pub use bridgeshuffle::bridgeshuffle;

pub mod lemonadestand;
pub use lemonadestand::lemonade;

pub mod spacemessage;
pub use spacemessage::spacemessage;

pub mod stickykeys;
pub use stickykeys::is_long_pressed;

pub mod nexthappyyear;
pub use nexthappyyear::happy_year;

pub mod wordbuckets;
pub use wordbuckets::split_into_buckets;

pub mod propershuffle;
pub use propershuffle::is_shuffled_well;

pub mod noyelling;
pub use noyelling::no_yelling;

pub mod timetoeat;
pub use timetoeat::time_to_eat;

pub mod removevirus;
pub use removevirus::remove_virus;

pub mod actualmemory;
pub use actualmemory::actual_memory;