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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_evens_mixed_numbers() {
        let mut nums = vec![1, 2, 3, 4, 5, 6];
        double_evens(&mut nums);
        assert_eq!(nums, vec![1, 4, 3, 8, 5, 12]);
    }

    #[test]
    fn test_double_evens_all_even() {
        let mut nums = vec![2, 4, 6];
        double_evens(&mut nums);
        assert_eq!(nums, vec![4, 8, 12]);
    }

    #[test]
    fn test_double_evens_all_odd() {
        let mut nums = vec![1, 3, 5, 7];
        double_evens(&mut nums);
        assert_eq!(nums, vec![1, 3, 5, 7]);
    }

    #[test]
    fn test_double_evens_empty_vector() {
        let mut nums: Vec<i32> = vec![];
        double_evens(&mut nums);
        assert_eq!(nums, vec![]);
    }

    #[test]
    fn test_double_evens_single_even() {
        let mut nums = vec![10];
        double_evens(&mut nums);
        assert_eq!(nums, vec![20]);
    }

    #[test]
    fn test_double_evens_single_odd() {
        let mut nums = vec![7];
        double_evens(&mut nums);
        assert_eq!(nums, vec![7]);
    }

    #[test]
    fn test_double_evens_with_zero() {
        let mut nums = vec![0, 1, 2, 3];
        double_evens(&mut nums);
        assert_eq!(nums, vec![0, 1, 4, 3]);
    }

    #[test]
    fn test_double_evens_with_negative_numbers() {
        let mut nums = vec![-4, -3, -2, 1, 2];
        double_evens(&mut nums);
        assert_eq!(nums, vec![-8, -3, -4, 1, 4]);
    }

    #[test]
    fn test_double_evens_large_numbers() {
        let mut nums = vec![1000000, 1000001, 500000];
        double_evens(&mut nums);
        assert_eq!(nums, vec![2000000, 1000001, 1000000]);
    }

    #[test]
    fn test_double_evens_consecutive_evens() {
        let mut nums = vec![2, 4, 6, 8, 10];
        double_evens(&mut nums);
        assert_eq!(nums, vec![4, 8, 12, 16, 20]);
    }
}
