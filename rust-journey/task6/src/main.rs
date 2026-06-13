fn find_largest(a: f64, b: f64, c: f64) -> f64 {
    if a>b {
        if a>c {a}
        else{c}
    }
    else{
        if b>c {b}
        else{c}
    }
}
fn find_largest_match(a: f64, b: f64, c: f64) -> f64 {
    match (a>b,a>c,b>c) {
        (true, true, _) => a,
        (false, _, true) => b,
        (false, _, false) => c,
        (true, false, false) => c,
        _ => c,
    }
}
fn main() {
    let (a,b,c): (f64, f64, f64) = (56.99,56.99,23.32);
    println!("Largest number using max(): {}", a.max(b).max(c));
    println!("Largest number using function: {}", find_largest(a, b, c));
    println!("Largest number using match: {}", find_largest_match(a, b, c))
}
