/// This represents a book.
pub struct Book {
    /// The attributes of a book include a title, an author,
    /// the release year, and whether or not it was borrowed.
    title: String,
    author: String,
    year: u16,
    is_borrowed: bool
}

/// This represents a library, with the only attributes
/// being the books it contains.
pub struct Library { books: Vec<Book> }

impl Book {
    /// Constructor to define a new book.
    /// All arguments are simply the attributes from before,
    /// excluding whether or not it is borrowed. By default,
    /// the book is not borrowed.
    pub fn new(title: String, author: String, year: u16) -> Self {
        Book {
            title: title,
            author: author,
            year: year,
            is_borrowed: false
        }
    }

    /// Returns a string summary of the book itself.
    pub fn summary(&self) -> String {
        format!("Title: {},\nAuthor: {},\nYear: {},\nIs Borrowed: {}",
                self.title, self.author,
                self.year.to_string(),
                self.is_borrowed.to_string())
    }
}

impl Library {
    /// Constructor to define a new library, instantiating a
    /// vector for the books.
    pub fn new() -> Self {
        Library {
            books: Vec::new()
        }
    }

    /// Adds a new book to the library.
    ///
    /// # Arguments
    /// - `book`: The book added to the library.
    ///
    /// # Example
    /// ```
    /// let mut library = Library::new();
    /// let book = Book::new("1984".to_string(), "George Orwell".to_string(), 1949);
    /// library.add_book(book);
    /// assert_eq!(library.available_books().len(), 1);
    /// ```
    pub fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    /// Checks out a book from the library.
    /// Returns whether or not the book was borrowed successfully.
    ///
    /// # Arguments
    /// - `title`: The title of the book to be checked out.
    ///
    /// # Example
    /// ```
    /// let mut library = Library::new();
    /// library.add_book(Book::new("1984".to_string(), "George Orwell".to_string(), 1949));
    /// assert!(library.checkout("1984").is_ok());
    /// assert!(library.checkout("1984").is_err()); // Already borrowed
    /// assert!(library.checkout("Missing Book").is_err()); // Not found
    /// ```
    pub fn checkout(&mut self, title: &str) -> Result<(), String> {
        for book in &mut self.books {
            if book.title == title {
                if book.is_borrowed {
                    return Err(format!("'{}' is already borrowed.", title));
                }
                book.is_borrowed = true;
                return Ok(());
            }
        }
        Err(format!("'{}' not found in library.", title))
    }

    /// Send back a checked out book from the library
    /// Returns whether or not the book was returned successfully.
    ///
    /// # Arguments
    /// - `title`: The title of the book to be returned.
    ///
    /// # Example
    /// ```
    /// let mut library = Library::new();
    /// library.add_book(Book::new("1984".to_string(), "George Orwell".to_string(), 1949));
    /// library.checkout("1984").unwrap();
    /// assert!(library.return_book("1984").is_ok());
    /// assert!(library.return_book("1984").is_err()); // Not borrowed
    /// ```
    pub fn return_book(&mut self, title: &str) -> Result<(), String> {
        for book in &mut self.books {
            if book.title == title {
                if !book.is_borrowed {
                    return Err(format!("'{}' is not already borrowed.", title));
                }
                book.is_borrowed = false;
                return Ok(());
            }
        }
        Err(format!("'{}' not found in library.", title))
    }

