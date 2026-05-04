/// Doubles every even number in a mutable vector.
///
/// Arguments:
/// - `numbers`: The mutable vector to be modified.
///
/// # Example
/// ```
/// let mut nums = vec![1, 2, 3, 4, 5, 6];
/// double_evens(&mut nums);
/// assert_eq!(nums, vec![1, 4, 3, 8, 5, 12]);
///
/// let mut nums = vec![2, 4, 6];
/// double_evens(&mut nums);
/// assert_eq!(nums, vec![4, 8, 12]);
/// ```
pub fn double_evens(numbers: &mut Vec<i32>) {
    for x in numbers.iter_mut() {
        if *x % 2 == 0 { *x *= 2; }
    }
}
