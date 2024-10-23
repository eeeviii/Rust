use std::io;

fn a_very_big_sum(arr: &[i64]) -> i64 {
    arr.iter().sum()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let _n: usize = input.trim().parse().expect("Invalid input");

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let arr: Vec<i64> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Not a number"))
        .collect();

    let result = a_very_big_sum(&arr);

    println!("{}", result);
}
