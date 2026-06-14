fn count_digits_math(n: i64) -> u32 {
    if n==0 {
        return 1
    }
    else{
        let mut count = 0;
        let mut num = n;
        while num > 0 {
            num /= 10;
            count += 1;
        }
        count
    }
}
fn count_digits_string(n: i64) -> usize {
    n.to_string().len()
}
fn main() {
    let n = 00000;
    println!("Total digits in {} is: {}", n, count_digits_math(n));
    println!("Total digits in {} is: {}", n, count_digits_string(n));
}
