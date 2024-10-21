use std::io::{self, BufRead};

fn simple_array_sum(arr: &[i32]) -> i32 {
    arr.iter().sum()
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    lines.next();

    let arr: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let result = simple_array_sum(&arr);
    println!("{}", result);
}