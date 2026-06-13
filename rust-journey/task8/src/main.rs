// task_08_multiplication_table

/// Prints a single multiplication table for a number `n` from 1 to 10
fn print_single_table(n: u32) {
    println!("--- Multiplication Table for {} ---", n);
    for i in 1..=10 {
        // {:>2} ensures the multiplier (i) aligns nicely if it reaches 10
        // {:>4} gives the result a right-aligned width of 4 spaces
        println!("{} x {:>2} = {:>4}", n, i, n * i);
    }
    println!();
}

/// Prints a complete 10x10 multiplication grid with aligned headers
fn print_10x10_grid() {
    println!("--- Bonus: 10 x 10 Multiplication Grid ---\n");

    // Print the top horizontal header row
    print!("    |"); // Top-left corner spacer
    for col in 1..=10 {
        print!("{:>4}", col);
    }
    println!("\n----|----------------------------------------");

    // Nested loops to populate the grid
    for row in 1..=10 {
        // Print the left vertical header column
        print!("{:>3} |", row);
        
        for col in 1..=10 {
            // Print the product, right-aligned to 4 spaces
            print!("{:>4}", row * col);
        }
        println!(); // Move to the next row
    }
}
fn main() {
    // 1. Single table execution
    print_single_table(7);

    // 2. Bonus grid execution
    print_10x10_grid();
}