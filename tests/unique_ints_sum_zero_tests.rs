use rust_leetcode::unique_ints_sum_zero::unique_ints_sum_zero;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn five_digits() {
        let result: Vec<i32> = unique_ints_sum_zero(5);
        let expected: Vec<i32> = vec![0, 1, -1, 2, -2];
        assert_eq!(result, expected);
    }

    #[test]
    fn three_digits() {
        let result: Vec<i32> = unique_ints_sum_zero(3);
        let expected: Vec<i32> = vec![0, 1, -1];
        assert_eq!(result, expected);
    }

    #[test]
    fn one_digit() {
        let result: Vec<i32> = unique_ints_sum_zero(1);
        let expected: Vec<i32> = vec![0];
        assert_eq!(result, expected);
    }

    #[test]
    fn four_digits() {
        let result: Vec<i32> = unique_ints_sum_zero(4);
        let expected: Vec<i32> = vec![1, -1, 2, -2];
        assert_eq!(result, expected);
    }

    #[test]
    fn ten_digits() {
        let result: Vec<i32> = unique_ints_sum_zero(10);
        let expected: Vec<i32> = vec![1, -1, 2, -2, 3, -3, 4, -4, 5, -5];
        assert_eq!(result, expected);
    }
}
