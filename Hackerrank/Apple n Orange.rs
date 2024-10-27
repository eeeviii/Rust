fn count_fruits(s: i32, t: i32, a: i32, b: i32, apples: Vec<i32>, oranges: Vec<i32>) -> (i32, i32) {
    let apple_count = apples.iter()
        .map(|&d| a + d)
        .filter(|&pos| pos >= s && pos <= t)
        .count() as i32;

    let orange_count = oranges.iter()
        .map(|&d| b + d)
        .filter(|&pos| pos >= s && pos <= t)
        .count() as i32;

    (apple_count, orange_count)
}

fn main() {
    let s = 7;
    let t = 11;
    let a = 4;
    let b = 12;
    let apples = vec![-2, 2, 1];
    let oranges = vec![5, -6];

    let (apple_count, orange_count) = count_fruits(s, t, a, b, apples, oranges);
    println!("{}", apple_count);
    println!("{}", orange_count);
}
