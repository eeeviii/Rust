fn divisible_sum_pairs(n: usize, k: i32, arr: Vec<i32>) -> i32 {
    let mut count = 0;

    for i in 0..n {
        for j in (i + 1)..n {
            if (arr[i] + arr[j]) % k == 0 {
                count += 1;
            }
        }
    }

    count
}

fn main() {
    let arr = vec![1, 3, 2, 6, 1, 2];
    let k = 3;
    let result = divisible_sum_pairs(arr.len(), k, arr);
    println!("{}", result);
}
