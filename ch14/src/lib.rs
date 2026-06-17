//! # Chapter 14: More about Cargo and Crates
//! This chapter covers more about Cargo and Crates, including how to create a library crate, how
//! to publish a crate to crates.io, and how to use Cargo features.

pub use self::kinds::{PrimaryColor, SecondaryColor};
pub use self::utils::mix;

pub mod kinds {
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::{PrimaryColor, SecondaryColor};

    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {

    }
}


/// Add two numbers together.
/// # Examples
/// ```
/// let result = ch14::add(2, 3);
/// assert_eq!(result, 5);
/// ``` 
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
