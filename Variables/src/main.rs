// Variables/src/main.rs
// This is a simple Rust program that demonstrates variable declaration, mutability, scope, and shadowing.
// To run this code, save it in a file named `main.rs` and use the command `cargo run` in the terminal.
// Make sure you have Rust installed on your system.
// You can install Rust from https://www.rust-lang.org/tools/install

#[allow(unused_variables)]
fn main() {
    println!("=====Variables in Rust=====");
    let x: i16 = 42;
    println!("the value of x is: {x}");
    // let y: i32 = 1000;
    // y = 2000; // This line will cause a compilation error because `y` is immutable
    let mut y: i32 = 1000;
    println!("the value of y before updating: {y}");
    y = 2000; // This line is now valid because `y` is mutable
    println!("the value of y after updating: {y}");

    //scope of variables
    {
        let z: i32 = 3000;
        println!("the value of z is: {z}");
    }
    // let s = z; // This line will cause a compilation error because `z` is out of scope
    // println!("the value of s is: {s}"); // This line will also cause a compilation error

    // Shadowing
    let a: i32 = 10;
    let a: i32 = a + 5; // Shadowing the previous `a`
    println!("the value of a is: {a}");

    let a = "Three"; // do not change this line
    let a = 10; // do not change the name of variable 'a'
    println!("a is: {}", a); // This line will cause a compilation error because `a` is now an integer
}
