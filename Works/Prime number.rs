fn is_prime(n: u64) -> bool {
    if n <= 1 || (n > 2 && n % 2 == 0) {
        return false;
    }
    (3..=((n as f64).sqrt() as u64)).filter(|i| n % i == 0).count() == 0
}

fn main() {
    let number = 11;
    println!("Is {} prime? {}", number, is_prime(number));
}
