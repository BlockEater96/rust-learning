//rules of ownership in Rust
// 1. Each value in Rust has a variable that is its "owner".
// 2. A value can only have one owner at a time.
// 3. When the owner goes out of scope, the value will be dropped.
// 4. Borrowing allows you to temporarily use a value without taking ownership.
#![allow(unused)]
fn main() {
    println!("======Ownership example in Rust!======");
    let s1: String = String::from("Hello, Rust!");
    let _s2 = s1.clone(); // Ownership of the string is moved from s1 to s2
    println!("s1: {}", s1); // This line would cause a compile-time error because s1 is no longer valid
    let vec_1: Vec<i32> = vec![1, 2, 3];
    //take_ownership(vec_1); // Ownership of vec_1 is moved to the function
    take_ownership;
    println!("vec_1: {:?}", vec_1); // This line would cause a compile-time error because vec_1 is no longer valid
}

fn generate_string() -> String {
    let some_string = String::from("I will generate a string");
    some_string
}

fn take_ownership(vec: Vec<i32>) {
    println!("Taking ownership of the vector: {:?}", vec);
    // The vector is dropped here when it goes out of scope
}
