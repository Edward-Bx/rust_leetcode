use rust_leetcode::shortest_to_char::shortest_to_char;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiple_hits() {
        let result: Vec<i32> = shortest_to_char(String::from("loveleetcode"), 'e');
        assert_eq!(result, [3, 2, 1, 0, 1, 0, 0, 1, 2, 2, 1, 0]);
    }

    #[test]
    fn multiple_hits_two_chars() {
        let result: Vec<i32> = shortest_to_char(String::from("aaba"), 'b');
        assert_eq!(result, [2, 1, 0, 1]);
    }

    #[test]
    fn only_hits() {
        let result: Vec<i32> = shortest_to_char(String::from("zzzzzzz"), 'z');
        assert_eq!(result, [0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn single_hit() {
        let result: Vec<i32> = shortest_to_char(String::from("snapje"), 'e');
        assert_eq!(result, [5, 4, 3, 2, 1, 0]);
    }

    #[test]
    fn very_long_string() {
        let result: Vec<i32> = shortest_to_char(String::from("thisisasuperlongstringthatisverylongandhasmanycharactersthisisasuperlongstringthatisverylongandhasmanycharacterstimestwo"), 'c');
        assert_eq!(
            result,
            [
                46, 45, 44, 43, 42, 41, 40, 39, 38, 37, 36, 35, 34, 33, 32, 31, 30, 29, 28, 27, 26,
                25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4,
                3, 2, 1, 0, 1, 2, 2, 1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
                17, 18, 19, 20, 21, 22, 23, 24, 25, 25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 14,
                13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 1, 2, 2, 1, 0, 1, 2, 3, 4, 5, 6, 7,
                8, 9, 10, 11, 12
            ]
        );
    }
}
