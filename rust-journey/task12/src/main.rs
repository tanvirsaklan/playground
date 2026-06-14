// A wrapper that prints a message when it is dropped from memory
struct TrackedString(String);

impl Drop for TrackedString {
    fn drop(&mut self) {
        println!("🔒 [DROP] TrackedString holding '{}' is being freed!", self.0);
    }
}

fn create_greeting(name: &str) -> TrackedString {
    TrackedString(format!("Hello, {}", name)) 
}

fn take_and_return(mut tracked: TrackedString) -> TrackedString {
    tracked.0.push('!'); 
    tracked            
}

fn main() {
    println!("Main function started.");
    
    {
        println!("Entering inner scope...");
        let s = take_and_return(create_greeting("Rustacean"));
        println!("Inside scope: s is alive. s says: {}", s.0);
    } // <--- 's' hits the closing brace here. Watch the console!

    println!("Main function ended.");
}