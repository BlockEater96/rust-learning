fn main() {
    println!("=====Condition Flow in Rust=====");
    'outer: loop {
        println!("This is the outer loop");
        break 'outer; // Break out of the outer loop
    }
    let vec: Vec<i32> = vec![1, 2, 3, 4, 5]; // Create a vector of integers
    for i in vec {
        if i == 3 {
            println!("Fount 3 ! skipping rest of the loop");
            continue; // Skip the rest of the loop when i is 3
        }
        println!("Current value: {}", i); // Print the current value
    }

    let mut num: i32 = 0;
    while num <= 5 {
        println!("Current number: {}", num);
        num += 1; // Increment the number
    }
}
