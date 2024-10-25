use std::io;

fn birthday_cake_candles(candles: Vec<u32>) -> u32 {
    let max_height = *candles.iter().max().unwrap();  
    candles.iter().filter(|&&h| h == max_height).count() as u32  
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap(); 
    
    input.clear();
    io::stdin().read_line(&mut input).unwrap();  
    let candles: Vec<u32> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect(); 

    let result = birthday_cake_candles(candles);
    println!("{}", result); 
}
