pub fn sum_even_fibonacci(limit: i32) -> i32 {
    let fibonaccis = generate_fibonacci(limit);
    let mut sum = 0;
    for n in fibonaccis {
        if n % 2 == 0 {
            sum += n
        }
    }
    sum
}

pub fn generate_fibonacci(limit: i32) -> Vec<i32> {
    let mut fibonacci = vec![1, 2];
    for n in 3..limit {
        let x = n as usize;
        let minor = fibonacci[x-3];
        let mayor = fibonacci[x-2];
        let sum = minor + mayor;
        if sum <= limit {
            fibonacci.push(minor + mayor);
        } else {
            break;
        }
    }
    fibonacci
}
