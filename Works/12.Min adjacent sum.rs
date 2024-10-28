use rand::Rng;

fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..100)).collect()
}

fn min_adjacent_sum(data: &[i32]) -> (usize, usize, i32) {
    let mut min_sum = i32::MAX;
    let mut min_pair = (0, 1);

    for i in 0..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            min_pair = (i, i + 1);
        }
    }

    (min_pair.0, min_pair.1, min_sum)
}

fn print_vector_with_min_pair(data: &[i32]) {
    print!("indexes: ");
    for i in 0..data.len() {
        print!("{:>3}.", i);
    }
    println!();

    print!("data:    ");
    for &value in data {
        print!("{:>3},", value);
    }
    println!();

    let (idx1, idx2, min_sum) = min_adjacent_sum(data);
    println!("min adjacent sum={}+{}={} at indexes:{},{}",
             data[idx1], data[idx2], min_sum, idx1, idx2);
    println!();
}

fn main() {
    let data = gen_random_vector(20);

    print_vector_with_min_pair(&data);
}
