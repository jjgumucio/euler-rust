mod euler_1;
mod euler_2;
mod euler_3;

fn main() {
    println!("Euler 1: Sum of all multiples of 3 & 5 below 1000: {}",
             euler_1::sum_multiples_3_5_till_number(999));
    println!("Euler 2: Sum of even fibonacci to {} is {}",
             4000000,
             euler_2::sum_even_fibonacci(4000000));
    println!("Euler 3: Biggest prime factor of {} is {}",
             600851475143 as u64,
             euler_3::get_max_prime_factor(600851475143 as u64));
}

fn get_multiples(multiple: i32, limit: i32) -> Vec<i32> {
    let mut multiples = Vec::new();
    for i in 1..limit + 1 {
        if i % multiple == 0 {
            multiples.push(i);
        } 
    }
    multiples
}

fn summation(numbers: Vec<i32>) -> i32 {
    let mut summation = 0;
    for i in numbers {
        summation += i;
    }
    summation
}

fn union(vectorA: Vec<i32>, vectorB: Vec<i32>) -> Vec<i32> {
    let mut uniques = vectorA.clone();
    for e in vectorB {
        if !uniques.contains(&e) {
            uniques.push(e)
        }
    }
    uniques
}
