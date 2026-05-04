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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_vowels_from_hello() {
        let result = remove_vowels("hello");
        assert_eq!(result, "hll");
    }

    #[test]
    fn test_remove_all_vowels() {
        let result = remove_vowels("aeiou");
        assert_eq!(result, "");
    }

    #[test]
    fn test_remove_vowels_from_rust() {
        let result = remove_vowels("rust");
        assert_eq!(result, "rst");
    }

    #[test]
    fn test_string_with_no_vowels() {
        let result = remove_vowels("xyz");
        assert_eq!(result, "xyz");
    }

    #[test]
    fn test_empty_string() {
        let result = remove_vowels("");
        assert_eq!(result, "");
    }

    #[test]
    fn test_mixed_case_vowels() {
        let result = remove_vowels("HeLLo WoRLd");
        assert_eq!(result, "HLL WRLd");
    }

    #[test]
    fn test_string_with_numbers_and_symbols() {
        let result = remove_vowels("hello123!@#world");
        assert_eq!(result, "hll123!@#wrld");
    }

    #[test]
    fn test_multiple_consecutive_vowels() {
        let result = remove_vowels("beautiful");
        assert_eq!(result, "btfl");
    }

    #[test]
    fn test_single_vowel() {
        let result = remove_vowels("a");
        assert_eq!(result, "");
    }
}
