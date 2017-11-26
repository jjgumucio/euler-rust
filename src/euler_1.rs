pub fn sum_multiples_3_5_till_number(limit: i32) -> i32 {
    let mut sum = 0;
    for n in 1..limit + 1 {
        if n % 3 == 0 || n % 5 == 0 {
            sum += n
        }
    }
    sum
}
