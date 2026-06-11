# Rust Mastery Practice Tasks
> From Python/Django Developer → Rust Systems & Web Dev Pro
> Focus: Ownership · Borrowing · Lifetimes · Mutability · Traits · Structs · Error Handling · no_std · DSA · Database

---

## How to Use This File

Each task has:
- **Goal** — what concept you're practicing
- **Instructions** — step-by-step what to build
- **Key Concepts** — Rust-specific things to learn/apply
- **Python Comparison** — bridge from your Django background
- **Expected Output or Behavior**

Work top to bottom. Don't skip. Each phase builds on the last.

---

# PHASE 1: Rust Syntax & Ownership (Tasks 1–20)

---

## Task 1 — Swap Two Numbers Without a Third Variable

**Goal:** Get comfortable with Rust variable binding and mutability syntax.

**Instructions:**
1. Create a new project: `cargo new task_01_swap`
2. Declare two mutable integer variables `a` and `b`
3. Swap their values using tuple destructuring: `(a, b) = (b, a);`
4. Print both variables before and after the swap
5. Try doing it with XOR bitwise operations as a bonus

**Key Concepts:**
- `let mut` for mutable variables
- Tuple destructuring assignment
- `println!` macro with `{}` placeholder

**Python Comparison:**
```python
a, b = b, a  # Python tuple swap
```
In Rust you must declare mutability explicitly. Without `mut`, the compiler refuses to compile.

**Expected Output:**
```
Before: a = 5, b = 10
After:  a = 10, b = 5
```

---

## Task 2 — Immutable vs Mutable Variables

**Goal:** Understand that Rust variables are immutable by default and when to use `mut`.

**Instructions:**
1. Create `task_02_mutability`
2. Declare an immutable variable `x = 10`
3. Try reassigning it and observe the compiler error
4. Declare a mutable variable `y = 10` and reassign it to `20`
5. Use **shadowing**: declare `let x = 5`, then `let x = x + 1`, then `let x = x * 2`. Print each step
6. Note the difference: shadowing creates a new binding; `mut` changes the same binding

**Key Concepts:**
- `let` vs `let mut`
- Shadowing with `let` (not mutation)
- Compiler error messages are your friend

**Python Comparison:**
Python variables are always reassignable. Rust forces you to declare your intent upfront — this prevents accidental mutation bugs.

---

## Task 3 — Temperature Converter

**Goal:** Practice functions, parameters, return types, and basic arithmetic.

**Instructions:**
1. Create `task_03_temperature`
2. Write a function `celsius_to_fahrenheit(c: f64) -> f64`
3. Write a function `fahrenheit_to_celsius(f: f64) -> f64`
4. Write a function `celsius_to_kelvin(c: f64) -> f64`
5. In `main`, call all three and print formatted output
6. Use `const` to define absolute zero in Celsius: `const ABSOLUTE_ZERO: f64 = -273.15;`

**Key Concepts:**
- Function signature: `fn name(param: Type) -> ReturnType`
- `f64` for floating point
- `const` for compile-time constants
- No implicit type coercion (no `float(x)` needed, but types must match)

**Formula:**
- `F = C * 9/5 + 32`
- `K = C + 273.15`

**Expected Output:**
```
100°C = 212.00°F
212°F = 100.00°C
100°C = 373.15K
```

---

## Task 4 — Simple Interest Calculator

**Goal:** Practice user input, type parsing, and basic math functions.

**Instructions:**
1. Create `task_04_simple_interest`
2. Add `use std::io;` at the top
3. Write a function to read a line from stdin: `fn read_input(prompt: &str) -> String`
4. Read principal, rate, and time from the user
5. Parse each string to `f64` using `.parse::<f64>().unwrap()`
6. Calculate SI = (P * R * T) / 100
7. Print the result

