#![allow(dead_code)]
fn main() {
    println!("=====Conditions in Rust=====");
    // let number = 10; // Declare a variable with an integer value
    //take number from the user
    let mut number = String::new(); // Create a mutable string to hold user input
    println!("Enter a number: "); // Prompt the user for input
    std::io::stdin()
        .read_line(&mut number) // Read a line from standard input and store it in `number`
        .expect("Failed to read line"); // Handle any errors that occur during input
    let number: i32 = number.trim().parse().expect("Please enter a valid number"); // Convert the input string to an integer, trimming whitespace and handling errors
    if number % 2 == 0 {
        println!("{} is an even number.", number); // Check if the number is even
    } else {
        println!("{} is an odd number.", number); // Otherwise, it's odd
    }
    if number > 0 {
        println!("{} is a positive number.", number);
    } else if number < 0 {
        println!("{} is a negative number.", number);
    } else {
        println!("{} is zero.", number);
    }
    let marks: i32 = 85; // Declare a variable with an integer value for marks
    println!("Marks: {}", marks); // Print the marks
    match marks {
        90..=100 => println!("Grade: A+"), // Match for marks between 90 and 100
        80..=89 => println!("Grade: A"),   // Match for marks between 80 and 89
        70..=79 => println!("Grade: B"),   // Match for marks between 70 and 79
        60..=69 => println!("Grade: C"),   // Match for marks between 60 and 69
        50..=59 => println!("Grade: D"),   // Match for marks between 50 and 59
        _ => println!("Grade: F"),         // Default case for any other value
    }
}

fn even_or_odd(num: i32) -> &'static str {
    if num % 2 == 0 {
        "even" // Return "even" if the number is even
    } else {
        "odd" // Return "odd" if the number is odd
    }
}
