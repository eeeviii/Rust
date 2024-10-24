use std::io;

fn diagonal_difference(matrix: &[Vec<i32>], n: usize) -> i32 {
    let mut primary_diagonal_sum = 0;
    let mut secondary_diagonal_sum = 0;

    for i in 0..n {
        primary_diagonal_sum += matrix[i][i];            
        secondary_diagonal_sum += matrix[i][n - 1 - i];  
    }

    (primary_diagonal_sum - secondary_diagonal_sum).abs()  

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let n: usize = input.trim().parse().expect("Invalid input");

    let mut matrix = Vec::new();
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let row: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse().expect("Not a number"))
            .collect();
        matrix.push(row);
    }

    let result = diagonal_difference(&matrix, n);
    println!("{}", result);
}
