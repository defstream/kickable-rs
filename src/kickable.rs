/// Returns true if the input supplied is kickable.
///
/// # Arguments
///
/// * `input` - A string to validate for kick-ability.
///
/// # Examples
///
#[allow(rustdoc::bare_urls)]
#[cfg_attr(not(feature = "derive"), doc = " ```ignore")]
#[doc = include_str!("../examples/cargo-example.rs")]
pub fn validate(input: &str) -> bool {
    if input.to_lowercase() == "it" {
        return true;
    }
    false
}
