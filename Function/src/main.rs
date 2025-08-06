fn main() {
    println!("=====Functions in Rust=====");
    let result = basic_math(10, 5); // Call the basic_math function with two integers
    println!(
        "Addition: {}, Subtraction: {}, Multiplication: {}, Division: {}",
        result.0, // Access the first element of the tuple (addition result)
        result.1, // Access the second element of the tuple (subtraction result)
        result.2, // Access the third element of the tuple (multiplication result)
        result.3, // Access the fourth element of the tuple (division result)
    );
    let (multiply, subtract, add, divide) = basic_math(20, 10); // Destructure the tuple returned by basic_math
    println!(
        "Multiply: {}, Subtract: {}, Add: {}, Divide: {}",
        multiply, // Access the multiplication result
        subtract, // Access the subtraction result
        add,      // Access the addition result
        divide,   // Access the division result
    );
    let greeting: String = {
        let first_name = "Hariom"; // Declare a variable for first name
        let last_name = "Jaiswal"; // Declare a variable for last name
        format!("Hello! {first_name} {last_name}") // Format a greeting message
    };
    println!("Greeting: {}", greeting); // Print the greeting message
}

fn addition(a: i32, b: i32) -> i32 {
    a + b // Function to add two integers
}
fn substraction(a: i32, b: i32) -> i32 {
    a - b
}
fn multiplication(a: i32, b: i32) -> i32 {
    a * b // Function to multiply two integers
}
fn division(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Division by zero is not allowed!"); // Panic if division by zero
    } else {
        a / b // Function to divide two integers
    }
}
fn basic_math(a: i32, b: i32) -> (i32, i32, i32, i32) {
    (
        addition(a, b),       // Call addition function
        substraction(a, b),   // Call substraction function
        multiplication(a, b), // Call multiplication function
        division(a, b),       // Call division function
    )
}
