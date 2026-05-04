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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_vector_has_higher_sum() {
        let v1 = vec![10, 20, 30];  // sum = 60
        let v2 = vec![5, 5, 5];     // sum = 15
        let result = higher_sum(&v1, &v2);
        assert_eq!(result, &v1);
    }

    #[test]
    fn test_second_vector_has_higher_sum() {
        let v1 = vec![1, 2, 3];     // sum = 6
        let v2 = vec![10, 10, 10];  // sum = 30
        let result = higher_sum(&v1, &v2);
        assert_eq!(result, &v2);
    }

    #[test]
    fn test_equal_sums_returns_second_vector() {
        let v1 = vec![1, 2, 3];     // sum = 6
        let v2 = vec![2, 2, 2];     // sum = 6
        let result = higher_sum(&v1, &v2);
        assert_eq!(result, &v2);
    }

    #[test]
    fn test_single_element_vectors() {
        let v1 = vec![5];
        let v2 = vec![10];
        let result = higher_sum(&v1, &v2);
        assert_eq!(result, &v2);
    }

    #[test]
    fn test_empty_vectors() {
        let v1: Vec<i32> = vec![];
        let v2: Vec<i32> = vec![];
        let result = higher_sum(&v1, &v2);
        assert_eq!(result, &v2);  // equal sums, returns second
    }

    #[test]
    fn test_one_empty_one_nonempty() {
        let v1: Vec<i32> = vec![];
        let v2 = vec![5, 10];  // sum = 15
        let result = higher_sum(&v1, &v2);
        assert_eq!(result, &v2);
    }

    #[test]
    fn test_negative_numbers() {
        let v1 = vec![-5, -10, 20];  // sum = 5
        let v2 = vec![-2, -2, 10];   // sum = 6
        let result = higher_sum(&v1, &v2);
        assert_eq!(result, &v2);
    }

    #[test]
    fn test_large_numbers() {
        let v1 = vec![1000000, 2000000];   // sum = 3000000
        let v2 = vec![999999, 999999];     // sum = 1999998
        let result = higher_sum(&v1, &v2);
        assert_eq!(result, &v1);
    }

    #[test]
    fn test_floating_point_numbers() {
        let v1 = vec![1.5, 2.5, 3.0];  // sum = 7.0
        let v2 = vec![2.0, 2.0, 2.5];  // sum = 6.5
        let result = higher_sum(&v1, &v2);
        assert_eq!(result, &v1);
    }

    #[test]
    fn test_zero_and_positive_numbers() {
        let v1 = vec![0, 0, 0];    // sum = 0
        let v2 = vec![1];          // sum = 1
        let result = higher_sum(&v1, &v2);
        assert_eq!(result, &v2);
    }
}