    /// Returns a vector of books that are available in the library.
    pub fn available_books(&self) -> Vec<&Book> {
        self.books
            .iter()
            .filter(|book| !book.is_borrowed)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_book_defaults() {
        let b = Book::new("A".into(), "B".into(), 2000);
        assert!(!b.is_borrowed);
    }

    #[test]
    fn test_book_summary_contains_title() {
        let b = Book::new("A".into(), "B".into(), 2000);
        assert!(b.summary().contains("A"));
    }

    #[test]
    fn test_book_summary_contains_author() {
        let b = Book::new("A".into(), "B".into(), 2000);
        assert!(b.summary().contains("B"));
    }

    #[test]
    fn test_book_summary_contains_year() {
        let b = Book::new("A".into(), "B".into(), 2000);
        assert!(b.summary().contains("2000"));
    }

    #[test]
    fn test_library_new_empty() {
        let l = Library::new();
        assert_eq!(l.books.len(), 0);
    }

    #[test]
    fn test_add_book_increases_count() {
        let mut l = Library::new();
        l.add_book(Book::new("A".into(), "B".into(), 1));
        assert_eq!(l.books.len(), 1);
    }

    #[test]
    fn test_available_books_initial() {
        let mut l = Library::new();
        l.add_book(Book::new("A".into(), "B".into(), 1));
        assert_eq!(l.available_books().len(), 1);
    }

    #[test]
    fn test_checkout_success() {
        let mut l = Library::new();
        l.add_book(Book::new("A".into(), "B".into(), 1));
        assert!(l.checkout("A").is_ok());
    }

    #[test]
    fn test_checkout_marks_borrowed() {
        let mut l = Library::new();
        l.add_book(Book::new("A".into(), "B".into(), 1));
        l.checkout("A").unwrap();
        assert!(l.books[0].is_borrowed);
    }

    #[test]
    fn test_checkout_twice_fails() {
        let mut l = Library::new();
        l.add_book(Book::new("A".into(), "B".into(), 1));
        l.checkout("A").unwrap();
        assert!(l.checkout("A").is_err());
    }

    #[test]
    fn test_checkout_missing() {
        let mut l = Library::new();
        assert!(l.checkout("X").is_err());
    }

    #[test]
    fn test_return_success() {
        let mut l = Library::new();
        l.add_book(Book::new("A".into(), "B".into(), 1));
        l.checkout("A").unwrap();
        assert!(l.return_book("A").is_ok());
    }

    #[test]
    fn test_return_marks_not_borrowed() {
        let mut l = Library::new();
        l.add_book(Book::new("A".into(), "B".into(), 1));
        l.checkout("A").unwrap();
        l.return_book("A").unwrap();
        assert!(!l.books[0].is_borrowed);
    }

    #[test]
    fn test_return_not_borrowed_fails() {
        let mut l = Library::new();
        l.add_book(Book::new("A".into(), "B".into(), 1));
        assert!(l.return_book("A").is_err());
    }

    #[test]
    fn test_return_missing() {
        let mut l = Library::new();
        assert!(l.return_book("X").is_err());
    }

    #[test]
    fn test_available_after_checkout() {
        let mut l = Library::new();
        l.add_book(Book::new("A".into(), "B".into(), 1));
        l.checkout("A").unwrap();
        assert_eq!(l.available_books().len(), 0);
    }

    #[test]
    fn test_available_after_return() {
        let mut l = Library::new();
        l.add_book(Book::new("A".into(), "B".into(), 1));
        l.checkout("A").unwrap();
        l.return_book("A").unwrap();
        assert_eq!(l.available_books().len(), 1);
    }

    #[test]
    fn test_multiple_books_available() {
        let mut l = Library::new();
        l.add_book(Book::new("A".into(), "B".into(), 1));
        l.add_book(Book::new("C".into(), "D".into(), 2));
        assert_eq!(l.available_books().len(), 2);
    }

    #[test]
    fn test_multiple_checkout() {
        let mut l = Library::new();
        l.add_book(Book::new("A".into(), "B".into(), 1));
        l.add_book(Book::new("C".into(), "D".into(), 2));
        l.checkout("A").unwrap();
        l.checkout("C").unwrap();
        assert_eq!(l.available_books().len(), 0);
    }

    #[test]
    fn test_partial_available() {
        let mut l = Library::new();
        l.add_book(Book::new("A".into(), "B".into(), 1));
        l.add_book(Book::new("C".into(), "D".into(), 2));
        l.checkout("A").unwrap();
        assert_eq!(l.available_books().len(), 1);
    }

    #[test]
    fn test_titles_match_exact() {
        let mut l = Library::new();
        l.add_book(Book::new("A".into(), "B".into(), 1));
        assert!(l.checkout("a").is_err());
    }

    #[test]
    fn test_return_then_checkout_again() {
        let mut l = Library::new();
        l.add_book(Book::new("A".into(), "B".into(), 1));
        l.checkout("A").unwrap();
        l.return_book("A").unwrap();
        assert!(l.checkout("A").is_ok());
    }

    #[test]
    fn test_many_books() {
        let mut l = Library::new();
        for i in 0..10 {
            l.add_book(Book::new(format!("{}", i), "B".into(), 1));
        }
        assert_eq!(l.available_books().len(), 10);
    }

    #[test]
    fn test_checkout_last_book() {
        let mut l = Library::new();
        l.add_book(Book::new("A".into(), "B".into(), 1));
        l.add_book(Book::new("Z".into(), "B".into(), 1));
        assert!(l.checkout("Z").is_ok());
    }

    #[test]
    fn test_return_last_book() {
        let mut l = Library::new();
        l.add_book(Book::new("A".into(), "B".into(), 1));
        l.add_book(Book::new("Z".into(), "B".into(), 1));
        l.checkout("Z").unwrap();
        assert!(l.return_book("Z").is_ok());
    }

    #[test]
    fn test_checkout_does_not_affect_others() {
        let mut l = Library::new();
        l.add_book(Book::new("A".into(), "B".into(), 1));
        l.add_book(Book::new("C".into(), "D".into(), 2));
        l.checkout("A").unwrap();
        assert!(!l.books[1].is_borrowed);
    }

    #[test]
    fn test_return_does_not_affect_others() {
        let mut l = Library::new();
        l.add_book(Book::new("A".into(), "B".into(), 1));
        l.add_book(Book::new("C".into(), "D".into(), 2));
        l.checkout("A").unwrap();
        l.return_book("A").unwrap();
        assert!(!l.books[1].is_borrowed);
    }

    #[test]
    fn test_summary_borrowed_true() {
        let mut b = Book::new("A".into(), "B".into(), 1);
        b.is_borrowed = true;
        assert!(b.summary().contains("true"));
    }

    #[test]
    fn test_summary_borrowed_false() {
        let b = Book::new("A".into(), "B".into(), 1);
        assert!(b.summary().contains("false"));
    }

    #[test]
    fn test_empty_library_available() {
        let l = Library::new();
        assert_eq!(l.available_books().len(), 0);
    }

    #[test]
    fn test_checkout_after_multiple_returns() {
        let mut l = Library::new();
        l.add_book(Book::new("A".into(), "B".into(), 1));
        l.checkout("A").unwrap();
        l.return_book("A").unwrap();
        l.return_book("A").err();
        assert!(l.checkout("A").is_ok());
    }

    #[test]
    fn test_return_after_failed_checkout() {
        let mut l = Library::new();
        l.add_book(Book::new("A".into(), "B".into(), 1));
        l.checkout("A").unwrap();
        l.checkout("A").err();
        assert!(l.return_book("A").is_ok());
    }

    #[test]
    fn test_add_multiple_then_checkout_one() {
        let mut l = Library::new();
        for i in 0..5 {
            l.add_book(Book::new(format!("{}", i), "B".into(), 1));
        }
        l.checkout("3").unwrap();
        assert_eq!(l.available_books().len(), 4);
    }

    #[test]
    fn test_checkout_nonexistent_after_adds() {
        let mut l = Library::new();
        for i in 0..5 {
            l.add_book(Book::new(format!("{}", i), "B".into(), 1));
        }
        assert!(l.checkout("10").is_err());
    }
}
