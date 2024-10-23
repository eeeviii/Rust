fn rotate(s: String, n: isize) -> String {
    let len = s.len() as isize;
    let shift = ((n % len) + len) % len; 

    let split_point = len as usize - shift as usize; 
    let (left, right) = s.split_at(split_point); 

    format!("{}{}", right, left) 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = "abcdefgh".to_string();
        let shifts = [
            (0, "abcdefgh"),
            (8, "abcdefgh"),
            (-8, "abcdefgh"),
            (1, "habcdefg"),
            (2, "ghabcdef"),
            (10, "ghabcdef"),
            (-1, "bcdefgha"),
            (-2, "cdefghab"),
            (-10, "cdefghab"),
        ];

        shifts.iter().for_each(|(n, expected)| {
            assert_eq!(rotate(s.clone(), *n), expected.to_string());
        });
    }
}
