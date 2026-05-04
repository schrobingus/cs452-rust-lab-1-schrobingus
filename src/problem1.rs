/// Returns the higher sum of two vectors.
/// Lifetime parameter and sum constraint had to be added for logic to work.
///
/// # Arguments
/// - `v1`: The first vector to compare.
/// - `v2`: The second vector to compare.
///
/// # Example
/// ```
/// let v1 = vec![1, 2, 3];      // sum = 6
/// let v2 = vec![2, 2, 2];      // sum = 6
/// let result = higher_sum(&v1, &v2);
/// assert_eq!(result, &v1);     // returns v1 (or v2 if equal)
/// ```
pub fn higher_sum<'a, T: std::ops::Add<Output = T> + Copy + Default + PartialOrd + std::iter::Sum<T>>(
    v1: &'a Vec<T>,
    v2: &'a Vec<T>
) -> &'a Vec<T> {
    let v1_sum: T = v1.iter().copied().sum();
    let v2_sum: T = v2.iter().copied().sum();
    if v1_sum > v2_sum { v1 } else { v2 }
}
