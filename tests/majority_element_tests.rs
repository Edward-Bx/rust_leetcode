use rust_leetcode::majority_element::majority_element;

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    #[test]
    fn expect_three() {
        let result = majority_element(vec![3, 2, 3]);
        assert_eq!(result, 3);
    }

    #[test]
    fn expect_two() {
        let result = majority_element(vec![2, 2, 1, 1, 1, 2, 2]);
        assert_eq!(result, 2);
    }

    #[test]
    fn expect_five() {
        let result = majority_element(vec![
            2, 5, 5, 5, 5, 5, 5, 5, 5, 5, 0, 2, 2, 2, 2, 1, 1, 1, 2, 2, 7, 8, 7, 6, 7, 8, 9, 8, 6,
            6, 6, 5, 5, 5,
        ]);
        assert_eq!(result, 5);
    }

    #[test]
    #[should_panic]
    fn empty_vector_panics() {
        // Extra logic to hide panic message from terminal when running tests
        let original_hook = panic::take_hook();
        panic::set_hook(Box::new(|_| {}));
        majority_element(vec![]);
        panic::set_hook(original_hook);
    }
}
