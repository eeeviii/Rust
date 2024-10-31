fn migratory_birds(arr: Vec<i32>) -> i32 {
    let mut counts = vec![0; 6]; // Вважаємо, що є тільки типи від 1 до 5

    for &bird in &arr {
        counts[bird as usize] += 1;
    }

    let mut max_count = 0;
    let mut bird_type = 0;

    for (i, &count) in counts.iter().enumerate().skip(1) {
        if count > max_count || (count == max_count && i < bird_type) {
            max_count = count;
            bird_type = i;
        }
    }

    bird_type as i32
}

fn main() {
    let arr = vec![1, 1, 2, 2, 3];
    let result = migratory_birds(arr);
    println!("{}", result);
}
