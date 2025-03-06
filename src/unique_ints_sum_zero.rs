pub fn unique_ints_sum_zero(n: i32) -> Vec<i32> {
    let mut outcome: Vec<i32> = Vec::new();
    let mut start_at: i32 = 0;
    let mut j: i32 = 1;
    let mut once: bool = true;

    if n % 2 > 0 {
        start_at += 1;
        outcome.push(0);
    }

    for _ in start_at..n {
        once = !once;
        if !once {
            outcome.push(j);
        } else {
            outcome.push(&j * -1);
            j += 1;
        }
    }

    outcome
}
