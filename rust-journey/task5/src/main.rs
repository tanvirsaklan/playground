use std::io;

fn read_float(prompt: &str) -> f64 {
    loop{
        let mut input = String::new();
        println!("{}", prompt);
        if io::stdin().read_line(&mut input).is_err() {
            println!("Line not read. Try again...");
            continue;            
        }
        match input.trim().parse::<f64>() {
            Ok(num) if num >= 0.0 => return num,
            _ => println!("Invalid format! Try again..."),
        }
    }
}
fn compound_interest(principal: f64, rate: f64, n: f64, time: f64) -> f64 {
    let actual_rate = rate/100.0;
    let base = 1.0 + (actual_rate/n);
    let exp = n * time;
    principal * (base.powf(exp))
}
fn main() {
    println!("============== Compound Interest =================");
    let p = read_float("Enter principal amount:");
    let r = read_float("Enter interest rate:");
    let n = read_float("Enter compounding frequency: (1=annually, 12=monthly, 365=daily)");
    let t = read_float("Enter time:");
    println!("Total amount is: {:.2}\nInterest Earned: {:.2}", compound_interest(p,r,n,t), compound_interest(p,r,n,t) - p);
}
