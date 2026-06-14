/*fn some() {
    let s1 = String::from("Hello");
    println!("{}", s1);
    let s2 = s1; // moved permanently from s1 to s2
    println!("{}", s1); // Can not access s1 anymore because 
    println!("{}", s2);
}
*/
fn some() {
    let s1 = String::from(" Some Hello");
    println!("Before s1: {}", s1);
    let s2 = s1.clone(); // now the variable data is cloned/copied
    println!("After s1: {}", s1); // both are okay
    println!("s2: {}", s2);
    let mut s3 = s1.clone();
    s3 += " From mutable s3";
    println!("{}", s3);
}
/*
fn take_string(s: String) {
    println!("Inside fn: {}",s);
}
fn main() {
    let s = String::from("Hello");
    println!("Before fn: {}", s);
    take_string(s); // this fn takes ownership of the variable, so we can not use it after.
    println!("After fn: {}", s);
}
 */
fn take_string(s: &String) { // `&String` borrows the variable, uses it, then drops them.
    println!("Inside fn: {}",s);
}
fn main() {
    let s = String::from("Hello");
    println!("Before fn: {}", s);
    take_string(&s); // thus borrowing happens `&s`
    println!("After fn: {}", s);
    some();
}