use std::io;

fn read_float(prompt: &str) -> f64 {
    loop {
        let mut input = String::new();
        println!("{}", prompt);
        if io::stdin().read_line(&mut input).is_err() {
            println!("Failed to read line. Try again...");
            continue;
        }
        match input.trim().parse::<f64>() {
            Ok(num) if num >= 0.0 => return num,
            _ => println!("Invalid input. Try again..."),
        }
    }
}
fn main() {
    let p = read_float("Enter principal amount:");
    let r = read_float("Enter rate of interest:");
    let t = read_float("Enter time in years:");
    let si = (p * r * t) / 100.0;
    println!("Interest: {}", si);    
}