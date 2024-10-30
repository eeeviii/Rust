fn birthday(s: Vec<i32>, d: i32, m: usize) -> i32 {
    let mut count = 0;

    for i in 0..=s.len() - m {
        let sum: i32 = s[i..i + m].iter().sum();
        if sum == d {
            count += 1;
        }
    }

    count
}

fn main() {
    let s = vec![1, 2, 1, 3, 2];
    let d = 3;
    let m = 2;
    
    let result = birthday(s, d, m);
    println!("{}", result);
}
