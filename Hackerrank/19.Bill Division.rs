use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap().unwrap();
    let parts: Vec<usize> = first_line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let k = parts[1];

    let second_line = lines.next().unwrap().unwrap();
    let bill: Vec<i32> = second_line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let charged: i32 = lines.next().unwrap().unwrap().parse().unwrap();

    let total_cost: i32 = bill.iter().sum();
    let anna_share = (total_cost - bill[k]) / 2;

    if charged == anna_share {
        println!("Bon Appetit");
    } else {
        println!("{}", charged - anna_share);
    }
}
