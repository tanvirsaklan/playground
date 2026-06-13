fn reverse_number(mut n: i64) -> i64 {
    let mut reversed = 0;
    while n != 0 {
        let last_digit = n%10;
        reversed = reversed*10 + last_digit;
        n/=10;
    }
    reversed
}
fn main() {
    let test_cases = [12345, -6789, 1000, 0];
    for &num in test_cases.iter() {
        println!("Original: {:>6} | Reversed: {:>6}", num, reverse_number(num));
    }
}