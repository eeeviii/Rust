fn invert_the_case(s: String) -> String {
    s.chars()
        .map(|c| {
            if c.is_uppercase() {
                c.to_lowercase().to_string()
            } else {
                c.to_uppercase().to_string()
            }
        })
        .collect()
}
fn main() {
    let result1 = invert_the_case("Hello".to_string());
    let result2 = invert_the_case("Привет".to_string());
    println!("{}", result1);
    println!("{}", result2);
}
