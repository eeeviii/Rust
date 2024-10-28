use std::cmp::Ordering;
use std::iter::successors;

fn gcd(a: i32, b: i32) -> i32 {
    match b.cmp(&0) {
        Ordering::Equal => a,
        _ => gcd(b, a % b),
    }
}

fn lcm(a: i32, b: i32) -> i32 {
    (a / gcd(a, b)) * b
}

fn get_total_x(a: Vec<i32>, b: Vec<i32>) -> i32 {
    let lcm_a = a.into_iter().reduce(|acc, x| lcm(acc, x)).unwrap();
    let gcd_b = b.into_iter().reduce(|acc, x| gcd(acc, x)).unwrap();

    successors(Some(lcm_a), |&x| Some(x + lcm_a))
        .take_while(|&x| x <= gcd_b)
        .filter(|&x| gcd_b % x == 0)
        .count() as i32
}

fn main() {
    let a = vec![2, 4];
    let b = vec![16, 32, 96];
    let result = get_total_x(a, b);
    println!("{}", result);
}
