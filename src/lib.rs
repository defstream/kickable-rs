// Licensed under the MIT license
// (see LICENSE or <http://opensource.org/licenses/MIT>) All files in the project carrying such
// notice may not be copied, modified, or distributed except according to those terms.

//! > **kickable** is a package created to answer the age old question... "_Can I Kick It?_"
//!
//! Quick Links:
//! - Can I Kick It [music video](https://www.youtube.com/watch?v=O3pyCGnZzYA)
//! ## Example
//!
//! Run
//! ```console
//! $ cargo add kickable
//! ```
//! Then use kickable in your code:
#![allow(rustdoc::bare_urls)]
#![doc = include_str!("../examples/kickable.rs")]
pub use anyhow::Result;
pub mod args;
pub mod config;
pub mod i18n;

/// Returns true if the input supplied is kickable.
///
/// # Arguments
///
/// * `input` - A string to validate for kick-ability.
///
pub fn validate(input: &str) -> bool {
    if input.trim().to_lowercase() == "it" {
        return true;
    }
    false
}

/// Returns true if the input supplied is kickable.
///
/// # Arguments
///
/// * `input` - A string to validate for kick-ability.
/// * `items` - A Vec<String> containing items to consider kickable
///
pub fn validate_amongst(input: &str, items: Vec<String>) -> bool {
    for item in items.into_iter() {
        if input.trim().to_lowercase() == item.trim().to_lowercase() {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_should_pass_it() {
        let result = validate("it");
        assert_eq!(result, true);
    }
    #[test]
    fn validate_should_pass_it_upper() {
        let result = validate("IT");
        assert_eq!(result, true);
    }
    #[test]
    fn validate_should_pass_it_padded() {
        let result = validate(" it ");
        assert_eq!(result, true);
    }
    #[test]
    fn validate_should_fail_empty() {
        let result = validate("");
        assert_eq!(result, false);
    }
    #[test]
    fn validate_should_fail_other() {
        let result = validate("other");
        assert_eq!(result, false);
    }

    #[test]
    fn validate_amongst_should_pass_it() {
        let result = validate_amongst("it", vec!["other".to_string(), "it".to_string()]);
        assert_eq!(result, true);
    }
    #[test]
    fn validate_amongst_should_pass_it_upper() {
        let result = validate_amongst("IT", vec!["it".to_string()]);
        assert_eq!(result, true);
    }
    #[test]
    fn validate_amongst_should_pass_it_padded() {
        let result = validate_amongst(" it ", vec!["it".to_string()]);
        assert_eq!(result, true);
    }
    #[test]
    fn validate_amongst_should_fail_empty() {
        let result = validate_amongst("", vec!["it".to_string()]);
        assert_eq!(result, false);
    }
    #[test]
    fn validate_amongst_should_fail_other() {
        let result = validate_amongst("other", vec!["it".to_string()]);
        assert_eq!(result, false);
    }
}
