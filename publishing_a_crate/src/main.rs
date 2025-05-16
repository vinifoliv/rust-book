fn main() {
    println!("Hello, world!");
}

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = publishing_a_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
fn add_one(x: i32) -> i32 {
    x + 1
}
