/// Removes vowels (A, E, I, O, and U) in a given string.
///
/// # Arguments
/// - `s`: The input string to remove vowels from.
///
/// # Example
/// ```
/// let result = remove_vowels("hello");
/// assert_eq!(result, "hll");
///
/// let result = remove_vowels("aeiou");
/// assert_eq!(result, "");
///
/// let result = remove_vowels("rust");
/// assert_eq!(result, "rst");
/// ```
pub fn remove_vowels(s: &str) -> String {
    s
        .replace("a", "")
        .replace("e", "")
        .replace("i", "")
        .replace("o", "")
        .replace("u", "")
}
