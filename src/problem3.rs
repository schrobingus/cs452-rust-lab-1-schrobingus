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
