/// Approach one uses a mutable variable `x` to increment its value by 20. This works because `x` is mutable.
/// We can mutate `x` because we are mutating it to a new value of same type.
/// Otherwise we can not mutate if we assign x's new value to a different type e.g. `x = "hello";`
fn approach_one() {
    let mut x = 10;
    println!("x: {}", x);
    x = x + 20;
    println!("x: {}", x);
}

/// Approach two uses a mutable variable `x` to assign a new value of different type. This does not work because `x` is mutable but it is mutated to a different type.
fn approach_two() {
    let mut x = 10;
    println!("x: {}", x);
    // x = "Hello".to_string(); // throws error because new value is of different type
    println!("x: {}", x);
}

/// Approach three uses shadowing to create a new variable `x` with a new value of same type.
fn approach_three() {
    let x = 10;
    println!("x: {}", x);
    let x = x + 20; // Shadowing
    println!("x: {}", x);
}

fn approach_four() {
    let x = 10;
    println!("x: {}", x);
    // x = x + 20; // throws error beacuse it is not mutable
    println!("x: {}", x);
}

fn main() {
    approach_one();
    // approach_two();
    approach_three();
    // approach_four();
}