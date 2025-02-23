use std::collections::HashMap;

pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut nums_map = HashMap::new();

    for num in nums.iter() {
        nums_map
            .entry(num)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    let mut max_key: Option<i32> = None;
    let mut max_value = 0;

    for (&key, &value) in &nums_map {
        if &value > &max_value {
            max_value = value;
            max_key = Some(*key);
        }
    }
    max_key.unwrap()
}
