#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
fn main() {
    println!("=====Data Types in Rust=====");
    // Integer types
    let a: i32 = 10; // Signed 32-bit integer range: -2,147,483,648 to 2,147,483,647
    let b: u32 = 20; // Unsigned 32-bit integer range: 0 to 4,294,967,295
    println!("Signed Integer: {}, Unsigned Integer: {}", a, b);
    // Floating-point types
    let x: f32 = 3.14; // 32-bit floating point
    let y: f64 = 2.718281828459045; // 64-bit floating point
    println!("Floating Point: f32 = {}, f64 = {}", x, y);
    // Boolean type
    let is_active: bool = true; // Boolean type can be true or false
    println!("Boolean Value: {}", is_active);
    // Character type
    let letter: char = 'R'; // Character type, single Unicode scalar value
    println!("Character: {}", letter);

    // Tuple type
    let tuple: (i32, f64, char) = (42, 3.14, 'A'); // Tuple containing different types
    println!("Tuple: ({}, {}, {})", tuple.0, tuple.1, tuple.2);
    // Array type
    let array: [i32; 3] = [1, 2, 3]; // Array of fixed size with elements of the same type
    println!("Array: {:?}", array);
    let array1: [i32; 5] = [0, 2, 3, 4, 5]; // Array initialized with the same value
    println!(
        "Array with same value: {:?}, sum of array is: {}",
        array1,
        sum(array1)
    );
    // Slice type
    let slice: &[i32] = &array1[1..4]; // Slice of an array, referencing a portion of the array
    let slice1: &[i32] = &slice[1..3];
    println!("Slice: {:?}, {:?}", slice, slice1);
    // String type
    let string: String = String::from("Hello, Rust!"); // String type, dynamically sized UTF-8 encoded string
    println!("String: {}", string);
    // String slice type
    let string_slice: &str = "Hello, Rust!"; // String slice, a reference to a string
    println!("String Slice: {}", string_slice);

    //vector type
    let vector: Vec<i32> = vec![1, 2, 3, 4, 5]; // Vector, a growable array type
    println!("Vector: {:?}", vector);
    // Vector with same value
    let vector1: Vec<i32> = vec![0; 5]; // Vector initialized with the same value
    println!(
        "Vector with same value: {:?}, sum of vector is: {}",
        vector1,
        vector_sum(vector.clone())
    );
    //type alias
    type IntAlias = i32; // Type alias for i32
    type tupleAlias = (i32, f64); // Type alias for a tuple of i32 and
}

// Function to calculate the sum of an array of integers
// Takes an array of 5 integers and returns their sum
fn sum(arr: [i32; 5]) -> i32 {
    let mut total = 0;
    for &sum in arr.iter() {
        total += sum; // Add each element to the total
    }
    return total; // Return the total sum
}

fn vector_sum(arr: Vec<i32>) -> i32 {
    let mut total = 0;
    for &sum in arr.iter() {
        total += sum; // Add each element to the total
    }
    return total; // Return the total sum
}
