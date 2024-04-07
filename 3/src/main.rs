fn main() {
    println!("{}", largest_prime_factor(600851475143));
}

fn largest_prime_factor(mut n: usize) -> usize {
    let mut largest: usize = 0;
    let mut x = 2;
    while x * x <= n {
        if n % x == 0 {
            largest = x;
            n = n / x;
        }
        x += 1;
    }
    if n > largest as usize {
        largest = n;
    }
    largest
}