fn main() {
    let mut a = 5;
    let mut b = 10;
    println!("Before swap: \na: {}, b: {}", a, b);
    (a,b) = (b,a);
    println!("After swap: \na: {}, b: {}", a, b);
}