/// A text pair is represented here.
pub struct TextPair<'a, 'b> {
    /// There is both a string `a`, and a string `b`.
    pub a_str: &'a str, pub b_str: &'b str
}

impl<'a, 'b> TextPair<'a, 'b> {
    /// Returns the longest of its two strings.
    pub fn longest(&self) -> &str {
        // Compares it's own strings, so there are no arguments needed.
        if self.a_str.len() > self.b_str.len() {
            self.a_str } else { self.b_str }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_with_first_string_longer() {
        let pair = TextPair {
            a_str: "hello world",
            b_str: "hi",
        };
        assert_eq!(pair.longest(), "hello world");
    }

    #[test]
    fn test_longest_with_second_string_longer() {
        let pair = TextPair {
            a_str: "cat",
            b_str: "elephant",
        };
        assert_eq!(pair.longest(), "elephant");
    }

    #[test]
    fn test_longest_with_equal_length_strings() {
        let pair = TextPair {
            a_str: "rust",
            b_str: "code",
        };
        assert_eq!(pair.longest(), "code");  // returns b_str when equal
    }

    #[test]
    fn test_longest_with_empty_first_string() {
        let pair = TextPair {
            a_str: "",
            b_str: "hello",
        };
        assert_eq!(pair.longest(), "hello");
    }

    #[test]
    fn test_longest_with_empty_second_string() {
        let pair = TextPair {
            a_str: "world",
            b_str: "",
        };
        assert_eq!(pair.longest(), "world");
    }

    #[test]
    fn test_longest_with_both_empty_strings() {
        let pair = TextPair {
            a_str: "",
            b_str: "",
        };
        assert_eq!(pair.longest(), "");  // returns b_str when equal
    }

    #[test]
    fn test_longest_with_single_character_strings() {
        let pair = TextPair {
            a_str: "a",
            b_str: "ab",
        };
        assert_eq!(pair.longest(), "ab");
    }

    #[test]
    fn test_longest_with_strings_containing_spaces() {
        let pair = TextPair {
            a_str: "hello world",
            b_str: "hi there",
        };
        assert_eq!(pair.longest(), "hello world");
    }

    #[test]
    fn test_longest_with_special_characters() {
        let pair = TextPair {
            a_str: "!@#",
            b_str: "!@#$%",
        };
        assert_eq!(pair.longest(), "!@#$%");
    }

    #[test]
    fn test_longest_with_unicode_characters() {
        let pair = TextPair {
            a_str: "café",
            b_str: "naïve",
        };
        // Both have 5 bytes in UTF-8, so returns b_str (equal case)
        assert_eq!(pair.longest(), "naïve");
    }
}
