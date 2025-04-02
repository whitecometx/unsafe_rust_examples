# Unsafe Rust Scenarios - README

This repository demonstrates various scenarios where unsafe Rust code is used. The examples cover common unsafe operations, their risks, and how to handle them safely. Unsafe Rust allows you to perform low-level operations that are not checked by the compiler, such as dereferencing raw pointers, accessing mutable static variables, and working with unions. While unsafe code is powerful, it requires careful handling to avoid undefined behavior.

---

## **Table of Contents**
- [Introduction](#introduction)
- [Prerequisites](#prerequisites)
- [Scenarios Covered](#scenarios-covered)
- [How to Run](#how-to-run)
- [Code Explanation](#code-explanation)

---

## **Introduction**

Unsafe Rust is a subset of the Rust programming language that allows you to bypass some of its safety guarantees. It is useful when you need to:
- Interact with low-level hardware or external libraries.
- Optimize performance-critical code.
- Access features not available in safe Rust.

This project provides examples of unsafe Rust usage and explains how to mitigate risks by wrapping unsafe operations in safe abstractions.

---

## **Prerequisites**

To run the code, you need:
1. **Rust**: Install Rust using [rustup](https://rustup.rs/).
2. **Cargo**: Cargo is included with Rust and is used to build and run the project.

---

## **Scenarios Covered**

The following scenarios are demonstrated in the `main.rs` file:

### **1. Accessing Raw Pointers**
- **Unsafe Operation**: Dereferencing raw pointers.
- **Solution**: Use an `unsafe` block to ensure safety.

### **2. Accessing Boxed Pointers**
- **Unsafe Operation**: Dereferencing pointers from `Box`.
- **Solution**: Use an `unsafe` block for dereferencing.

### **3. Null Pointer Dereferencing**
- **Unsafe Operation**: Dereferencing a null pointer.
- **Solution**: Check for null before dereferencing.

### **4. Unsafe Code in Functions**
- **Unsafe Operation**: Calling a function that contains unsafe code.
- **Solution**: Wrap the function call in an `unsafe` block.

### **5. Accessing Mutable Static Variables**
- **Unsafe Operation**: Modifying a mutable static variable.
- **Solution**: Use an `unsafe` block for access.

### **6. Accessing Unions**
- **Unsafe Operation**: Reading fields of a union.
- **Solution**: Use an `unsafe` block for field access.

### **7. Accessing Enums**
- Enums in Rust are safe by default and can be accessed using pattern matching without requiring an `unsafe` block.

---

## **How to Run**

1. Clone the repository:
   ```bash
   git clone https://github.com/your-repo-name/unsafe-rust-scenarios.git
   cd unsafe-rust-scenarios
   ```

2. Build the project:
   ```bash
   cargo build
   ```

3. Run the project:
   ```bash
   cargo run
   ```

---

## **Code Explanation**

Here is a brief explanation of each scenario:

### Scenario 1: Accessing Raw Pointers
```rust
let a = 10;
let ptr: *const i32 = &a;
unsafe {
    println!("SCENARIO 1: the value of a is {}", *ptr);
}
```
Raw pointers allow direct memory access but require an `unsafe` block for dereferencing.

---

### Scenario 2: Accessing Boxed Pointers
```rust
let b = Box::new(10);
let ptr_b: *const i32 = &*b;
unsafe {
    println!("SCENARIO 2: the value of b is {}", *ptr_b);
}
```
Boxed pointers (`Box`) can be dereferenced unsafely, but using an `unsafe` block ensures safety.

---

### Scenario 3: Null Pointer Dereferencing
```rust
let ptr_c: *const i32 = ptr::null();
unsafe {
    if !ptr_c.is_null() {
        println!("SCENARIO 3: the value of ptrC is {}", *ptr_c);
    } else {
        println!("SCENARIO 3: ptrC is null");
    }
}
```
Always check for null before dereferencing raw pointers to avoid segmentation faults.

---

### Scenario 4: Unsafe Code in Functions
```rust
unsafe fn wraps_unsafe_code() {
    let a = 10;
    let ptr: *const i32 = &a;
    println!("the value of a is {}", *ptr);
}

unsafe {
    wraps_unsafe_code();
}
```
Wrap unsafe function calls in an `unsafe` block for controlled execution.

---

### Scenario 5: Accessing Mutable Static Variables
```rust
static mut POINTS: i32 = 10;
println!("SCENARIO 5: the value of POINTS is {}", unsafe { POINTS });
```
Access mutable static variables using an `unsafe` block.

---

### Scenario 6: Accessing Unions
```rust
union Win {
    bronze: i32,
    silver: i32,
    gold: i32,
}
let win = Win { bronze: 10 };
unsafe {
    println!("SCENARIO 6: the value of win is {}", win.bronze);
}
```
Unions require an `unsafe` block for field access because they do not enforce type safety.

---

### Scenario 7: Accessing Enums
```rust
enum Win2 {
    bronze(i32),
    silver(i32),
    gold(i32),
}
let win2 = Win2::bronze(10);
if let Win2::bronze(value) = win2 {
    println!("SCENARIO 7: the value of win is {}", value);
}
```
Enums are safe by default and can be accessed using pattern matching without requiring an `unsafe` block.

---

By following this README, you can understand and safely experiment with unsafe Rust scenarios while learning how to mitigate risks effectively!

