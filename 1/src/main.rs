
fn sum_of_multiples_of_3_and_5(n: usize) -> usize {
    let mut sum = 0;
    for i in 0..n {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
    }
    sum
}

fn main() {
    println!("{}", sum_of_multiples_of_3_and_5(1000));
}
