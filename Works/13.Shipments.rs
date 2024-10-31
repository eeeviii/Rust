use rand::Rng;

fn count_permutation(shipments: &Vec<u32>) -> Option<usize> {
    let total: u32 = shipments.iter().sum();
    let n = shipments.len() as u32;

    if total % n != 0 {
        return None;
    }

    let target_weight = total / n;
    let mut moves = 0;
    let mut balance = 0;

    for &weight in shipments {
        balance += weight as isize - target_weight as isize;
        moves += balance.abs();
    }

    Some(moves as usize)
}

fn gen_shipments(n: usize) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let target_weight = rng.gen_range(1..100);
    vec![target_weight; n]
}

fn main() {
    let shipments1 = vec![8, 2, 2, 4, 4];
    match count_permutation(&shipments1) {
        Some(moves) => println!("Для кораблів {:?} мінімальна кількість переміщень: {}", shipments1, moves),
        None => println!("Для кораблів {:?} неможливо розподілити вагу рівномірно", shipments1),
    }

    let shipments2 = vec![9, 3, 7, 2, 9];
    match count_permutation(&shipments2) {
        Some(moves) => println!("Для кораблів {:?} мінімальна кількість переміщень: {}", shipments2, moves),
        None => println!("Для кораблів {:?} неможливо розподілити вагу рівномірно", shipments2),
    }

    let generated_shipments = gen_shipments(5);
    println!("Згенеровані кораблі з рівномірним вантажем: {:?}", generated_shipments);
    match count_permutation(&generated_shipments) {
        Some(moves) => println!("Мінімальна кількість переміщень: {}", moves),
        None => println!("Неможливо розподілити вагу рівномірно"),
    }
}
