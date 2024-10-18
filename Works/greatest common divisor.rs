fn gcd (a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
fn main() {
    let num1 = 52;
    let num2 = 69;
    
    println!("GCD of {} and {} is {}", num1, num2, gcd(num1,num2));
}
