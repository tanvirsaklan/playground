// task_14_string_concat

fn main() {
    let first = String::from("John");
    let last = String::from("Doe");

    // =========================================================================
    // Method 1: The `+` Operator (Moves the left operand)
    // =========================================================================
    
    // We must push a space in between. Note how ownership shifts:
    // 1. `first` is MOVED into the first `+` operation.
    // 2. `" "` is a `&str` slice (borrowed).
    // 3. The result of `first + " "` is a temporary new String.
    // 4. `&last` is passed as a reference (borrowed via deref coercion).
    let full_name_plus = first + " " + &last;
    
    println!("Method 1 (plus): {}", full_name_plus);
    
    // UNCOMMENTING the line below will cause compilation error E0382:
    // println!("Can I still use first? {}", first);
    // Reason: `first` was consumed by the `+` operator and no longer exists here!
    
    println!("Can I still use last? {} (Yes, it was only borrowed)", last);

    // =========================================================================
    // Method 2: The `format!` Macro (Borrows all operands)
    // =========================================================================
    
    // Since `first` was moved above, let's make fresh variables for a clean test
    let first_clean = String::from("Jane");
    let last_clean = String::from("Smith");

    // `format!` internally uses references. It does NOT take ownership.
    let full_name_format = format!("{} {}", first_clean, last_clean);

    println!("\nMethod 2 (format!): {}", full_name_format);
    println!("first_clean: {} (Still usable!)", first_clean);
    println!("last_clean:  {} (Still usable!)", last_clean);
}

/*
=========================================================================
WHY DOES THE `+` OPERATOR MOVE THE LEFT OPERAND?
=========================================================================
Under the hood, the `+` operator in Rust translates directly to a trait method call. 
The signature for appending strings looks like this:

    fn add(self, s: &str) -> String

Notice the first parameter: it is `self`, NOT `&self`. 
- Because it takes `self` by value, the `add` function takes absolute ownership 
  of the left-hand string (`first`). 
- It reuses `first`'s internal heap buffer capacity to append the bytes of the 
  right-hand string directly onto it, preventing a brand-new memory allocation.
- This is a deliberate performance optimization built into Rust. If it borrowed 
  both sides, it would be forced to allocate a completely new block of heap memory 
  every single time you used `+`.
*/


/*
Why Rust Rejects It
As we saw in the previous task, the + operator is not magic;
It maps directly to a trait method signature.
For strings, that signature is strictly defined as:

fn add(self, s: &str) -> String

Rust forces this design on you for performance and memory safety.
With `first + &last`: Rust takes ownership of the heap memory buffer already allocated for first and copies the bytes of last directly into the end of it. It doesn't need to ask the operating system for a brand new memory slot; it reuses what is already there.
If `&first + &last` were allowed: Neither side would own any memory. Rust would be forced to silently allocate a completely separate, third buffer on the heap behind your scenes to combine them.
Rust’s core design philosophy is explicit performance. It refuses to hide expensive heap allocations behind simple operators like +. If you want a brand-new allocation without giving up ownership of your original variables, Rust forces you to be explicit about it by using the format! macro instead.
 */