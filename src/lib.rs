use rand::prelude::*;

lazy_static::lazy_static! {
    static ref MOCKS: Vec<&'static str> = vec![
        "Your mother was a hamster and your father smelt of elderberries.",
        "You have disturbing nasal hair.",
        "You fight like a diary farmer.",
        "Your pottery skills are lacking.",
        "You are a windows user.",
        "Your headphones cost less than $100.",
        "Do you even HKT?",
        "Your left knee is too circular."
    ];
}

/// Run the universal mocking framework.
///
/// # Examples
///
/// ```rust
/// use universal_mocking_library::mock;
/// println!("{}", mock());
/// ```
pub fn mock() -> &'static str {
    MOCKS.choose(&mut thread_rng()).unwrap()
}