**Key Concepts:**
- `std::io::stdin().read_line()`
- String `.trim()` to remove newline
- `.parse::<T>()` returns a `Result` — using `.unwrap()` for now (you'll handle errors properly in Phase 7)
- String references `&str` vs owned `String`

**Python Comparison:**
```python
p = float(input("Principal: "))
```
In Rust, `input()` doesn't exist. You read a line, trim it, then parse it.

---

## Task 5 — Compound Interest Calculator

**Goal:** Practice `f64` math and the `powi`/`powf` methods.

**Instructions:**
1. Create `task_05_compound_interest`
2. Write a function `compound_interest(principal: f64, rate: f64, n: f64, time: f64) -> f64`
3. Formula: `A = P * (1 + r/n)^(n*t)` where r is rate/100
4. Return the total amount and print both amount and interest earned
5. Accept n (compounding frequency) as: 1=annually, 12=monthly, 365=daily

**Key Concepts:**
- `f64::powf()` for float exponents
- Returning values from functions (no `return` keyword needed for last expression)
- Formatting floats with `{:.2}` for 2 decimal places

---

## Task 6 — Find Largest Among Three Numbers

**Goal:** Practice `if/else if/else` and comparison operators.

**Instructions:**
1. Create `task_06_largest`
2. Write a function `largest_of_three(a: i32, b: i32, c: i32) -> i32`
3. Implement using `if/else if/else`
4. Implement a second version using `.max()` chained: `a.max(b).max(c)`
5. Write a third version using a `match` expression on boolean tuples
6. Compare all three outputs

**Key Concepts:**
- `if` is an expression in Rust (can return a value)
- `i32` integer type
- Methods on primitives: `.max()`, `.min()`
- `match` expressions

---

## Task 7 — Prime Number Checker

**Goal:** Practice loops, early returns, and boolean logic.

**Instructions:**
1. Create `task_07_prime`
2. Write `fn is_prime(n: u64) -> bool`
3. Handle edge cases: numbers less than 2 are not prime
4. Loop from 2 to `sqrt(n)` using `(2..=((n as f64).sqrt() as u64))`
5. If `n % i == 0`, return `false` early
6. In `main`, check several numbers and print results
7. Bonus: print all primes up to 100

**Key Concepts:**
- `u64` unsigned integer
- Range syntax `2..=limit` (inclusive)
- `return` for early exit
- Type casting with `as`

**Expected Output:**
```
2 is prime: true
4 is prime: false
17 is prime: true
```

---

## Task 8 — Multiplication Table Generator

**Goal:** Practice nested loops and formatted output.

**Instructions:**
1. Create `task_08_multiplication_table`
2. Accept a number `n` (hardcoded or from input)
3. Print the multiplication table for `n` from 1 to 10
4. Format output so columns align: use `{:>4}` for right-aligned 4-wide fields
5. Bonus: print a full 10×10 multiplication grid

**Key Concepts:**
- `for i in 1..=10` loop
- Nested `for` loops
- Print formatting with width specifiers: `{:>width$}`

---

## Task 9 — Reverse a Number

**Goal:** Practice integer arithmetic (modulo and division).

**Instructions:**
1. Create `task_09_reverse_number`
2. Write `fn reverse_number(mut n: i64) -> i64`
3. Use `% 10` to extract the last digit, `/ 10` to remove it, build reversed number
4. Handle negative numbers (preserve the sign)
5. Test with: 12345, -6789, 1000, 0

**Key Concepts:**
- `mut` parameter (parameter ownership is yours, you can make it mutable)
- Integer arithmetic operators: `%`, `/`, `*`
- Loop with accumulator pattern

---

## Task 10 — Count Digits in a Number

**Goal:** Practice integer-to-string conversion and string methods.

**Instructions:**
1. Create `task_10_count_digits`
2. Write two versions:
   - `fn count_digits_math(n: i64) -> u32` — using repeated division
   - `fn count_digits_string(n: i64) -> usize` — convert to string, count chars
3. Handle the special case: 0 has 1 digit
4. Compare both approaches

**Key Concepts:**
- `n.to_string()` converts any integer to a `String`
- `.chars().count()` to count Unicode characters
- `i64::abs()` for absolute value

---

## Task 11 — Create a String and Transfer Ownership

**Goal:** Understand the single-owner rule — the core of Rust's memory model.

**Instructions:**
1. Create `task_11_ownership_move`
2. Create a `String` with `String::from("hello")`
3. Assign it to another variable — observe the move
4. Try to use the original variable after the move — read the compiler error carefully
5. Write a function `fn take_string(s: String)` that takes ownership
6. Call it and then try to use the string after — note it's gone
7. Comment your code explaining why this works the way it does

**Key Concepts:**
- `String` is a heap-allocated type — it moves, not copies
- After a move, the original binding is invalid (compile error, not runtime crash)
- This is Rust's way of preventing double-free memory bugs
- Primitives like `i32` implement `Copy` — they don't move

**Python Comparison:**
Python uses reference counting with garbage collection. Rust tracks ownership at compile time — zero runtime cost.

---

## Task 12 — Return Ownership from a Function

**Goal:** Learn that functions can give ownership back via return values.

**Instructions:**
1. Create `task_12_return_ownership`
2. Write `fn create_greeting(name: &str) -> String` — creates and returns a String
3. Write `fn take_and_return(s: String) -> String` — takes ownership, appends "!", returns it
4. Chain these: `let s = take_and_return(create_greeting("Rustacean"));`
5. Print the final string — ownership is yours again

**Key Concepts:**
- Functions can return `String` to transfer ownership to the caller
- The return value's lifetime is tied to the caller's scope
- `&str` is a string slice (borrowed reference) — different from `String`

---

## Task 13 — Clone vs Move Experiment

**Goal:** Know when to clone (deep copy) and when moving is sufficient.

**Instructions:**
1. Create `task_13_clone_vs_move`
2. Create a `Vec<i32>` with some values
3. Move it into a function — verify it can't be used afterward
4. Clone it before moving: `let v2 = v.clone();` — now both work
5. Time (conceptually) that clone is O(n) — it copies all elements
6. Demonstrate `Copy` with `i32`: primitives are always copied, never moved
7. Write a comment explaining when to use clone vs references

**Key Concepts:**
- `.clone()` performs a deep copy — allocates new memory
- `Copy` trait: types that are cheap to copy (integers, booleans, chars, tuples of Copy types)
- Clone is explicit and deliberate — Rust won't silently copy expensive types

---

## Task 14 — String Concatenation Using Ownership

**Goal:** Understand how `+` operator and `format!` work differently with ownership.

**Instructions:**
1. Create `task_14_string_concat`
2. Demonstrate `+` operator: `let s3 = s1 + &s2;` — note s1 is moved, s2 is borrowed
3. Try to use s1 after — read the error
4. Use `format!("{}{}", s1, s2)` — both strings are borrowed, neither is moved
5. Build a full name from first + " " + last using both methods
6. Explain in a comment why `+` moves the left operand

**Key Concepts:**
- `+` on `String` calls `fn add(self, s: &str) -> String` — self is moved
- `format!` macro borrows all its arguments
- `&String` auto-derefs to `&str` (deref coercion)

---

## Task 15 — Vector Ownership Transfer

**Goal:** Practice moving collections in and out of functions.

**Instructions:**
1. Create `task_15_vec_move`
2. Create `let numbers: Vec<i32> = vec![1, 2, 3, 4, 5];`
3. Write `fn sum_vec(v: Vec<i32>) -> (Vec<i32>, i32)` — consumes and returns the vector along with the sum
4. Call it, destructure the returned tuple
5. Write `fn double_values(mut v: Vec<i32>) -> Vec<i32>` — mutates and returns
6. Chain calls, collect the final vector

**Key Concepts:**
- Tuples for returning multiple values
- Destructuring with `let (vec, sum) = ...`
- `mut` on function parameters
- `.iter().sum::<i32>()` for summing

---

## Task 16 — Immutable Borrow of a Vector

**Goal:** Use references to read data without taking ownership.

**Instructions:**
1. Create `task_16_immutable_borrow`
2. Create a vector `numbers`
3. Write `fn print_vec(v: &Vec<i32>)` — takes an immutable reference
4. Write `fn find_max(v: &Vec<i32>) -> Option<i32>` — returns max or None if empty
5. Call both functions, then use `numbers` again after — it was only borrowed, not moved

**Key Concepts:**
- `&T` is an immutable (shared) reference
- You can have multiple immutable references simultaneously
- The original owner retains ownership after the function returns
- `Option<T>` for nullable values — Rust has no null

---

## Task 17 — Mutable Borrow of a Vector

**Goal:** Modify data through a reference without taking ownership.

**Instructions:**
1. Create `task_17_mutable_borrow`
2. Create `let mut numbers = vec![3, 1, 4, 1, 5];`
3. Write `fn sort_and_dedup(v: &mut Vec<i32>)` — sorts in place, removes duplicates
4. Write `fn append_values(v: &mut Vec<i32>, new_vals: &[i32])` — adds items
5. Call both, print the vector before and after
6. Try creating two mutable references at the same time — read the error

**Key Concepts:**
- `&mut T` is a mutable (exclusive) reference
- Only ONE mutable reference allowed at a time (prevents data races)
- Cannot have immutable and mutable references simultaneously
- `v.sort()`, `v.dedup()`, `v.extend_from_slice()`

---

## Task 18 — Modify String Using Mutable References

**Goal:** Manipulate string data through `&mut String`.

**Instructions:**
1. Create `task_18_string_mut_ref`
2. Write `fn make_uppercase(s: &mut String)` — converts in place using `*s = s.to_uppercase()`
3. Write `fn append_suffix(s: &mut String, suffix: &str)` — adds a suffix
4. Write `fn truncate_to(s: &mut String, max_len: usize)` — truncates if longer
5. Apply all three to the same string and print at each step

**Key Concepts:**
- `*s` dereferences to access the String through the reference
- String methods: `.to_uppercase()`, `.push_str()`, `.truncate()`
- The borrow ends when the reference goes out of scope

---

## Task 19 — Dangling Reference Prevention

**Goal:** See how Rust prevents one of C's most dangerous bugs.

**Instructions:**
1. Create `task_19_no_dangling`
2. Try to write a function that returns a reference to a local variable:
   ```rust
   fn dangle() -> &String {
       let s = String::from("hello");
       &s  // s drops here!
   }
   ```
3. Read the compiler error about lifetimes
4. Fix it by returning the owned `String` instead
5. Write another version using a lifetime annotation: `fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str`
6. Comment explaining what lifetime `'a` means

**Key Concepts:**
- Lifetimes ensure references don't outlive the data they point to
- The borrow checker enforces this at compile time
- Lifetime annotations don't change how long data lives — they describe relationships

---

## Task 20 — Document Ownership Rules

**Goal:** Consolidate everything from Tasks 11–19 by writing your own notes in code.

**Instructions:**
1. Create `task_20_ownership_notes`
2. Write a Rust source file that is heavily commented, demonstrating all three ownership rules:
   - Each value has one owner
   - When the owner goes out of scope, the value is dropped
   - There can only be one owner at a time
3. Demonstrate the borrowing rules:
   - Any number of immutable references
   - Exactly one mutable reference
   - References must always be valid
4. Include working code examples for each rule
5. This file will be your personal reference sheet

---

# PHASE 2: Loops & Control Flow (Tasks 21–35)

---

## Task 21 — Print 1–100 Using `loop`

**Goal:** Understand Rust's infinite `loop` with manual break.

**Instructions:**
1. Create `task_21_loop`
2. Use `loop { }` with a counter variable
3. Break when the counter exceeds 100
4. `loop` can return a value: `let result = loop { if condition { break value; } };` — demonstrate this

**Key Concepts:**
- `loop` is Rust's infinite loop (no condition)
- `break` exits, `continue` skips to next iteration
- `loop` with `break value` is an expression that returns a value

---

## Task 22 — Print 1–100 Using `while`

**Goal:** Use condition-based looping.

**Instructions:**
1. Use `while counter <= 100 { }` to print numbers
2. Implement countdown from 100 to 1 using while
3. Use `while let Some(val) = stack.pop()` to drain a vector (while-let pattern)

**Key Concepts:**
- `while condition { }` — evaluates condition each iteration
- `while let` — combines while with pattern matching

---

## Task 23 — Print 1–100 Using `for`

**Goal:** Use iterator-based looping — the preferred Rust style.

**Instructions:**
1. Use `for i in 1..=100` to print numbers
2. Iterate a vector by reference: `for item in &numbers`
3. Use `.enumerate()`: `for (index, value) in numbers.iter().enumerate()`
4. Use `.rev()` to iterate in reverse
5. Use `(1..=100).step_by(5)` to step by 5

**Key Concepts:**
- `1..100` is exclusive end; `1..=100` is inclusive end
- `for` always iterates over something that implements `Iterator`
- `&numbers` iterates by reference (no ownership transfer)

---

## Task 24 — FizzBuzz

**Goal:** Practice conditional logic with pattern matching.

**Instructions:**
1. Create `task_24_fizzbuzz`
2. Implement classic FizzBuzz for 1–100
3. Write it two ways:
   - Using `if/else if/else`
   - Using `match (n % 3, n % 5) { (0, 0) => "FizzBuzz", ... }`
4. Store results in a `Vec<String>` and print all at once

**Key Concepts:**
- `match` on tuples
- `_` wildcard in match arms
- `match` is exhaustive — all cases must be covered

---

## Task 25 — Fibonacci Series

**Goal:** Practice iterative and recursive approaches.

**Instructions:**
1. Create `task_25_fibonacci`
2. Write iterative `fn fibonacci_iter(n: u32) -> u64`
3. Write recursive `fn fibonacci_rec(n: u32) -> u64`
4. Write a version using a memoization `HashMap`
5. Print first 20 Fibonacci numbers, comparing performance
6. Notice overflow at large values — use `u128` or add overflow checking

**Key Concepts:**
- Recursive functions in Rust (no special syntax required)
- `HashMap<u32, u64>` for memoization
- Integer overflow — Rust panics on overflow in debug, wraps in release

---

## Task 26 — Factorial Using Loop

**Goal:** Practice accumulator pattern.

**Instructions:**
1. Write `fn factorial_loop(n: u64) -> u64`
2. Start with accumulator = 1, multiply up to n
3. Handle edge case: 0! = 1
4. Test up to 20! (max for u64 is 20!)
5. For larger numbers, use a `Vec` of digits or the `num-bigint` crate

---

## Task 27 — Factorial Using Recursion

**Goal:** Understand recursive function structure and stack.

**Instructions:**
1. Write `fn factorial_rec(n: u64) -> u64`
2. Base case: `if n == 0 { return 1; }`
3. Recursive case: `n * factorial_rec(n - 1)`
4. Note: Rust does not optimize tail calls — deep recursion will stack overflow
5. Write a tail-recursive version using an accumulator parameter

---

## Task 28 — GCD Calculator

**Goal:** Implement the Euclidean algorithm.

**Instructions:**
1. Write `fn gcd(mut a: u64, mut b: u64) -> u64`
2. Use the Euclidean algorithm: while b != 0, swap (a, b) with (b, a % b)
3. Test with several pairs
4. Verify: `gcd(48, 18) = 6`, `gcd(100, 75) = 25`

---

## Task 29 — LCM Calculator

**Goal:** Build on GCD using the mathematical relationship.

**Instructions:**
1. Write `fn lcm(a: u64, b: u64) -> u64` using `a * b / gcd(a, b)`
2. Handle overflow for large numbers using `u128`
3. Write `fn lcm_of_vec(numbers: &[u64]) -> u64` using `.fold()`

**Key Concepts:**
- `.fold(initial, |acc, &x| operation)` — reduce a collection to a single value
- Slices `&[u64]` — reference to a contiguous sequence (more flexible than `&Vec`)

---

## Task 30 — Number Guessing Game

**Goal:** Build a complete interactive program combining loops, I/O, and conditionals.

**Instructions:**
1. Create `task_30_guessing_game`
2. Add `rand` crate to `Cargo.toml`: `rand = "0.8"`
3. Generate a random number between 1 and 100: `rand::thread_rng().gen_range(1..=100)`
4. Loop until the user guesses correctly:
   - Read input, parse to `i32`
   - Print "Too high", "Too low", or "Correct!"
   - Count the number of attempts
5. Print the attempt count on success
6. Handle invalid input gracefully (non-numeric entry)

**Key Concepts:**
- Adding external crates via `Cargo.toml`
- `use` statements for imports
- Combining `loop`, `break`, I/O, and match

---

## Task 31 — Triangle Pattern

**Goal:** Practice nested loops and string repetition.

**Instructions:**
Print this pattern for n=5:
```
*
**
***
****
*****
```
Use `"*".repeat(i)` and also print with spaces for a right-aligned version.

---

## Task 32 — Reverse Triangle

**Goal:** Reverse loop direction.

**Instructions:**
Print:
```
*****
****
***
**
*
```
Use `(1..=n).rev()` or count down from n.

---

## Task 33 — Pyramid Pattern

**Goal:** Combine forward and backward loops.

**Instructions:**
Print for n=5:
```
    *
   ***
  *****
 *******
*********
```
Each row i has `(n-i)` spaces and `(2*i-1)` stars.

---

## Task 34 — Diamond Pattern

**Goal:** Combine Tasks 31, 32, 33.

**Instructions:**
Print a diamond shape for n=5 (top pyramid + inverted pyramid without repeating the middle row). Use helper functions for top half and bottom half.

---

## Task 35 — Pascal's Triangle

**Goal:** Practice 2D vectors and combinatorics.

**Instructions:**
1. Create `task_35_pascal`
2. Use `Vec<Vec<u64>>` to represent the triangle
3. Row 0 = [1], Row 1 = [1,1], each subsequent row: first and last are 1, middle elements are sum of two above
4. Build `fn pascal_triangle(rows: usize) -> Vec<Vec<u64>>`
5. Print each row centered

**Key Concepts:**
- `Vec<Vec<T>>` — vector of vectors (2D structure)
- Indexing: `triangle[row][col]`

---

# PHASE 3: String Mastery (Tasks 36–55)

---

## Task 36 — Reverse a String

**Goal:** Understand Rust strings are UTF-8 and reversing requires care.

**Instructions:**
1. Create `task_36_reverse_string`
2. Write `fn reverse_string(s: &str) -> String`
3. Use `.chars().rev().collect::<String>()`
4. **Do not** just reverse bytes — that breaks multi-byte Unicode characters
5. Test with ASCII and a Unicode string (e.g., "héllo")

**Key Concepts:**
- `&str` is a string slice — a reference to UTF-8 data
- `.chars()` iterates Unicode scalar values
- `.collect::<String>()` gathers an iterator into a String
- Strings are NOT arrays of bytes in the human-readable sense

**Python Comparison:**
```python
s[::-1]  # Python — works on Unicode
```
Rust's `s.bytes().rev()` would break Unicode. Always use `.chars()`.

---

## Task 37 — Palindrome Checker

**Goal:** Practice string comparison after transformation.

**Instructions:**
1. Write `fn is_palindrome(s: &str) -> bool`
2. Compare the string to its reverse (use your Task 36 function)
3. Write a case-insensitive version
4. Write a version that ignores non-alphanumeric characters
5. Test with: "racecar", "A man a plan a canal Panama", "hello"

**Key Concepts:**
- `.to_lowercase()` returns a `String` (allocates)
- `.chars().filter(|c| c.is_alphanumeric())`

---

## Task 38 — Count Vowels

**Goal:** Practice iterating over characters with conditions.

**Instructions:**
1. Write `fn count_vowels(s: &str) -> usize`
2. Count a, e, i, o, u (case-insensitive)
3. Use `.chars().filter(|c| "aeiou".contains(*c)).count()`
4. Also count each vowel separately, returning a `HashMap<char, usize>`

---

## Task 39 — Count Consonants

**Goal:** Build on Task 38.

**Instructions:**
1. Write `fn count_consonants(s: &str) -> usize`
2. Count alphabetic characters that are not vowels
3. Use `c.is_alphabetic() && !"aeiou".contains(c)`

---

## Task 40 — Count Words

**Goal:** Practice string splitting.

**Instructions:**
1. Write `fn count_words(s: &str) -> usize`
2. Use `.split_whitespace().count()`
3. Handle multiple spaces, tabs, and newlines correctly
4. Write a version that splits on a custom delimiter

**Key Concepts:**
- `.split_whitespace()` handles any whitespace
- `.split(',')` splits on a specific character
- `.split_terminator(pattern)` for more control

---

## Task 41 — Find Longest Word

**Goal:** Practice `.max_by_key()` on iterators.

**Instructions:**
1. Write `fn longest_word(s: &str) -> &str`
2. Split on whitespace, find the word with maximum length
3. Use `.split_whitespace().max_by_key(|w| w.len())`
4. Return the word (a slice of the original string — no allocation!)
5. Handle empty strings with `Option<&str>`

**Key Concepts:**
- Returning a string slice `&str` that borrows from the input
- `.max_by_key()` and `.min_by_key()`

---

## Task 42 — Find Shortest Word

**Goal:** Mirror of Task 41.

**Instructions:**
Write `fn shortest_word(s: &str) -> Option<&str>` using `.min_by_key()`.

---

## Task 43 — Remove Duplicate Characters

**Goal:** Practice `HashSet` for deduplication while preserving order.

**Instructions:**
1. Write `fn remove_duplicates(s: &str) -> String`
2. Use a `HashSet<char>` to track seen characters
3. Iterate chars, only push to result if not in the set
4. Preserve original order (unlike collecting into a HashSet directly)

**Key Concepts:**
- `HashSet<T>` for O(1) membership testing
- `.insert()` returns `bool` — true if it was newly inserted

---

## Task 44 — Character Frequency Counter

**Goal:** Build a HashMap-based frequency table.

**Instructions:**
1. Write `fn char_frequency(s: &str) -> HashMap<char, usize>`
2. Use `*count.entry(c).or_insert(0) += 1`
3. Sort the results by frequency (descending) and print top 5 most common characters

**Key Concepts:**
- `HashMap::entry().or_insert()` — the idiomatic way to count
- Sorting a Vec of tuples by `.sort_by(|a, b| b.1.cmp(&a.1))`

---

## Task 45 — String Compression

**Goal:** Implement run-length encoding.

**Instructions:**
1. Write `fn compress(s: &str) -> String`
2. "aabbbcccc" → "a2b3c4"
3. If a char appears once, just write the char (no "1")
4. Handle empty input

---

## Task 46 — String Decompression

**Goal:** Parse and expand run-length encoded strings.

**Instructions:**
1. Write `fn decompress(s: &str) -> String`
2. "a2b3c4" → "aabbbcccc"
3. Parse pairs of (letter, count) where count may be absent (= 1)
4. Use `.chars().peekable()` to look ahead

**Key Concepts:**
- `Peekable` iterator: `.peekable()` lets you look at the next element without consuming it

---

## Task 47 — Caesar Cipher

**Goal:** Practice char arithmetic.

**Instructions:**
1. Write `fn caesar_encrypt(text: &str, shift: u8) -> String`
2. Write `fn caesar_decrypt(text: &str, shift: u8) -> String`
3. Shift only alphabetic characters; preserve case; leave others unchanged
4. Shift wraps around: 'z' + 1 = 'a'
5. Formula: `((c - b'a' + shift) % 26 + b'a') as char`

**Key Concepts:**
- `b'a'` is the byte value of 'a' (97)
- Char arithmetic via byte values
- `.is_ascii_alphabetic()`, `.is_ascii_uppercase()`

---

## Task 48 — ROT13

**Goal:** Special case of Caesar cipher (shift = 13).

**Instructions:**
1. Write `fn rot13(s: &str) -> String`
2. ROT13 is its own inverse: `rot13(rot13(s)) == s`
3. Implement using your Caesar cipher, or use a lookup approach
4. Verify the inverse property

---

## Task 49 — Anagram Checker

**Goal:** Practice sorting strings for comparison.

**Instructions:**
1. Write `fn is_anagram(a: &str, b: &str) -> bool`
2. Two words are anagrams if they contain the same characters with the same frequencies
3. Approach 1: sort both char arrays and compare
4. Approach 2: use two `HashMap<char, i32>` and compare them
5. Case-insensitive comparison

---

## Task 50 — Email Validator

**Goal:** Practice string parsing without regex.

**Instructions:**
1. Write `fn is_valid_email(email: &str) -> bool`
2. Rules: must contain exactly one `@`, local part before `@` is non-empty, domain after `@` contains at least one `.`, domain parts are non-empty
3. Use `.contains('@')`, `.split_once('@')`, `.split('.')`
4. Return detailed error messages using a custom enum

**Key Concepts:**
- `.split_once(delimiter)` — splits at first occurrence, returns `Option<(&str, &str)>`
- Pattern matching on `Option`

---

## Task 51 — URL Parser

**Goal:** Parse structured strings into components.

**Instructions:**
1. Create a struct:
   ```rust
   struct ParsedUrl {
       scheme: String,
       host: String,
       path: String,
       query: Option<String>,
   }
   ```
2. Write `fn parse_url(url: &str) -> Option<ParsedUrl>`
3. Parse: `https://example.com/path?query=value`
4. Split on `://`, then `/`, then `?`

---

## Task 52 — Markdown Heading Extractor

**Goal:** Process a multi-line string, filtering lines by prefix.

**Instructions:**
1. Write `fn extract_headings(markdown: &str) -> Vec<(usize, String)>`
2. Return (level, text) pairs: `## Hello` → `(2, "Hello")`
3. Count leading `#` characters for level
4. Strip the `# ` prefix from text
5. Test with a multi-line string literal using `r#"..."#`

---

## Task 53 — CSV Parser

**Goal:** Parse tabular data from a string.

**Instructions:**
1. Write `fn parse_csv(input: &str) -> Vec<Vec<String>>`
2. Split on newlines, then on commas
3. Trim whitespace from each field
4. Write `fn csv_to_structs(csv: &str) -> Vec<Student>` where Student has name, age, grade
5. Handle the header row

---

## Task 54 — JSON Parser Using serde

**Goal:** Learn Rust's dominant serialization library.

**Instructions:**
1. Create `task_54_json`
2. Add to `Cargo.toml`:
   ```toml
   serde = { version = "1.0", features = ["derive"] }
   serde_json = "1.0"
   ```
3. Define a struct with `#[derive(Serialize, Deserialize, Debug)]`
4. Parse a JSON string: `let user: User = serde_json::from_str(json_str)?`
5. Serialize to JSON: `let json = serde_json::to_string_pretty(&user)?`
6. Handle nested objects and arrays

**Key Concepts:**
- Derive macros — code generated at compile time
- `?` operator for error propagation (preview of Phase 7)
- `serde_json::Value` for dynamic/unknown JSON

---

## Task 55 — Mini Grep

**Goal:** Build a simple file search tool.

**Instructions:**
1. Create `task_55_mini_grep`
2. Read a search pattern and filename from `std::env::args()`
3. Read the file with `std::fs::read_to_string()`
4. Filter lines containing the pattern
5. Print matching lines with line numbers
6. Add a `-i` flag for case-insensitive search

**Key Concepts:**
- `std::env::args()` — command-line arguments
- `std::fs::read_to_string()` — reads entire file to String
- Printing to stderr for errors: `eprintln!`

---

# PHASE 4: Arrays, Vectors & Collections (Tasks 56–75)

---

## Task 56 — Find Maximum in Array

**Goal:** Practice slices and iterators.

**Instructions:**
1. Write `fn find_max(arr: &[i32]) -> Option<i32>`
2. Use `.iter().max().copied()`
3. Implement manually using a fold: `.iter().fold(i32::MIN, |acc, &x| acc.max(x))`
4. Compare: when would you use manual vs iterator methods?

**Key Concepts:**
- `&[i32]` is a slice — works for both arrays `[i32; N]` and vectors `Vec<i32>`
- `i32::MIN` and `i32::MAX` — min/max values for the type
- `.copied()` turns `Option<&i32>` into `Option<i32>`

---

## Task 57 — Find Minimum in Array

**Goal:** Mirror of Task 56.

**Instructions:**
Write `fn find_min(arr: &[i32]) -> Option<i32>`.

---

## Task 58 — Calculate Average

**Goal:** Practice iterator methods for aggregation.

**Instructions:**
1. Write `fn average(arr: &[f64]) -> Option<f64>`
2. Return `None` for empty slices
3. Use `.iter().sum::<f64>()` divided by `.len() as f64`
4. Also calculate variance and standard deviation

---

## Task 59 — Rotate Array Left

**Goal:** Practice slice operations.

**Instructions:**
1. Write `fn rotate_left(v: &mut Vec<i32>, k: usize)`
2. Rotate by k positions: [1,2,3,4,5] left by 2 → [3,4,5,1,2]
3. Use `v.rotate_left(k % v.len())`
4. Implement manually using a temporary vector

---

## Task 60 — Rotate Array Right

**Goal:** Mirror of Task 59.

**Instructions:**
Write `fn rotate_right(v: &mut Vec<i32>, k: usize)` using `v.rotate_right()`.

---

## Task 61 — Remove Duplicates from Sorted Array

**Goal:** Practice deduplication algorithms.

**Instructions:**
1. Write `fn remove_duplicates_sorted(v: &mut Vec<i32>)` using `v.dedup()`
2. Write `fn remove_duplicates_unsorted(v: Vec<i32>) -> Vec<i32>` using a HashSet
3. Note: `dedup()` only removes consecutive duplicates — sort first for full dedup

---

## Task 62 — Merge Two Sorted Arrays

**Goal:** Implement merge step from merge sort.

**Instructions:**
1. Write `fn merge_sorted(a: &[i32], b: &[i32]) -> Vec<i32>`
2. Use two pointers, compare and advance
3. Append remaining elements of whichever array has leftovers
4. The result should be sorted

---

## Task 63 — Find Second Largest

**Goal:** Single-pass algorithm design.

**Instructions:**
1. Write `fn second_largest(arr: &[i32]) -> Option<i32>`
2. Track both largest and second largest in a single pass
3. Return None if the array has fewer than 2 elements or all elements are equal

---

## Task 64 — Find Missing Number

**Goal:** Apply the sum formula trick.

**Instructions:**
1. Given a slice of n-1 distinct numbers from 1 to n, find the missing one
2. Write `fn find_missing(arr: &[i32], n: i32) -> i32`
3. Expected sum = n*(n+1)/2; actual sum = arr.iter().sum(); missing = expected - actual
4. Also implement using XOR for practice

---

## Task 65 — Two Sum Problem

**Goal:** Classic interview question; practice HashMap for O(n) solution.

**Instructions:**
1. Write `fn two_sum(nums: &[i32], target: i32) -> Option<(usize, usize)>`
2. Naive O(n²): nested loops
3. Optimal O(n): use `HashMap<i32, usize>` mapping value → index
4. For each number, check if `target - number` exists in the map

---

## Task 66 — Frequency Counter Using HashMap

**Goal:** Apply HashMap to counting problems.

**Instructions:**
1. Write `fn frequency_count(data: &[i32]) -> HashMap<i32, usize>`
2. Find the most frequent element
3. Find elements appearing exactly once
4. Sort by frequency for a leaderboard

---

## Task 67 — Word Frequency Analyzer

**Goal:** Apply frequency counting to text.

**Instructions:**
1. Write `fn word_frequency(text: &str) -> Vec<(String, usize)>`
2. Normalize: lowercase, remove punctuation
3. Return sorted by frequency (descending)
4. Print top 10 most common words

---

## Task 68 — Student Grade System

**Goal:** Model data and compute statistics.

**Instructions:**
1. Create:
   ```rust
   struct Student {
       name: String,
       scores: Vec<f64>,
   }
   ```
2. Implement methods: `.average()`, `.grade()` (A/B/C/D/F), `.highest()`, `.lowest()`
3. Build a `Vec<Student>` and print a report sorted by average

---

## Task 69 — Inventory Management

**Goal:** HashMap with struct values.

**Instructions:**
1. Create:
   ```rust
   struct Product {
       name: String,
       price: f64,
       quantity: u32,
   }
   ```
2. Use `HashMap<String, Product>` as inventory
3. Write functions: `add_product`, `remove_product`, `update_quantity`, `total_value`
4. Print a formatted inventory report

---

## Task 70 — Contact Manager

**Goal:** Full CRUD operations on a collection.

**Instructions:**
1. Create:
   ```rust
   struct Contact {
       name: String,
       phone: String,
       email: String,
   }
   ```
2. Implement: add, remove, search by name, update, list all (sorted by name)
3. Use `Vec<Contact>` or `HashMap<String, Contact>`

---

## Task 71 — Custom Vec Wrapper

**Goal:** Understand newtype pattern and method delegation.

**Instructions:**
1. Create `struct NumberList(Vec<i32>)`
2. Implement methods: `push`, `pop`, `sum`, `mean`, `median`, `std_dev`
3. Implement `Display` trait for pretty printing
4. This is the newtype pattern — wrap a type to add behavior

**Key Concepts:**
- Newtype pattern: `struct Wrapper(InnerType)`
- Access inner with `.0`
- Implementing traits for your types

---

## Task 72 — Implement a Stack

**Goal:** Build a generic stack from scratch.

**Instructions:**
1. Create `struct Stack<T> { data: Vec<T> }`
2. Implement: `push(&mut self, item: T)`, `pop(&mut self) -> Option<T>`, `peek(&self) -> Option<&T>`, `is_empty(&self) -> bool`, `size(&self) -> usize`
3. Make it generic over any type `T`
4. Test with `Stack<i32>` and `Stack<String>`

---

## Task 73 — Implement a Queue

**Goal:** Build a FIFO queue.

**Instructions:**
1. Create `struct Queue<T> { data: VecDeque<T> }`
2. Use `std::collections::VecDeque` for efficient front/back operations
3. Implement: `enqueue`, `dequeue`, `peek`, `is_empty`, `size`

---

## Task 74 — Implement a Deque

**Goal:** Double-ended queue with both-end operations.

**Instructions:**
1. Wrap `VecDeque<T>` with push/pop operations for both front and back
2. Demonstrate with a palindrome checker using the deque

---

## Task 75 — Circular Queue

**Goal:** Fixed-size queue with wrap-around indexing.

**Instructions:**
1. Create `struct CircularQueue<T> { data: Vec<Option<T>>, head: usize, tail: usize, size: usize, capacity: usize }`
2. Implement enqueue (fail if full), dequeue (fail if empty)
3. Use modulo arithmetic for wrap-around: `self.tail = (self.tail + 1) % self.capacity`

---

# PHASE 5: Structs & Enums (Tasks 76–90)

---

## Task 76 — User Struct

**Goal:** Master struct definition, instantiation, and methods.

**Instructions:**
1. Define:
   ```rust
   struct User {
       username: String,
       email: String,
       age: u32,
       is_active: bool,
   }
   ```
2. Implement a `new` constructor
3. Add methods: `deactivate(&mut self)`, `change_email(&mut self, new_email: String)`
4. Implement `Display` trait for formatted output
5. Use struct update syntax: `let user2 = User { email: String::from("new@mail.com"), ..user1 };`

**Key Concepts:**
- `impl StructName { }` block for methods
- `&self` for read-only methods, `&mut self` for mutating methods
- Associated functions (no `self`): `User::new(...)` — like static methods in Python

---

## Task 77 — Product Struct

**Goal:** Practice with floating point and optional fields.

**Instructions:**
1. Define `struct Product` with: name, price, category, discount (optional: `Option<f64>`)
2. Implement: `final_price(&self) -> f64` (applies discount if present), `apply_discount(&mut self, pct: f64)`
3. Derive `Debug` and `Clone`

---

## Task 78 — Employee Struct

**Goal:** Practice struct with multiple data types.

**Instructions:**
1. Define `struct Employee` with: id, name, department, salary, hire_date (as a String)
2. Implement: `annual_salary(&self) -> f64`, `raise(&mut self, percent: f64)`, `years_of_service(&self) -> u32` (hard-code current year)

---

## Task 79 — Student Struct

**Goal:** Struct with a Vec field.

**Instructions:**
1. Define `struct Student` with: name, id, courses (`Vec<String>`), grades (`HashMap<String, f64>`)
2. Implement: `enroll(&mut self, course: String)`, `add_grade(&mut self, course: &str, grade: f64)`, `gpa(&self) -> f64`

---

## Task 80 — Library Book Struct

**Goal:** Optional fields and enums.

**Instructions:**
1. Define:
   ```rust
   enum BookStatus { Available, CheckedOut { due_date: String }, Reserved }
   struct Book { isbn: String, title: String, author: String, status: BookStatus }
   ```
2. Implement: `checkout(&mut self, due: String)`, `return_book(&mut self)`, `is_available(&self) -> bool`

---

## Task 81 — Library Management System

**Goal:** Build a complete CRUD system with multiple structs.

**Instructions:**
1. Combine `Book`, `Member` (struct with id, name, `Vec<String>` for borrowed ISBN's)
2. Build `Library` struct with `HashMap<String, Book>` and `HashMap<String, Member>`
3. Implement: `add_book`, `remove_book`, `checkout_book(member_id, isbn)`, `return_book(member_id, isbn)`, `search_by_title`, `list_available`
4. Use proper error handling: return `Result<(), LibraryError>` from operations

---

## Task 82 — Banking System

**Goal:** Model financial transactions with error handling.

**Instructions:**
1. Create `struct Account { id: u64, owner: String, balance: f64, transactions: Vec<Transaction> }`
2. Create `struct Transaction { amount: f64, kind: TransactionKind, timestamp: String }`
3. Create `enum TransactionKind { Deposit, Withdrawal, Transfer { to_account: u64 } }`
4. Implement: `deposit`, `withdraw` (fail if insufficient funds), `transfer`, `statement`
5. All mutating methods return `Result<(), BankError>`

---

## Task 83 — Expense Tracker

**Goal:** Practice date-like strings, categories, and reporting.

**Instructions:**
1. Define `struct Expense { amount: f64, category: ExpenseCategory, description: String, date: String }`
2. Define `enum ExpenseCategory { Food, Transport, Housing, Entertainment, Other }`
3. Build `ExpenseTracker` with a `Vec<Expense>`
4. Implement: `add_expense`, `total_by_category`, `monthly_report`, `largest_expense`, `filter_by_category`

---

## Task 84 — Todo App

**Goal:** Classic project with full CRUD and status management.

**Instructions:**
1. Define:
   ```rust
   enum Priority { Low, Medium, High }
   enum Status { Todo, InProgress, Done }
   struct TodoItem { id: u32, title: String, priority: Priority, status: Status, tags: Vec<String> }
   ```
2. Build `TodoList` with: `add`, `remove`, `update_status`, `list_by_status`, `list_by_priority`
3. Implement `Display` for each type

---

## Task 85 — Task Scheduler

**Goal:** Time-based ordering and priority queues.

**Instructions:**
1. Define `struct Task { id: u32, name: String, scheduled_at: String, priority: u8, recurring: bool }`
2. Build `Scheduler` with a sorted `Vec<Task>` (sorted by scheduled_at)
3. Implement: `schedule`, `cancel`, `next_task`, `list_upcoming(n: usize)`

---

## Task 86 — Enum for Task States (State Machine)

**Goal:** Use enums to model valid state transitions.

**Instructions:**
1. Define:
   ```rust
   enum TaskState { Created, Assigned { to: String }, InProgress { started: String }, Done { completed: String }, Cancelled { reason: String } }
   ```
2. Implement transitions: `assign(to: String) -> Result<TaskState, String>`, `start() -> Result<...>`, `complete() -> Result<...>`, `cancel(reason: String) -> Result<...>`
3. Enforce that you can only start an Assigned task, only complete an InProgress task, etc.

---

## Task 87 — Enum for HTTP Methods

**Goal:** Use enums with data.

**Instructions:**
1. Define `enum HttpMethod { GET, POST, PUT, DELETE, PATCH }`
2. Add methods: `is_safe(&self) -> bool` (GET is safe), `is_idempotent(&self) -> bool`
3. Implement `Display` and `FromStr` (parsing "GET" → `HttpMethod::GET`)

---

## Task 88 — Enum for Calculator Operations

**Goal:** Data-carrying enums.

**Instructions:**
1. Define `enum Operation { Add(f64, f64), Sub(f64, f64), Mul(f64, f64), Div(f64, f64), Pow(f64, f64) }`
2. Write `fn calculate(op: Operation) -> Result<f64, String>`
3. Handle division by zero
4. Demonstrate pattern matching exhaustiveness

---

## Task 89 — Enum for Payment Types

**Goal:** Complex data-carrying enums.

**Instructions:**
1. Define:
   ```rust
   enum PaymentMethod {
       CreditCard { number: String, expiry: String, cvv: String },
       BankTransfer { account: String, routing: String },
       Cash { amount_tendered: f64 },
       Cryptocurrency { wallet: String, currency: String },
   }
   ```
2. Write `fn process_payment(method: &PaymentMethod, amount: f64) -> Result<String, String>`

---

## Task 90 — State Machine Using Enums

**Goal:** Model a traffic light or order lifecycle.

**Instructions:**
1. Define `enum OrderState { Pending, Confirmed, Shipped { tracking: String }, Delivered, Cancelled }`
2. Implement `Order` struct with a `state: OrderState`
3. Methods: `confirm()`, `ship(tracking: String)`, `deliver()`, `cancel()`
4. Each method should only work from valid preceding states — return `Err` otherwise
5. Implement `Display` for each state

---

# PHASE 6: Traits & Generics (Tasks 91–105)

---

## Task 91 — Printable Trait

**Goal:** Define and implement a custom trait.

**Instructions:**
1. Define `trait Printable { fn print(&self); fn print_with_label(&self, label: &str) { print!("{}: ", label); self.print(); } }`
2. Implement for `i32`, `String`, and your `User` struct from Task 76
3. Note the default method — `print_with_label` works for all implementors without extra code

**Key Concepts:**
- Traits define shared behavior (like Python's abstract base classes, but enforced at compile time)
- Default method implementations
- `impl TraitName for TypeName { }`

---

## Task 92 — Area Trait

**Goal:** Polymorphism through traits.

**Instructions:**
1. Define `trait Shape { fn area(&self) -> f64; fn name(&self) -> &str; }`
2. Implement for `Circle { radius: f64 }`, `Rectangle { width: f64, height: f64 }`, `Triangle { base: f64, height: f64 }`
3. Create a `Vec<Box<dyn Shape>>` and call `area()` on each — this is dynamic dispatch

**Key Concepts:**
- `dyn Trait` — dynamic dispatch (runtime polymorphism)
- `Box<dyn Trait>` — heap-allocated trait object
- Static dispatch (generics) vs dynamic dispatch (`dyn`) — trade-offs

---

## Task 93 — Perimeter Trait

**Goal:** Multiple trait implementations.

**Instructions:**
1. Define `trait Perimeter { fn perimeter(&self) -> f64; }`
2. Implement for the same shapes
3. Write a function `fn describe(shape: &(impl Shape + Perimeter))` — accepts types implementing both traits

**Key Concepts:**
- `impl Trait` in function parameters (anonymous generic)
- Trait bounds: `T: Shape + Perimeter`
- `+` to require multiple traits

---

## Task 94 — Generic Swap Function

**Goal:** Write your first generic function.

**Instructions:**
1. Write `fn swap<T>(a: &mut T, b: &mut T)` using a temporary
2. Call with `i32`, `String`, and `Vec<f64>`
3. Verify it works for any type without needing `Copy`

**Key Concepts:**
- `<T>` declares a type parameter
- Generic functions work with any type that satisfies the bounds
- Monomorphization: Rust generates a concrete version for each type used at compile time (zero overhead)

---

## Task 95 — Generic Max Function

**Goal:** Use trait bounds to constrain generics.

**Instructions:**
1. Write `fn max_of<T: PartialOrd>(a: T, b: T) -> T`
2. `PartialOrd` is the trait that provides `<` and `>`
3. Call with `i32`, `f64`, and `char`
4. Try calling with a type that doesn't implement `PartialOrd` — observe the error

---

## Task 96 — Generic Stack

**Goal:** Apply generics to your Task 72 Stack.

**Instructions:**
1. Refactor `Stack<T>` to work with any type (it likely already does if you used `Vec<T>`)
2. Add a bound for printing: `fn print_all(&self) where T: Display`
3. Demonstrate with multiple types

---

## Task 97 — Generic Queue

**Goal:** Same as 96 for Queue.

---

## Task 98 — Generic Linked List

**Goal:** Implement a linked list — famous for ownership challenges.

**Instructions:**
1. Define:
   ```rust
   enum List<T> {
       Cons(T, Box<List<T>>),
       Nil,
   }
   ```
2. Implement: `new() -> List<T>`, `push(self, val: T) -> List<T>`, `len(&self) -> usize`
3. Implement an iterator for the list
4. Explain in comments why `Box` is needed (recursive types need known size)

**Key Concepts:**
- `Box<T>` — heap allocation; used to make recursive types finite-sized
- Recursive data structures
- This is a learning exercise — use `Vec` or `std::collections::LinkedList` in real code

---

## Task 99 — Custom Iterator

**Goal:** Implement the `Iterator` trait.

**Instructions:**
1. Create `struct Counter { count: u32, max: u32 }`
2. Implement `Iterator for Counter { type Item = u32; fn next(&mut self) -> Option<u32> { ... } }`
3. Once you implement `next`, you get `.map()`, `.filter()`, `.sum()`, `.collect()` for free
4. Chain methods: `Counter::new(5).filter(|x| x % 2 == 0).sum::<u32>()`

**Key Concepts:**
- `Iterator` trait only requires `next()` — all other methods have default implementations
- Associated type: `type Item`
- This is how `for` loops work internally

---

## Task 100 — Implement Display Trait

**Goal:** Control how your types print.

**Instructions:**
1. Implement `std::fmt::Display` for your `User`, `Product`, `TodoItem` structs
2. Format: `impl fmt::Display for User { fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "User({})", self.username) } }`
3. `Display` is what `{}` uses; `Debug` is what `{:?}` uses

---

## Task 101 — Implement Debug Trait

**Goal:** Derive vs manual Debug implementation.

**Instructions:**
1. Use `#[derive(Debug)]` on a simple struct — see the auto-generated output
2. Manually implement `Debug` for a struct with sensitive fields (e.g., hide password)
3. `{:?}` uses Debug, `{:#?}` pretty-prints

---

## Task 102 — Implement From Trait

**Goal:** Enable type conversions.

**Instructions:**
1. Implement `From<String> for Email` (a newtype wrapping String)
2. Implement `From<i32> for Temperature`
3. Once `From` is implemented, you get `Into` for free: `let e: Email = "test@mail.com".to_string().into();`

---

## Task 103 — Implement Into Trait

**Goal:** Understand the From/Into relationship.

**Instructions:**
1. Write a function `fn send_email(to: impl Into<String>)` — accepts anything convertible to String
2. Call with `&str`, `String`, and custom types implementing `Into<String>`

---

## Task 104 — Implement Default Trait

**Goal:** Provide sensible defaults.

**Instructions:**
1. Use `#[derive(Default)]` on a struct where all fields implement `Default`
2. Manually implement `Default` for a struct needing custom defaults
3. Use `..Default::default()` in struct initialization to fill unspecified fields

---

## Task 105 — Plugin System Using Traits

**Goal:** Real-world use of trait objects.

**Instructions:**
1. Define `trait Processor { fn name(&self) -> &str; fn process(&self, input: &str) -> String; }`
2. Create: `UppercaseProcessor`, `ReverseProcessor`, `WordCountProcessor`
3. Build a `Pipeline { processors: Vec<Box<dyn Processor>> }` that chains them
4. Run input through all processors in sequence

---

# PHASE 7: Error Handling (Tasks 106–120)

---

## Task 106 — Parse Integer Safely

**Goal:** Stop using `.unwrap()` and handle errors properly.

**Instructions:**
1. `"42".parse::<i32>()` returns `Result<i32, ParseIntError>`
2. Handle with `match`: `Ok(n) => ..., Err(e) => ...`
3. Handle with `.unwrap_or(0)` (default on error)
4. Handle with `.unwrap_or_else(|e| { eprintln!("Error: {}", e); 0 })`
5. Use the `?` operator to propagate: works inside functions returning `Result`

**Key Concepts:**
- `Result<T, E>` — either `Ok(T)` or `Err(E)`
- `Option<T>` — either `Some(T)` or `None`
- `?` operator: if `Err`, returns early from function; if `Ok`, unwraps
- `.unwrap()` panics on error — only acceptable in examples/tests

---

## Task 107 — Read File Safely

**Goal:** Chain multiple fallible operations.

**Instructions:**
1. `std::fs::read_to_string("file.txt")` returns `Result<String, io::Error>`
2. Handle: file not found, permission error
3. Write a function returning `Result<String, io::Error>` and use `?` to propagate
4. Use `.map_err(|e| format!("Failed to read: {}", e))` to add context

---

## Task 108 — Custom Error Type

**Goal:** Define your own error types.

**Instructions:**
1. Create an enum:
   ```rust
   #[derive(Debug)]
   enum AppError {
       NotFound(String),
       ParseError(String),
       IoError(std::io::Error),
   }
   ```
2. Implement `std::fmt::Display` for `AppError`
3. Implement `std::error::Error` for `AppError`
4. Implement `From<std::io::Error> for AppError` — enables `?` to auto-convert

---

## Task 109 — Error Propagation with `?`

**Goal:** Write clean error-propagating code.

**Instructions:**
1. Write a function that reads a file, parses each line as an integer, and returns the sum
2. Signature: `fn sum_from_file(path: &str) -> Result<i64, AppError>`
3. Use `?` after every fallible operation — no match/unwrap needed
4. Notice how clean the happy path looks

---

## Task 110 — Recoverable Error Example

**Goal:** Handle errors and continue execution.

**Instructions:**
1. Write a program that reads multiple files
2. For each file, attempt to read and process it
3. On error, log the error and continue to the next file
4. Collect both successes and failures, report at the end

---

## Task 111 — Non-Recoverable Error Example

**Goal:** Know when to panic.

**Instructions:**
1. Use `panic!("message")` intentionally in a case that "should never happen"
2. Use `assert!(condition, "message")` for invariant checking
3. Use `.expect("context message")` instead of `.unwrap()` in production code
4. Learn: panics are for programmer errors; `Result` is for expected failures

---

## Task 112 — CLI Calculator with Error Handling

**Goal:** Full application with robust error handling.

**Instructions:**
1. Read two numbers and an operator from command-line args or stdin
2. Parse the numbers — return error if not valid
3. Parse the operator — return error for unknown operator
4. Compute result — return error for division by zero
5. All errors use your custom `CalcError` enum
6. Main returns `Result<(), CalcError>` and uses `?` throughout

---

## Task 113 — CLI File Reader

**Goal:** Robust file processing with error reporting.

**Instructions:**
1. Accept a filename from args
2. Read the file, handle all possible errors with context
3. Process the file (count lines, words, characters — like `wc`)
4. Report errors clearly with `eprintln!` and exit with non-zero code using `std::process::exit(1)`

---

## Task 114 — Log Analyzer

**Goal:** Parse structured text with error handling.

**Instructions:**
1. Parse log lines of format: `[2024-01-15 10:23:45] ERROR module: message`
2. Create `struct LogEntry { timestamp: String, level: LogLevel, module: String, message: String }`
3. Return `Result<LogEntry, ParseError>` for each line
4. Filter by log level, count errors, find the most common error module

---

## Task 115 — Config Parser

**Goal:** Parse key-value configuration files.

**Instructions:**
1. Parse a `.env`-style file: `KEY=VALUE` per line
2. Support comments (lines starting with `#`)
3. Return `HashMap<String, String>`
4. Validate required keys — return error if missing
5. Type-convert values with proper errors

---

## Task 116 — Password Validator

**Goal:** Accumulate multiple errors rather than returning on first.

**Instructions:**
1. Write `fn validate_password(password: &str) -> Result<(), Vec<String>>`
2. Rules: min 8 chars, at least one uppercase, one lowercase, one digit, one special char
3. Collect ALL violations, not just the first one
4. Return `Ok(())` if valid, `Err(vec_of_messages)` if not

---

## Task 117 — Command Runner

**Goal:** Execute system commands from Rust.

**Instructions:**
1. Use `std::process::Command` to run shell commands
2. Write `fn run_command(cmd: &str, args: &[&str]) -> Result<String, String>`
3. Capture stdout and stderr separately
4. Return the output on success, error message on failure

---

## Task 118 — Backup Utility

**Goal:** Combine file operations with error handling.

**Instructions:**
1. Write a utility that copies files from a source directory to a backup directory
2. Add a timestamp to the backup filename
3. Handle: source not found, destination not writable, already backed up
4. Report success/failure for each file

---

## Task 119 — Zip Extractor

**Goal:** Use an external crate with Result-based APIs.

**Instructions:**
1. Add `zip = "0.6"` to Cargo.toml
2. Extract a zip file to a directory
3. Handle all error cases: file not found, invalid zip, write errors
4. Show a progress count: "Extracting file 3/10..."

---

## Task 120 — File Search Utility

**Goal:** Recursive directory traversal.

**Instructions:**
1. Write `fn search_files(dir: &str, pattern: &str) -> Result<Vec<PathBuf>, io::Error>`
2. Use `std::fs::read_dir()` to iterate directory contents
3. Recursively descend into subdirectories
4. Match filenames containing the pattern

---

# PHASE 8: File System Projects (Tasks 121–135)

---

## Task 121 — File Copy Utility

**Instructions:**
1. Use `std::fs::copy(src, dst)` — simple one-liner
2. Build a more sophisticated version that: shows progress for large files, preserves metadata, handles existing destination (prompt or overwrite flag)
3. Accept `--force` flag to overwrite without prompting

---

## Task 122 — File Move Utility

**Instructions:**
1. Try `std::fs::rename()` first (fast, same filesystem)
2. Fall back to copy+delete if rename fails (different filesystem)
3. Verify the copy succeeded before deleting the source

---

## Task 123 — File Rename Utility

**Instructions:**
1. Build a batch rename tool
2. Rename all files in a directory matching a pattern
3. Support: add prefix, add suffix, replace substring, change extension
4. Dry-run mode: show what would be renamed without doing it

---

## Task 124 — Directory Tree Printer

**Instructions:**
1. Print a tree like the Unix `tree` command:
   ```
   src/
   ├── main.rs
   ├── lib.rs
   └── utils/
       └── helpers.rs
   ```
2. Use recursion with a `prefix` parameter to track indentation
3. Show file sizes next to filenames

---

## Task 125 — File Statistics Tool

**Instructions:**
1. Given a file, report: size in bytes, line count, word count, char count, most common word
2. Implement efficiently — don't read the whole file into memory at once
3. Use `BufReader` for line-by-line processing

**Key Concepts:**
- `std::io::BufReader` — buffered reading
- `.lines()` iterator on `BufReader` — one line at a time
- Memory-efficient for large files

---

## Task 126 — Duplicate File Finder

**Instructions:**
1. Walk a directory tree, compute a hash (use MD5 or SHA256 via `sha2` crate) for each file
2. Group files by hash — files with same hash are duplicates
3. Report duplicate groups with their paths and total wasted space

---

## Task 127 — Folder Size Calculator

**Instructions:**
1. Recursively sum sizes of all files in a directory
2. Format sizes human-readably: bytes, KB, MB, GB
3. Print a breakdown by subdirectory, sorted by size (largest first)

---

## Task 128 — Mini File Explorer (CLI)

**Instructions:**
1. Interactive CLI loop: show current directory contents
2. Commands: `cd dirname`, `ls`, `cat filename`, `mkdir dirname`, `rm filename`, `pwd`, `quit`
3. Use `std::env::current_dir()` and `std::env::set_current_dir()`

---

## Task 129 — Log File Analyzer

**Instructions:**
1. Read a server log file (create a sample if needed)
2. Count requests by HTTP status code
3. Find the most frequent IP addresses
4. Calculate requests per hour
5. Find the slowest requests (if response time is in the log)

---

## Task 130 — Text Editor (CLI)

**Instructions:**
1. Open a file, display contents with line numbers
2. Commands: `edit N` (edit line N), `append text`, `insert N text`, `delete N`, `save`, `quit`
3. Keep an undo stack: `Vec<Vec<String>>` where each entry is a snapshot

---

## Task 131 — Notes Application

**Instructions:**
1. Notes stored as individual `.txt` files in a `~/.notes/` directory
2. Commands: `new "title"`, `list`, `show id`, `edit id`, `delete id`, `search "term"`
3. Auto-assign incremental IDs

---

## Task 132 — Markdown Note Manager

**Instructions:**
1. Extend Task 131: notes stored as `.md` files
2. Extract and index tags from notes (lines starting with `#tags:`)
3. Filter notes by tag
4. List notes sorted by last modified time

---

## Task 133 — Personal Wiki

**Instructions:**
1. Markdown files with `[[WikiLink]]` syntax linking to other notes
2. `list` — show all pages
3. `show page` — display a page with linked pages highlighted
4. `links page` — show all pages linking to this page (backlinks)
5. Build a `HashMap<String, Vec<String>>` of page → pages it links to

---

## Task 134 — Journal Application

**Instructions:**
1. One entry per day, stored as `YYYY-MM-DD.md`
2. `today` — open/create today's entry
3. `list` — show all entries
4. `show YYYY-MM-DD` — display a specific date
5. `search "term"` — search across all entries

---

## Task 135 — Local Document Search Engine

**Instructions:**
1. Index all `.txt` and `.md` files in a directory
2. Build an inverted index: `HashMap<String, Vec<(filename, line_number)>>`
3. Search returns filenames and line numbers for each match
4. Support multi-word queries (AND logic: all words must appear in the file)
5. Rank results by number of matches

---

# PHASE 9: Database Programming (Tasks 136–150)

---

## Task 136 — Connect to SQLite

**Goal:** Set up SQLite with Rust.

**Instructions:**
1. Create `task_136_sqlite`
2. Add to `Cargo.toml`:
   ```toml
   rusqlite = { version = "0.31", features = ["bundled"] }
   ```
3. Connect: `let conn = Connection::open("mydb.db")?;`
4. Connect to in-memory DB: `Connection::open_in_memory()?`
5. Close is automatic (Drop trait)

**Key Concepts:**
- `rusqlite` is the standard SQLite crate
- `features = ["bundled"]` compiles SQLite in — no system SQLite needed
- Connection implements `Drop` — closes automatically

---

## Task 137 — Create Tables

**Instructions:**
1. Execute DDL:
   ```rust
   conn.execute("CREATE TABLE IF NOT EXISTS users (
       id INTEGER PRIMARY KEY,
       name TEXT NOT NULL,
       email TEXT UNIQUE NOT NULL
   )", [])?;
   ```
2. Use `IF NOT EXISTS` for idempotent creation
3. Create multiple related tables (users, posts, comments)

---

## Task 138 — Insert Data

**Instructions:**
1. Parameterized insert:
   ```rust
   conn.execute("INSERT INTO users (name, email) VALUES (?1, ?2)", params![name, email])?;
   ```
2. Get last inserted row ID: `conn.last_insert_rowid()`
3. Batch insert multiple rows

---

## Task 139 — Update Data

**Instructions:**
1. `conn.execute("UPDATE users SET name = ?1 WHERE id = ?2", params![new_name, id])?`
2. Check affected rows: the return value is the number of rows changed
3. Update with conditions: only update if a field matches

---

## Task 140 — Delete Data

**Instructions:**
1. `conn.execute("DELETE FROM users WHERE id = ?1", params![id])?`
2. Implement soft delete: add `deleted_at TEXT` column, set timestamp instead of removing
3. Write a cleanup function to permanently delete soft-deleted records older than 30 days

---

## Task 141 — Query Records

**Instructions:**
1. Query all:
   ```rust
   let mut stmt = conn.prepare("SELECT id, name, email FROM users")?;
   let users = stmt.query_map([], |row| {
       Ok(User { id: row.get(0)?, name: row.get(1)?, email: row.get(2)? })
   })?;
   ```
2. Query with filter: `WHERE name LIKE ?1`
3. Query single row: `conn.query_row(...)`

---

## Task 142 — Pagination

**Instructions:**
1. `SELECT * FROM users ORDER BY id LIMIT ?1 OFFSET ?2`
2. Write `fn get_page(conn: &Connection, page: usize, per_page: usize) -> Result<Vec<User>>`
3. Get total count for pagination metadata: `SELECT COUNT(*) FROM users`

---

## Task 143 — Search Functionality

**Instructions:**
1. Case-insensitive search: `WHERE name LIKE ?1` with `%search_term%`
2. Search across multiple columns: `WHERE name LIKE ?1 OR email LIKE ?1`
3. FTS (Full Text Search): `CREATE VIRTUAL TABLE docs USING fts5(title, body)`

---

## Task 144 — Transactions

**Goal:** Ensure data integrity across multiple operations.

**Instructions:**
1. Begin a transaction: `conn.execute("BEGIN", [])?`
2. Perform multiple operations
3. Commit: `conn.execute("COMMIT", [])?` or Rollback: `conn.execute("ROLLBACK", [])?`
4. Use rusqlite's `transaction()` helper:
   ```rust
   let tx = conn.transaction()?;
   tx.execute(...)?;
   tx.execute(...)?;
   tx.commit()?;
   ```
5. Implement a bank transfer: debit one account, credit another — must be atomic

---

## Task 145 — Prepared Statements

**Instructions:**
1. Prepare once, execute many:
   ```rust
   let mut stmt = conn.prepare("INSERT INTO logs (msg) VALUES (?1)")?;
   for msg in messages { stmt.execute(params![msg])?; }
   ```
2. Benchmark: 1000 inserts with prepared vs unprepared statement

---

## Task 146 — CLI Todo App with SQLite

**Goal:** Full application backed by persistent storage.

**Instructions:**
1. Create `task_146_todo_sqlite`
2. Schema: `CREATE TABLE todos (id INTEGER PRIMARY KEY, title TEXT NOT NULL, done BOOLEAN DEFAULT 0, created_at TEXT)`
3. Commands:
   - `add "task title"` — insert new todo
   - `done N` — mark todo N as complete
   - `list` — show all incomplete todos
   - `all` — show all todos
   - `delete N` — remove a todo
4. Use `chrono` crate for timestamps (or just `std::time`)

---

## Task 147 — Expense Tracker with SQLite

**Instructions:**
1. Schema: `expenses(id, amount REAL, category TEXT, description TEXT, date TEXT)`
2. Commands: `add amount category description`, `list`, `total`, `by-category`, `month YYYY-MM`
3. Show a summary with totals per category

---

## Task 148 — Notes App with SQLite

**Instructions:**
1. Schema: `notes(id, title TEXT, content TEXT, tags TEXT, created_at TEXT, updated_at TEXT)`
2. Store tags as JSON array or comma-separated in a TEXT column
3. Commands: `new title`, `write id "content"`, `list`, `show id`, `tag id "tag"`, `search "term"`

---

## Task 149 — Password Manager with SQLite

**Instructions:**
1. Schema: `passwords(id, service TEXT, username TEXT, encrypted_password BLOB, notes TEXT)`
2. Use a master password to derive an encryption key (use `ring` or `aes-gcm` crate)
3. Encrypt passwords before storing, decrypt on retrieval
4. Commands: `add service username password`, `get service`, `list`, `delete service`

---

## Task 150 — Inventory Manager with SQLite

**Instructions:**
1. Schema: `products(id, name, sku TEXT UNIQUE, price REAL, quantity INTEGER, category TEXT)`
2. Commands: `add sku name price qty category`, `list`, `update-qty sku new_qty`, `low-stock threshold`, `total-value`, `search name`
3. Track stock movements: `stock_movements(id, product_id, change, reason, date)`

---

# PHASE 10: DSA Foundations (Tasks 151–180)

---

## Task 151 — Linear Search

**Instructions:**
1. Write `fn linear_search<T: PartialEq>(arr: &[T], target: &T) -> Option<usize>`
2. Returns the index of first occurrence, or None
3. Time complexity: O(n)

---

## Task 152 — Binary Search

**Instructions:**
1. Write `fn binary_search<T: Ord>(arr: &[T], target: &T) -> Option<usize>`
2. Requires sorted input
3. Use low/high pointers, not recursion (avoids stack overflow)
4. Compare with `std::slice::binary_search()`
5. Time complexity: O(log n)

---

## Task 153 — Bubble Sort

**Instructions:**
1. Write `fn bubble_sort<T: Ord>(arr: &mut Vec<T>)`
2. Early termination: if no swaps in a pass, array is sorted
3. Time: O(n²) worst/average, O(n) best

---

## Task 154 — Selection Sort

**Instructions:**
1. Write `fn selection_sort<T: Ord>(arr: &mut Vec<T>)`
2. Find minimum of unsorted portion, swap to front
3. Time: O(n²) always — no best case improvement

---

## Task 155 — Insertion Sort

**Instructions:**
1. Write `fn insertion_sort<T: Ord>(arr: &mut Vec<T>)`
2. Build sorted portion from left, insert each element in the right position
3. Time: O(n²) worst, O(n) best (nearly sorted data)

---

## Task 156 — Merge Sort

**Instructions:**
1. Write `fn merge_sort<T: Ord + Clone>(arr: &[T]) -> Vec<T>`
2. Returns a new sorted Vec (functional style)
3. Split in half, sort each half recursively, merge
4. Time: O(n log n) always

---

## Task 157 — Quick Sort

**Instructions:**
1. Write `fn quick_sort<T: Ord>(arr: &mut Vec<T>, low: usize, high: usize)`
2. Choose last element as pivot, partition around it
3. Recursively sort sub-arrays
4. Time: O(n log n) average, O(n²) worst

---

## Task 158 — Heap Sort

**Instructions:**
1. Write `fn heap_sort<T: Ord>(arr: &mut Vec<T>)`
2. First build a max-heap (heapify)
3. Repeatedly extract max to end of array
4. Time: O(n log n) always

---

## Task 159 — Counting Sort

**Instructions:**
1. Write `fn counting_sort(arr: &[usize]) -> Vec<usize>`
2. Only works on non-negative integers with bounded range
3. Count occurrences, reconstruct sorted array
4. Time: O(n + k) where k is the range

---

## Task 160 — Radix Sort

**Instructions:**
1. Write `fn radix_sort(arr: &mut Vec<u32>)`
2. Sort digit by digit from least significant to most significant
3. Use counting sort as the stable sort for each digit
4. Time: O(d * n) where d is the number of digits

---

## Task 161 — Singly Linked List

**Instructions:**
1. Define:
   ```rust
   type Link<T> = Option<Box<Node<T>>>;
   struct Node<T> { value: T, next: Link<T> }
   struct LinkedList<T> { head: Link<T>, length: usize }
   ```
2. Implement: `push_front`, `pop_front`, `peek`, `len`, `is_empty`
3. Implement the `Iterator` trait for your list

---

## Task 162 — Doubly Linked List

**Instructions:**
1. Use `Rc<RefCell<Node<T>>>` for shared ownership needed by doubly-linked nodes
2. Each node has `prev` and `next` links
3. This is complex in Rust — explore `std::collections::LinkedList` as well
4. Implement: `push_front`, `push_back`, `pop_front`, `pop_back`

---

## Task 163 — Stack (DSA)

**Instructions:**
Reimplement your Task 72 stack with added: `is_full(capacity)`, overflow protection. Write a balanced parentheses checker using the stack.

---

## Task 164 — Queue (DSA)

**Instructions:**
Extend Task 73 with: implement a print queue simulation with priorities.

---

## Task 165 — Priority Queue

**Instructions:**
1. Use `std::collections::BinaryHeap<T>` (max-heap by default)
2. For min-heap, use `Reverse(T)`: `BinaryHeap::from(vec![Reverse(3), Reverse(1), Reverse(2)])`
3. Implement a task scheduler using a priority queue

---

## Task 166 — Binary Tree

**Instructions:**
1. Define:
   ```rust
   enum BTree<T> { Leaf, Node { val: T, left: Box<BTree<T>>, right: Box<BTree<T>> } }
   ```
2. Implement: `insert`, `contains`, `depth`, `count`
3. Traversals: `inorder`, `preorder`, `postorder` — each returns a `Vec<T>`

---

## Task 167 — Binary Search Tree (BST)

**Instructions:**
1. BST property: left subtree values < node, right subtree values > node
2. Implement: `insert`, `search`, `min`, `max`, `delete`
3. In-order traversal of a BST gives sorted output — verify this

---

## Task 168 — AVL Tree

**Instructions:**
1. Self-balancing BST
2. Maintain a height field in each node
3. After insert/delete, check balance factor (height difference between left/right)
4. If unbalanced, perform rotations (LL, RR, LR, RL)
5. This is challenging — take your time

---

## Task 169 — Trie

**Instructions:**
1. Define: `struct TrieNode { children: HashMap<char, TrieNode>, is_end: bool }`
2. Implement: `insert(word)`, `search(word) -> bool`, `starts_with(prefix) -> bool`
3. Application: autocomplete — `fn suggestions(prefix: &str) -> Vec<String>`

---

## Task 170 — Heap (Data Structure)

**Instructions:**
1. Implement a max-heap manually using a Vec
2. Parent of node i is `(i-1)/2`; children of i are `2*i+1` and `2*i+2`
3. Implement: `push` (bubble up), `pop` (bubble down), `peek`

---

## Task 171 — Depth-First Search (DFS)

**Instructions:**
1. Represent a graph as `HashMap<usize, Vec<usize>>`
2. Implement recursive DFS: `fn dfs(graph: &Graph, start: usize, visited: &mut HashSet<usize>)`
3. Implement iterative DFS using a Stack
4. Return the order nodes were visited

---

## Task 172 — Breadth-First Search (BFS)

**Instructions:**
1. Implement BFS using a Queue (VecDeque)
2. Find shortest path (in terms of edges) between two nodes
3. Return the path, not just whether it's reachable

---

## Task 173 — Dijkstra's Algorithm

**Instructions:**
1. Weighted graph: `HashMap<usize, Vec<(usize, u32)>>` (node → [(neighbor, weight)])
2. Use a `BinaryHeap<(Reverse<u32>, usize)>` as a priority queue
3. Find shortest distances from source to all nodes
4. Reconstruct the actual shortest path

---

## Task 174 — Union-Find (Disjoint Set Union)

**Instructions:**
1. Implement with `parent: Vec<usize>` and `rank: Vec<usize>`
2. Operations: `find(x)` (with path compression), `union(x, y)` (by rank)
3. Application: detect cycles in an undirected graph

---

## Task 175 — Topological Sort

**Instructions:**
1. For a DAG (Directed Acyclic Graph)
2. Implement using Kahn's algorithm (BFS-based): count in-degrees, repeatedly remove zero-in-degree nodes
3. Detect cycles (if output has fewer nodes than the graph, there's a cycle)
4. Application: task dependency ordering

---

## Task 176 — LRU Cache

**Instructions:**
1. Least Recently Used cache with O(1) get and put
2. Use `HashMap<i32, Rc<RefCell<Node>>>` + doubly linked list
3. Or use `LinkedHashMap` from the `linked-hash-map` crate
4. Evict the least recently used entry when at capacity

---

## Task 177 — Sliding Window Maximum

**Instructions:**
1. Given array and window size k, find the max in each window
2. Use a `VecDeque<usize>` (monotonic deque) for O(n) solution
3. Return `Vec<i32>` of maximums

---

## Task 178 — Longest Substring Without Repeating Characters

**Instructions:**
1. Use sliding window with `HashMap<char, usize>` tracking last seen index
2. Return the length and the actual substring
3. Time: O(n)

---

## Task 179 — Merge Intervals

**Instructions:**
1. Given `Vec<(i32, i32)>` of intervals, merge overlapping ones
2. Sort by start time first
3. Walk through, extending current interval or starting new one

---

## Task 180 — Kth Largest Element

**Instructions:**
1. Find the kth largest element in an unsorted array
2. Method 1: Sort and index — O(n log n)
3. Method 2: Min-heap of size k — O(n log k)
4. Method 3: Quickselect — O(n) average

---

# PHASE 11: Concurrency (Tasks 181–195)

---

## Task 181 — Spawn Threads

**Goal:** Understand Rust's fearless concurrency.

**Instructions:**
1. Spawn threads with `std::thread::spawn(|| { ... })`
2. Join: `let handle = thread::spawn(...); handle.join().unwrap();`
3. Spawn multiple threads, join all
4. Move data into threads: `thread::spawn(move || { ... })` — the `move` keyword

**Key Concepts:**
- Threads need `'static` lifetime for their data, or `move` closure
- `JoinHandle<T>` — the thread's return value
- Rust prevents data races at compile time

---

## Task 182 — Shared Counter Using Arc

**Instructions:**
1. `Arc<T>` — Atomic Reference Counted pointer (thread-safe `Rc`)
2. `Arc<Mutex<i32>>` for shared mutable state
3. Clone the Arc for each thread: `let counter = Arc::clone(&counter)`
4. Lock to access: `let mut val = counter.lock().unwrap(); *val += 1;`

---

## Task 183 — Mutex Example

**Instructions:**
1. Demonstrate: without Mutex, you can't share mutable state across threads (compile error)
2. With `Mutex<T>`, locking gives you exclusive access
3. Deadlock example: two threads each holding a lock and waiting for the other's lock

---

## Task 184 — RwLock Example

**Instructions:**
1. `RwLock<T>` — multiple readers OR one writer
2. Use when you have many reads and few writes
3. `.read()` for shared read lock, `.write()` for exclusive write lock
4. Compare performance with Mutex for read-heavy workloads

---

## Task 185 — Thread Pool

**Instructions:**
1. Create a fixed-size pool of worker threads
2. Workers wait on a channel for jobs
3. Submit jobs via the channel; a free worker picks it up
4. Implement `ThreadPool::new(size)` and `.execute(job)`

---

## Task 186 — Producer-Consumer

**Instructions:**
1. Producer thread generates work items and sends them on a channel
2. Consumer thread receives and processes them
3. Use `std::sync::mpsc::channel()` — multiple producer, single consumer
4. Multiple producers: clone the sender for each producer

---

## Task 187 — Parallel File Processing

**Instructions:**
1. Read a directory of text files
2. Process each file in a separate thread
3. Collect results using channels or join handles
4. Compare time with sequential processing

---

## Task 188 — Concurrent Web Scraper

**Instructions:**
1. Spawn a thread per URL to fetch (use `reqwest::blocking` or `ureq`)
2. Collect results from all threads
3. Handle errors per-thread without crashing others
4. Rate limit: only N threads at a time using a Semaphore or thread pool

---

## Task 189 — Concurrent Log Parser

**Instructions:**
1. Split a large log file into chunks
2. Parse each chunk in a separate thread
3. Merge results (combine HashMaps from each thread)
4. Time the parallel vs sequential version

---

## Task 190 — Multi-Threaded Downloader

**Instructions:**
1. Download multiple files concurrently
2. One thread per file, each writes to its own output file
3. Show a progress indicator for each download
4. Handle partial downloads — retry on failure

---

## Task 191 — Channel Communication

**Instructions:**
1. Deep dive into `mpsc` channels
2. Bounded channel: `sync_channel(capacity)` — blocks sender when full
3. Unbounded channel: `channel()` — never blocks sender
4. Pass complex structs through channels
5. Build a pipeline: stage1 → channel → stage2 → channel → stage3

---

## Task 192 — Work Queue

**Instructions:**
1. Build a work queue where workers pull tasks from a shared queue
2. Use `Arc<Mutex<VecDeque<Task>>>` as the shared queue
3. Worker threads loop: lock, pop, unlock, process
4. Poison a mutex: what happens if a worker panics while holding the lock?

---

## Task 193 — Job Scheduler

**Instructions:**
1. Schedule jobs to run at a future time
2. Main thread sleeps until next job's time, then dispatches it to a worker thread
3. Support recurring jobs
4. Thread-safe job queue using Arc + Mutex

---

## Task 194 — Rate Limiter

**Instructions:**
1. Implement a token bucket rate limiter
2. Tokens fill up at a fixed rate; each request consumes one
3. `Arc<Mutex<TokenBucket>>` for thread-safe access
4. Test with concurrent "requests" from multiple threads

---

## Task 195 — Concurrent Cache

**Instructions:**
1. Build a cache using `Arc<RwLock<HashMap<K, V>>>`
2. Many threads can read simultaneously; writes are exclusive
3. Add a TTL (time-to-live): expire entries after N seconds
4. Implement a background cleanup thread

---

# PHASE 12: Async Rust (Tasks 196–210)

---

## Task 196 — Learn Tokio Basics

**Goal:** Set up async Rust with the Tokio runtime.

**Instructions:**
1. Add `tokio = { version = "1", features = ["full"] }` to Cargo.toml
2. Mark `main` as `#[tokio::main] async fn main()`
3. Write an `async fn greet(name: &str)` — use `tokio::time::sleep`
4. `await` the function: `greet("Rust").await;`

**Key Concepts:**
- `async fn` returns a `Future` — doesn't run until awaited
- `.await` runs the future to completion
- `tokio::main` starts the async runtime
- No OS threads are created per task — cooperative multitasking

**Python Comparison:**
```python
import asyncio
async def greet(): await asyncio.sleep(1)
asyncio.run(greet())  # Almost identical pattern
```

---

## Task 197 — Async Timer

**Instructions:**
1. `tokio::time::sleep(Duration::from_secs(1)).await`
2. Spawn multiple async tasks: `tokio::spawn(async { ... })`
3. Run tasks concurrently: `tokio::join!(task1, task2, task3)` — all run at the same time
4. Compare `join!` (concurrent) vs sequential awaiting

---

## Task 198 — Async File Reader

**Instructions:**
1. `use tokio::fs;`
2. `let content = fs::read_to_string("file.txt").await?;`
3. Read multiple files concurrently with `join!`
4. Compare time with sequential reading

---

## Task 199 — Async File Writer

**Instructions:**
1. `tokio::fs::write("output.txt", content).await?`
2. Append to file: open with `OpenOptions`, write
3. Write multiple files concurrently

---

## Task 200 — Async TCP Client

**Instructions:**
1. `use tokio::net::TcpStream;`
2. Connect: `TcpStream::connect("127.0.0.1:8080").await?`
3. Write data: `stream.write_all(b"hello").await?`
4. Read response: `stream.read(&mut buf).await?`

---

## Task 201 — Async TCP Server

**Instructions:**
1. `TcpListener::bind("127.0.0.1:8080").await?`
2. Accept loop: `loop { let (socket, _) = listener.accept().await?; tokio::spawn(handle(socket)); }`
3. Handle each connection in its own spawned task
4. Echo server: read data, write it back

---

## Task 202 — Async Chat Server

**Instructions:**
1. Use `tokio::sync::broadcast::channel` for broadcasting messages to all clients
2. Each client gets a clone of the sender and a receiver
3. Spawn two tasks per connection: one reads from socket, one writes to socket

---

## Task 203 — Async Web Scraper

**Instructions:**
1. Add `reqwest = { version = "0.11", features = ["json"] }` to Cargo.toml
2. `reqwest::get(url).await?.text().await?`
3. Scrape multiple URLs concurrently using `tokio::spawn`
4. Collect all results

---

## Task 204 — Async Downloader

**Instructions:**
1. Download files using reqwest with streaming: `.bytes_stream()`
2. Write chunks to disk as they arrive
3. Download multiple files concurrently
4. Show download progress (bytes received / total size)

---

## Task 205 — Async Task Queue

**Instructions:**
1. Use `tokio::sync::mpsc::channel` for async channel
2. Producer sends tasks; worker receives and processes
3. Multiple workers: each calls `recv()` on the same receiver wrapped in `Arc<Mutex<Receiver>>`

---

## Task 206 — Async SQLite App

**Instructions:**
1. Add `tokio-rusqlite = "0.5"` for async SQLite
2. Wrap operations in `conn.call(|conn| { ... }).await`
3. Build an async CRUD service

---

## Task 207 — Async PostgreSQL App

**Instructions:**
1. Add `tokio-postgres = "0.7"` or `sqlx = { version = "0.7", features = ["postgres", "runtime-tokio"] }`
2. Connect to a local PostgreSQL instance (use Docker: `docker run -p 5432:5432 -e POSTGRES_PASSWORD=pass postgres`)
3. Create tables, insert, query using prepared statements

---

## Task 208 — Async Redis App

**Instructions:**
1. Add `redis = { version = "0.25", features = ["tokio-comp"] }`
2. Connect to Redis (Docker: `docker run -p 6379:6379 redis`)
3. SET, GET, DEL, EXPIRE, LPUSH, LPOP operations asynchronously

---

## Task 209 — Async Message Processor

**Instructions:**
1. Create a pipeline: ingest → transform → store
2. Each stage is an async task communicating via channels
3. Buffer between stages: bounded channels apply backpressure

---

## Task 210 — Build an Async Microservice

**Instructions:**
1. Add `axum = "0.7"` web framework
2. Create a simple REST API:
   - `GET /items` — list all
   - `POST /items` — create
   - `GET /items/:id` — get one
   - `DELETE /items/:id` — delete
3. Use `Arc<Mutex<Vec<Item>>>` as in-memory store
4. Return JSON responses

---

# PHASE 13: Systems Programming (Tasks 211–225)

---

## Task 211 — Hex Viewer

**Instructions:**
1. Read a binary file in chunks
2. Print offset, hex bytes (16 per row), and ASCII representation
3. Output like the `xxd` command

---

## Task 212 — Memory Inspector

**Instructions:**
1. Print memory addresses of variables: `let x = 42; println!("{:p}", &x);`
2. Compare addresses of stack vs heap variables
3. Show that `Box<T>` puts data on the heap
4. Visualize how a Vec is laid out (pointer, length, capacity on stack; data on heap)

---

## Task 213 — Binary Parser

**Instructions:**
1. Read a binary file (e.g., a BMP image or a custom format)
2. Parse fixed-size fields using byte slices
3. Use `u32::from_le_bytes([b0, b1, b2, b3])` to convert bytes to integers
4. Create structs mirroring the binary layout

---

## Task 214 — ELF Parser (Basic)

**Instructions:**
1. Read an ELF binary (any Linux executable)
2. Parse the ELF header: magic number, architecture, entry point
3. List section names
4. This is for learning — use `goblin` crate for production

---

## Task 215 — Process Monitor

**Instructions:**
1. Add `sysinfo = "0.30"` crate
2. List all running processes: PID, name, CPU%, memory
3. Refresh every second, clear screen, redisplay
4. Filter by process name

---

## Task 216 — CPU Usage Monitor

**Instructions:**
1. Use `sysinfo` to get per-CPU core usage
2. Display as ASCII bar chart: `[████████░░░░] 65%`
3. Update every second

---

## Task 217 — RAM Monitor

**Instructions:**
1. Total memory, used memory, available memory
2. Display as percentage with progress bar
3. Alert if memory usage exceeds 90%

---

## Task 218 — Mini Shell

**Instructions:**
1. Display a prompt: `rustsh $ `
2. Read a line of input
3. Split into command and arguments
4. Execute using `std::process::Command`
5. Built-in commands: `cd`, `exit`, `pwd`
6. Handle `$?` (last exit code)

---

## Task 219 — Environment Manager

**Instructions:**
1. Read all env vars: `std::env::vars()`
2. Set env var: `std::env::set_var(key, value)`
3. Remove env var: `std::env::remove_var(key)`
4. Build a `.env` file loader that sets variables in the current process

---

## Task 220 — Cron-like Scheduler

**Instructions:**
1. Define jobs with a schedule (every N seconds for simplicity)
2. Main loop checks time, runs due jobs
3. Jobs run in separate threads to not block the scheduler
4. Log each execution with timestamp

---

## Task 221 — TCP Server

**Instructions:**
1. `std::net::TcpListener::bind("127.0.0.1:8080")?`
2. For each connection: spawn a thread to handle it
3. Echo back whatever the client sends
4. Gracefully handle disconnects

---

## Task 222 — HTTP Parser

**Instructions:**
1. Parse raw HTTP/1.1 request text
2. Extract: method, path, HTTP version, headers (HashMap), body
3. Parse query string from path
4. Return a `ParsedRequest` struct

---

## Task 223 — HTTP Server

**Instructions:**
1. Build on Task 221 and 222
2. Route requests: `GET /` → 200 OK with HTML, `GET /time` → current time, anything else → 404
3. Parse request headers
4. Return proper HTTP response format

---

## Task 224 — Key-Value Store

**Instructions:**
1. In-memory KV store: `HashMap<String, String>`
2. TCP protocol: `SET key value\n`, `GET key\n`, `DEL key\n`
3. Persistence: periodically write state to a file as JSON
4. Load from file on startup

---

## Task 225 — Redis Clone (Basic)

**Instructions:**
1. Extend Task 224 with: `EXPIRE key seconds`, `TTL key`, `KEYS pattern`
2. Support data types: String and List (`LPUSH`, `LPOP`, `LLEN`)
3. Multi-client support using threads
4. `PING` → `PONG`

---

# PHASE 14: Embedded & no_std (Tasks 226–245)

---

## Task 226 — Create a no_std Project

**Goal:** Understand what the standard library provides and how to work without it.

**Instructions:**
1. Create a library crate: `cargo new --lib task_226_no_std`
2. Add `#![no_std]` at the top of `lib.rs`
3. Add a panic handler: `#[panic_handler] fn panic(_: &PanicInfo) -> ! { loop {} }`
4. Note what's no longer available: heap allocation, threads, file system, networking
5. What IS available: core types (Option, Result), iterators, math, sorting slices

**Key Concepts:**
- `no_std` means no standard library — only `core` and optionally `alloc`
- Required for microcontrollers, kernels, and WebAssembly
- Most logic code can be `no_std` compatible

---

## Task 227 — Blink LED (Simulation or Hardware)

**Instructions:**
For simulation (no hardware):
1. Install `cargo-embed` or just understand the pattern
2. Write pseudo-code: `loop { led.set_high(); delay_ms(500); led.set_low(); delay_ms(500); }`
3. Study the `embedded-hal` abstraction traits
4. Understand that `embedded-hal` traits allow writing hardware-agnostic code

For hardware (if available):
1. Use `probe-run` and an STM32 or Raspberry Pi Pico
2. Use the appropriate HAL crate for your board

---

## Tasks 228–245 — Embedded Hardware Tasks

For tasks 228–245 (UART, GPIO, PWM, ADC, I2C, SPI, sensors, displays), follow this pattern:

**If you have hardware:**
1. Identify your microcontroller (STM32, RP2040, ESP32, Arduino/AVR)
2. Find the appropriate Rust HAL crate on crates.io
3. Follow the board-specific examples in the HAL repository
4. Use `probe-run` or `cargo-flash` to flash your firmware

**If you don't have hardware (simulation/learning):**
1. Read the `embedded-hal` trait documentation — understand the abstractions
2. Write `no_std` logic that would work with any compliant hardware
3. Test your logic using a mock HAL implementation
4. Simulate with QEMU: `cargo install cargo-embed`

**Core embedded-hal traits to study:**
- `OutputPin` / `InputPin` — digital GPIO
- `serial::Write` — UART output
- `Delay` — blocking delay
- `I2c` — I2C bus operations
- `Spi` — SPI bus operations
- `Pwm` — PWM signal generation
- `adc::OneShot` — ADC readings

---

# PHASE 15: Professional Projects (Tasks 246–260)

---

## Task 246 — CLI Password Manager

**Instructions:**
Full-featured version of Task 149. Add:
1. Master password hashing with Argon2
2. AES-256-GCM encryption for stored passwords
3. Password generator: `generate_password(length, use_symbols, use_numbers)`
4. Import/export to encrypted file
5. Password strength checker

---

## Task 247 — Git Clone (Basic)

**Instructions:**
1. Implement `init` — create `.git` directory structure
2. Implement `hash-object` — hash a file and store as a blob object
3. Implement `cat-file` — read and display a stored object
4. Implement `write-tree` — create a tree object from current directory
5. This is a learning project — focus on understanding git's object model

---

## Task 248 — URL Shortener Backend

**Instructions:**
1. Use `axum` web framework
2. POST `/shorten` with `{"url": "https://..."}` → returns `{"short": "abc123"}`
3. GET `/:code` → 302 redirect to original URL
4. Store mappings in SQLite
5. Generate short codes: random 6 alphanumeric characters

---

## Task 249 — Task Queue System

**Instructions:**
1. Persistent task queue backed by SQLite
2. Workers pull tasks, process them, mark as done
3. Failed tasks are retried up to 3 times
4. Task types: defined by a string tag + JSON payload

---

## Task 250 — Event Bus

**Instructions:**
1. Publish-subscribe system
2. `subscribe(event_type, handler)` — register a callback
3. `publish(event_type, payload)` — trigger all handlers for that event type
4. Thread-safe using Arc + RwLock
5. Async version using tokio channels

---

## Task 251 — Local Search Engine

**Instructions:**
1. Expand Task 135: add TF-IDF ranking
2. TF (term frequency) = count of term in doc / total terms in doc
3. IDF (inverse document frequency) = log(total docs / docs containing term)
4. TF-IDF score = TF * IDF — higher = more relevant
5. Return top results ranked by score

---

## Task 252 — Static Site Generator

**Instructions:**
1. Read Markdown files from `content/` directory
2. Convert Markdown to HTML (use `pulldown-cmark` crate)
3. Apply a template (simple string substitution)
4. Write output HTML to `public/` directory
5. Copy static assets (CSS, images)

---

## Task 253 — Terminal File Manager

**Instructions:**
1. Use `crossterm` or `tui-rs` for terminal UI
2. Two-pane layout: left and right directory views
3. Keys: arrow keys to navigate, Enter to open, Tab to switch panes, `d` to delete, `c`/`v` to copy/paste, `q` to quit

---

## Task 254 — SQLite ORM (Minimal)

**Instructions:**
1. Define a derive macro (or use a builder pattern) to map structs to tables
2. Auto-generate CREATE TABLE, INSERT, SELECT, UPDATE, DELETE queries
3. `#[derive(Model)] struct User { id: i64, name: String }` → generates `User::create_table()`, `User::find(id)`, etc.

---

## Task 255 — Mini Docker-like Runner

**Instructions:**
1. Use Linux namespaces (via `nix` crate) to isolate processes
2. Create a new PID namespace, mount namespace
3. `chroot` into a filesystem directory
4. Run a command in the isolated environment
5. This is Linux-specific — understand containers at the syscall level

---

## Task 256 — Redis Clone (Full)

**Instructions:**
1. Extend Task 225 with: all major data types (String, List, Hash, Set, Sorted Set)
2. RDB persistence: serialize state to disk on shutdown
3. AOF (Append Only File): log each write command
4. Replication basics: leader accepts writes, follower replicates

---

## Task 257 — Kafka-style Queue

**Instructions:**
1. Topics with partitions
2. Producers append messages to a topic
3. Consumers read from an offset, move offset forward
4. Messages are retained (not deleted on consume)
5. Persist to disk using append-only files

---

## Task 258 — MQTT Broker

**Instructions:**
1. Implement MQTT v3.1.1 protocol parser
2. Support: CONNECT, PUBLISH, SUBSCRIBE, UNSUBSCRIBE, DISCONNECT
3. QoS 0 (fire and forget) delivery
4. Wildcard topics: `sensor/+/temp`, `sensor/#`

---

## Task 259 — Embedded Telemetry System

**Instructions:**
1. Simulate sensor readings (temperature, pressure, etc.)
2. Encode readings in a compact binary format
3. Send over a TCP connection simulating UART
4. Server receives and logs to SQLite
5. Web dashboard (simple HTML) shows latest readings

---

## Task 260 — Personal Rust Framework

**Instructions:**
1. Extract patterns from your previous 259 tasks
2. Create a library crate with your most-used utilities:
   - Error handling helpers
   - Database connection pool
   - Config loading
   - CLI argument parsing
   - Logging setup
3. Publish to crates.io (or just keep it as a local library)

---

# BONUS CHALLENGES

---

## Task 261 — Rewrite Todo App 5 Ways

Write a complete Todo app five times, each time applying a different Rust concept:

**Version 1 — Structs:** Use plain structs and Vec. No traits, no generics.

**Version 2 — Traits:** Define a `TodoStorage` trait. Implement it with an in-memory store and a file-backed store.

**Version 3 — Generics:** Make the todo system generic: `TodoList<T: Task>` where `Task` is a trait.

**Version 4 — Async:** Use Tokio. Storage operations are async. Multiple users can use the app concurrently.

**Version 5 — SQLite:** Use rusqlite for persistence. Full CRUD with transactions.

---

## Task 262 — Rewrite Calculator 3 Ways

**Version 1:** Simple functions, no structs.
**Version 2:** `enum Operation` with a `calculate()` method.
**Version 3:** Trait-based: `trait Calculable { fn evaluate(&self) -> f64; }`, expression tree.

---

## Task 263 — Rewrite Note App 3 Ways

**Version 1:** Files on disk (plaintext).
**Version 2:** SQLite backend.
**Version 3:** Async HTTP backend (notes stored on a server).

---

## Task 264 — Rewrite File Explorer 3 Ways

**Version 1:** CLI with basic commands.
**Version 2:** TUI with interactive navigation.
**Version 3:** Web UI served from a Rust HTTP server.

---

## Task 265 — Rewrite Inventory System 3 Ways

**Version 1:** In-memory with Vec/HashMap.
**Version 2:** SQLite with full CRUD.
**Version 3:** REST API with axum + SQLite.

---

## Task 266 — Implement Borrow Checker Rules From Memory

Write a Rust file that demonstrates each rule of the borrow checker from memory:
- Move semantics
- Copy types
- Immutable borrows
- Mutable borrows (single at a time)
- Lifetime basics
- Dangling reference prevention
- Non-lexical lifetimes (NLL)

Comment each rule in plain English as if explaining to a junior developer.

---

## Task 267 — Build Your Own `Result<T, E>`

1. Define `enum MyResult<T, E> { Ok(T), Err(E) }`
2. Implement: `.unwrap()`, `.map()`, `.map_err()`, `.and_then()`, `.or_else()`, `.unwrap_or()`
3. Write a custom `?`-like macro: `try_my!(expr)` that returns early on error
4. This makes you understand how `Result` actually works internally

---

## Task 268 — Build Your Own `Vec<T>`

1. Define `struct MyVec<T> { ptr: *mut T, len: usize, cap: usize }`
2. Use `std::alloc::{alloc, dealloc, realloc}` for memory management
3. Implement: `new()`, `push()`, `pop()`, `get()`, `len()`, `capacity()`
4. Implement `Drop` to free memory
5. This is `unsafe` code — understand what invariants you must maintain

---

## Task 269 — Build Your Own `String`

1. Define `struct MyString { vec: MyVec<u8> }` using your Task 268 Vec
2. Ensure UTF-8 validity on construction
3. Implement: `new()`, `push_str()`, `len()`, `as_str() -> &str`, `chars()`

---

## Task 270 — Build Your Own `HashMap<K, V>`

1. Use an array of `Vec<(K, V)>` as buckets (open hashing / chaining)
2. Hash keys using `std::hash::Hash`
3. Implement: `new()`, `insert()`, `get()`, `remove()`, `contains_key()`
4. Resize when load factor exceeds 0.75

---

## Task 271 — Fix 50 Ownership Bugs

Write or find 50 programs that don't compile due to ownership/borrow errors. Fix each one and document:
1. What the error was
2. Why Rust rejected it
3. The correct fix

Keep this as a `BORROW_BUGS.md` reference document.

---

## Task 272 — Solve 100 LeetCode Problems in Rust

Solve LeetCode problems using Rust. Focus on:
- Easy (40): arrays, strings, math, basic data structures
- Medium (50): DP, trees, graphs, intervals, sliding window
- Hard (10): advanced DP, graph algorithms

For each solution, write a comment explaining the time/space complexity.

---

## Task 273 — Solve 50 Codeforces Problems in Rust

Focus on: I/O-heavy problems (learn fast I/O in Rust), implementation problems, math problems.

Fast I/O template:
```rust
use std::io::{self, BufRead, Write, BufWriter};
fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());
    for line in stdin.lock().lines() { writeln!(out, "{}", line.unwrap()).unwrap(); }
}
```

---

## Task 274 — Contribute to an Open-Source Rust Crate

1. Find a crate you use (from your earlier tasks)
2. Look at the issue tracker on GitHub
3. Fix a bug, add a feature, or improve documentation
4. Submit a PR

Good starting points: `rustlings`, `cli-tools`, utility crates you found useful.

---

## Task 275 — Build a Complete Production-Ready CLI Suite

Combine your best work into one unified CLI tool with subcommands:
- `mytool notes` — notes manager
- `mytool todo` — todo manager
- `mytool budget` — expense tracker
- `mytool pass` — password manager
- `mytool files` — file utilities

Use `clap = "4"` for argument parsing. SQLite for all persistence. Ship it as a single binary.

---

# Quick Reference

## Cargo Commands
```sh
cargo new project_name      # new binary
cargo new --lib lib_name    # new library
cargo build                 # compile debug
cargo build --release       # compile optimized
cargo run                   # build and run
cargo test                  # run tests
cargo clippy                # linter
cargo fmt                   # formatter
cargo doc --open            # generate and open docs
cargo add crate_name        # add dependency (requires cargo-edit)
```

## Frequently Needed Crates
```toml
serde = { version = "1", features = ["derive"] }
serde_json = "1"
rusqlite = { version = "0.31", features = ["bundled"] }
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.11", features = ["json"] }
rand = "0.8"
clap = { version = "4", features = ["derive"] }
chrono = "0.4"
anyhow = "1"         # easy error handling
thiserror = "1"      # custom error types
log = "0.4"          # logging facade
env_logger = "0.11"  # log backend
axum = "0.7"         # web framework
```

## Ownership Cheat Sheet
```
T          → moves (one owner at a time)
&T         → immutable borrow (many allowed simultaneously)
&mut T     → mutable borrow (only one, no other borrows)
Box<T>     → heap-allocated T, single owner
Rc<T>      → reference-counted, single-threaded shared ownership
Arc<T>     → atomic reference-counted, multi-threaded shared ownership
Cell<T>    → interior mutability for Copy types
RefCell<T> → interior mutability, runtime borrow checking
Mutex<T>   → exclusive access, thread-safe
RwLock<T>  → shared reads / exclusive writes, thread-safe
```

## Error Handling Patterns
```rust
// Propagate with ?
fn do_thing() -> Result<(), MyError> {
    let content = std::fs::read_to_string("file.txt")?;
    Ok(())
}

// Add context (with anyhow)
use anyhow::{Context, Result};
let f = std::fs::File::open("file.txt")
    .context("Failed to open config file")?;

// Custom error with thiserror
use thiserror::Error;
#[derive(Error, Debug)]
enum MyError {
    #[error("Not found: {0}")]
    NotFound(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
```