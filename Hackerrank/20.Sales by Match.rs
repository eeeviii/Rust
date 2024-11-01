use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    let colors: Vec<u32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let pairs = sock_merchant(n, colors);
    println!("{}", pairs);
}

fn sock_merchant(n: usize, colors: Vec<u32>) -> u32 {
    let mut color_count = HashMap::new();
    
    for color in colors {
        *color_count.entry(color).or_insert(0) += 1;
    }

    let mut pairs = 0;
    for &count in color_count.values() {
        pairs += count / 2;
    }

    pairs
}
