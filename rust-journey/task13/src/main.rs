fn consume_vector(v: Vec<i32>) {
    println!("Vector consumed inside function. Length: {}", v.len());
    // v goes out of scope here and its heap memory is freed!
}

fn consume_primitive(num: i32) {
    println!("Primitive consumed inside function: {}", num);
}

fn main() {
    // =========================================================================
    // 1. The MOVE Behavior (Heap-Allocated Data)
    // =========================================================================
    let v1 = vec![1, 2, 3, 4, 5];
    
    consume_vector(v1);
    
    // UNCOMMENTING the line below will cause a compilation error (E0382):
    // println!("Trying to use v1: {:?}", v1); 
    // Reason: Ownership of the heap pointer was moved entirely into the function.

    // =========================================================================
    // 2. The CLONE Behavior (Explicit Deep Copy)
    // =========================================================================
    let original_vec = vec![10, 20, 30];
    
    // Deep copy: Allocates new memory on the heap and copies every element
    let cloned_vec = original_vec.clone(); 
    
    consume_vector(cloned_vec); // We move the clone, leaving original intact
    
    println!("original_vec is still usable because we cloned it: {:?}", original_vec);

    // =========================================================================
    // 3. The COPY Behavior (Stack-Only Primitives)
    // =========================================================================
    let x: i32 = 42;
    
    consume_primitive(x); // Implicitly copies 'x' because i32 implements the Copy trait
    
    println!("x is still perfectly usable: {}", x); // No error!
}

/*
=========================================================================
CLONE VS REFERENCES: WHEN TO USE WHICH?
=========================================================================
1. Use CLONE when:
   - You genuinely need an independent, mutable modification of the data 
     without altering the original source.
   - You are spinning up threads (e.g., std::thread::spawn) and need the 
     data to outlive the current function scope ('static lifetime required).

2. Use REFERENCES (&T or &mut T) when: (in other words borrowing)
   - You only need read-only access to inspect the data (saves CPU cycles and memory).
   - You are writing lightweight operations (like printing or calculating metrics).
   - Performance matters. Passing a reference is always an O(1) pointer-copy operation, 
     whereas .clone() is an O(n) operation that hits the OS allocator.
*/