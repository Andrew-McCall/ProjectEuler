fn main() {
    println!("{}", largest_prime_factor(600851475143));
}

fn largest_prime_factor(n: usize) -> usize {
    for i in 1..n{
        let current = n-i;
        println!("{}", current);
        if n%current == 0 && is_prime(current) {
            return current;
        } 
    }
    0
}

fn is_prime(n: usize) -> bool {
    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }
    true
}