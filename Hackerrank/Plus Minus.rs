use std::io;

fn plus_minus(arr: &[i32]) {
    let n = arr.len() as f64;
    let mut positive_count = 0;
    let mut negative_count = 0;
    let mut zero_count = 0;

    for &num in arr.iter() {
        if num > 0 {
            positive_count += 1;
        } else if num < 0 {
            negative_count += 1;
        } else {
            zero_count += 1;
        }
    }

    println!("{:.6}", positive_count as f64 / n);
    println!("{:.6}", negative_count as f64 / n);
    println!("{:.6}", zero_count as f64 / n);
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let arr: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    plus_minus(&arr);
}
