use std::io;

fn mini_max_sum(arr: &[u64]) {
    let total_sum: u64 = arr.iter().sum();  
    let min_value = arr.iter().min().unwrap();  
    let max_value = arr.iter().max().unwrap(); 

    let min_sum = total_sum - max_value;  
    let max_sum = total_sum - min_value;  

    println!("{} {}", min_sum, max_sum);
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();  
    let arr: Vec<u64> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect(); 

    mini_max_sum(&arr);
}
