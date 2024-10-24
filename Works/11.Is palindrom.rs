fn is_palindrome(x: u32) -> bool {
    let original = x.to_string();
    let reversed: String = original.chars().rev().collect();
    original == reversed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        let data = [
            (123, false),
            (121, true),
            (1221, true),
        ];

        data.iter().for_each(|(n, exp)| {
            assert_eq!(is_palindrome(*n), *exp);
        });
    }
}

fn main() {
    let num = 121;
    println!("Is {} a palindrome? {}", num, is_palindrome(num));
}
