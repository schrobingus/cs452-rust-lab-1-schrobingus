
// PROBLEM 1

// Lifetime parameter and sum constraint had to be added for logic to work.
fn higher_sum<'a, T: std::ops::Add<Output = T> + Copy + Default + PartialOrd + std::iter::Sum<T>>(
    v1: &'a Vec<T>,
    v2: &'a Vec<T>
) -> &'a Vec<T> {
    let v1_sum: T = v1.iter().copied().sum();
    let v2_sum: T = v2.iter().copied().sum();
    if v1_sum > v2_sum { v1 } else { v2 }
}

// PROBLEM 2

fn remove_vowels(s: &str) -> String {
    s
        .replace("a", "")
        .replace("e", "")
        .replace("i", "")
        .replace("o", "")
        .replace("u", "")
}

// PROBLEM 3

struct TextPair<'a, 'b> {
    a_str: &'a str, b_str: &'b str
}

impl<'a, 'b> TextPair<'a, 'b> {
    fn longest(&self) -> &str {
        if self.a_str.len() > self.b_str.len() {
            self.a_str } else { self.b_str }
    }
}

// PROBLEM 4

fn double_evens(numbers: &mut Vec<i32>) {
    for x in numbers.iter_mut() {
        if *x % 2 == 0 { *x *= 2; }
    }
}

// PROBLEM 5

struct Book {
    title: String,
    author: String,
    year: u16,
    is_borrowed: bool
}

struct Library { books: Vec<Book> }

impl Book {
    fn new(title: String, author: String, year: u16) -> Self {
        Book {
            title: title,
            author: author,
            year: year,
            is_borrowed: false
        }
    }

    fn summary(&self) -> String {
        format!("Title: {},\nAuthor: {},\nYear: {},\nIs Borrowed: {}",
                self.title, self.author,
                self.year.to_string(),
                self.is_borrowed.to_string())
    }
}

impl Library {
    fn new() -> Self {
        Library {
            books: Vec::new()
        }
    }

    fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    fn checkout(&mut self, title: &str) -> Result<(), String> {
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

    fn return_book(&mut self, title: &str) -> Result<(), String> {
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

    fn available_books(&self) -> Vec<&Book> {
        self.books
            .iter()
            .filter(|book| !book.is_borrowed)
            .collect()
    }
}

// MAIN FUNCTION

fn main() {
    let vec1 = vec![1, 2, 3, 4];
    let vec2 = vec![5, 6];
    let result = higher_sum(&vec1, &vec2);
    println!("{:?}", result);

    let text = "Hello, World!";
    let result = remove_vowels(text);
    println!("{}", result);  // Should print "Hll, Wrld!"
    println!("{}", text);    // Original string should be unchanged

    let first = String::from("Short");
    {
        let second = String::from("This is a longer string");
        let pair = TextPair {
            a_str: &first,
            b_str: &second
        };
        println!("Longest: {}", pair.longest());
    }

    let mut nums = vec![1, 2, 3, 4, 5, 6];
    double_evens(&mut nums);
    println!("{:?}", nums);  // Should print [1, 4, 3, 8, 5, 12]
}
