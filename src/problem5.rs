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
