/// Prints all elements of a vector using an immutable reference
fn print_vec(v: &Vec<i32>) {
    println!("Vector elements:");
    for item in v {
        print!("{} ", item);
    }
    println!("\n");
}

/// Finds the maximum value in a vector, returning None if the vector is empty
fn find_max(v: &Vec<i32>) -> Option<i32> {
    if v.is_empty() {
        None
    } else {
        // v.iter() yields references, so we dereference the maximum value found
        let mut max_val = v[0];
        for &item in v {
            if item > max_val {
                max_val = item;
            }
        }
        Some(max_val)
    }
}

fn main() {
    // The original owner of the vector memory allocation
    let numbers = vec![23, 89, 42, 12, 7, 55];

    // 1. Pass an immutable reference to print_vec
    print_vec(&numbers);

    // 2. Pass an immutable reference to find_max
    match find_max(&numbers) {
        Some(max) => println!("The maximum value is: {}", max),
        None => println!("The vector is empty!"),
    }

    // 3. Verification: We can still use 'numbers' here because it was never moved!
    println!("\nVerification: 'numbers' is still accessible. Total count: {}", numbers.len());
}