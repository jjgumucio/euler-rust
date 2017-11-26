pub fn get_max_prime_factor(mut big_number: u64) -> u64 {
    let mut factor: u64 = 2;
    let mut last_factor: u64 = 1;
    while big_number > 1 {
        if big_number % factor == 0 {
            big_number = big_number / factor;
            last_factor = factor;
            while big_number % factor == 0 {
                big_number = big_number / factor;
            }
        }
        factor += 1;
    }
    last_factor
}
