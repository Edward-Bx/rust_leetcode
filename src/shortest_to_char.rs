pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
    let mut found_char: Vec<i32> = Vec::new();
    let mut k: usize = 0;
    let mut result: Vec<i32> = Vec::new();

    for (i, chr) in s.chars().enumerate() {
        if chr == c {
            found_char.push(i as i32);
        }
    }

    for j in 0..s.len() {
        if let Some(curr) = found_char.get(k) {
            let j_i32 = j as i32;
            let diff: i32 = (j_i32 - *curr).abs();

            if j_i32 <= *curr {
                result.push(diff);
            } else {
                if let Some(next) = found_char.get(k + 1) {
                    let diff_next = (j_i32 - *next).abs();
                    if diff > diff_next {
                        k += 1;
                        result.push(diff_next);
                    } else {
                        result.push(diff)
                    }
                } else {
                    result.push(diff);
                }
            }
        }
    }

    result
}
