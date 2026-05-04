mod problem1;
mod problem2;
mod problem3;
mod problem4;
mod problem5;

use problem1::higher_sum;
use problem2::remove_vowels;
use problem3::TextPair;
use problem4::double_evens;

fn main() {
    // Problem 1 Example
    let vec1 = vec![1, 2, 3, 4];
    let vec2 = vec![5, 6];
    let result = higher_sum(&vec1, &vec2);
    println!("{:?}", result);

    // Problem 2 Example
    let text = "Hello, World!";
    let result = remove_vowels(text);
    println!("{}", result);  // Should print "Hll, Wrld!"
    println!("{}", text);    // Original string should be unchanged

    // Problem 3 Example
    let first = String::from("Short");
    {
        let second = String::from("This is a longer string");
        let pair = TextPair {
            a_str: &first,
            b_str: &second
        };
        println!("Longest: {}", pair.longest());
    }

    // Problem 4 Example
    let mut nums = vec![1, 2, 3, 4, 5, 6];
    double_evens(&mut nums);
    println!("{:?}", nums);  // Should print [1, 4, 3, 8, 5, 12]
}
