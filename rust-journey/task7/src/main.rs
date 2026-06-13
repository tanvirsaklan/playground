fn is_prime(n:u64) -> bool {
    for i in 2..=((n as f64).sqrt() as u64) {
        if n%i == 0 {return false}
        else{continue}
    }
    return true
}
fn main() {
    for i in 1..100 {
        if is_prime(i) {
            println!("{}",i)
        }
        else {
            continue
        }
    }
}