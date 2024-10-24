use std::io;

fn staircase(n: usize) {
    for i in 1..=n {
        let spaces = " ".repeat(n - i);
        let hashes = "#".repeat(i);
        println!("{}{}", spaces, hashes);
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    staircase(n);
}
