fn main() {
    let mut a : usize = 1;
    let mut b : usize = 0;
    let mut c : usize = 0;

    let mut sum = 0;
    while c < 4e9 as usize {
        c = a + b;
        a = b;
        b = c;

        if c % 2 == 0 {
            sum += c;
        }
    }
    println!("{}", sum);
}
