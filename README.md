# cs452-rust-lab1

Create a Rust module that implements each of the following exercises.  You will need to use Cargo, see <https://doc.rust-lang.org/cargo/guide/index.html> for more details.

Also be sure to implement proper unit tests.   Proper unit tests cover all code paths and include edge cases.   

I should be able to run all the unit tests by running `cargo test`.   Note, main functions are not required, they are just shown below as an example of using the functions you are writing.  Feel free to have main functions if you like.   Just understand that there should be only one main function per module and your grade is based on the non main functions, unit tests and docstrings. 

Also be sure to write proper docstrings.  See <https://doc.rust-lang.org/rust-by-example/meta/doc.html> for information on Rust docstrings. (Yes I know that the code below does not have proper docstrings)

## Exercise 1: Borrowing Basics
Create a function that takes two vectors by reference and returns the vector with the higher sum without taking ownership.

```rust
// Implement this function
fn higher_sum<T: std::ops::Add<Output = T> + Copy + Default + PartialOrd>(
    v1: &Vec<T>, 
    v2: &Vec<T>
) -> &Vec<T> {
    // Your code here
}

// Example usage
fn main() {
    let vec1 = vec![1, 2, 3, 4];
    let vec2 = vec![5, 6];
    let result = higher_sum(&vec1, &vec2);
    println!("{:?}", result);  // Should print [5, 6]
}
```

Note, the function signature needs a bit of explaining.

- `fn higher_sum<T: ...>`: This defines a generic function named `higher_sum` that works with a type parameter `T`.
- The type parameter `T` has several trait bounds (constraints):
    - `std::ops::Add<Output = T>`: `T` must implement the Add trait with the output type also being `T`. This means you can add values of type `T` together and get a result of the same type.
    - `Copy`: `T` must implement the `Copy` trait, which means it can be copied by simply copying bits (no need for deep copying).
    - `Default`: `T` must implement the `Default` trait, which provides a default value for the type (likely used to initialize a sum at zero).
    - `PartialOrd`: `T` must implement `PartialOrd`, which allows values to be compared with <, >, etc. This is needed to determine which sum is greater.
 

#### These template paramaters are a way of letting the generic type be any numeric type.  


## Exercise 2: String Manipulation with Borrowing
Create a function that takes a string slice and returns a new string with all vowels removed.

```rust
fn remove_vowels(s: &str) -> String {
    // Your code here
}

// Example usage
fn main() {
    let text = "Hello, World!";
    let result = remove_vowels(text);
    println!("{}", result);  // Should print "Hll, Wrld!"
    println!("{}", text);    // Original string should be unchanged
}
```


## Exercise 3: Struct with Lifetimes
Create a struct that holds references to two strings with different lifetimes.

```rust
// Define the struct with appropriate lifetimes
struct TextPair<'a, 'b> {
    // Your code here
}

// Implement a method that returns the longer string
impl<'a, 'b> TextPair<'a, 'b> {
    fn longest(&self) -> &str {
        // Your code here
    }
}

fn main() {
    let first = String::from("Short");
    {
        let second = String::from("This is a longer string");
        let pair = TextPair {
            // Initialize your struct here
        };
        println!("Longest: {}", pair.longest());
    }
    // second is out of scope here, but first is still valid
}
```

## Excercise 4  Advanced Borrowing - Mutable References
Create a function that takes a mutable reference to a vector and doubles every even number.

```rust
// Implement this function
fn double_evens(numbers: &mut Vec<i32>) {
    // Your code here
}

fn main() {
    let mut nums = vec![1, 2, 3, 4, 5, 6];
    double_evens(&mut nums);
    println!("{:?}", nums);  // Should print [1, 4, 3, 8, 5, 12]
}
```


## Exercise 5: Struct Implementation - Book Library
Create a Book struct and a Library struct that manages a collection of books.

```rust
struct Book {
    // Define fields: title, author, year, is_borrowed
}

struct Library {
    // Store a collection of books
}

impl Book {
    fn new(title: String, author: String, year: u16) -> Self {
        // Initialize a new book (not borrowed by default)
    }

    fn summary(&self) -> String {
        // Return a formatted summary of the book
    }
}

impl Library {
    fn new() -> Self {
        // Initialize an empty library
    }

    fn add_book(&mut self, book: Book) {
        // Add a book to the library
    }

    fn checkout(&mut self, title: &str) -> Result<(), String> {
        // Mark a book as borrowed if it exists and is available
        // Return an error message if not found or already borrowed
    }

    fn return_book(&mut self, title: &str) -> Result<(), String> {
        // Mark a book as not borrowed
        // Return an error if book is not found or not borrowed
    }

    fn available_books(&self) -> Vec<&Book> {
        // Return references to all available books
    }
}
```

