use std::collections::HashMap;
fn main() {
    println!("=====Hash Maps in Rust=====");
    let mut person: HashMap<&str, i32> = HashMap::new();
    person.insert("Alice", 30);
    person.insert("Bob", 25);
    person.insert("Charlie", 35);
    person.entry("Diana").or_insert(28);
    person.entry("Alice").or_insert(32); // This won't change Alice's age
    println!("Initial HashMap: {:?}", person);
    println!(" the age of Alice is: {}", person.get("Alice").unwrap());

    if person.contains_key("Bob") {
        println!("Bob is in the HashMap.");
    } else {
        println!("Bob is not in the HashMap.");
    }
    match person.get("Charlie") {
        Some(&age) => println!("Charlie's age is: {}", age),
        None => println!("Charlie is not in the HashMap."),
    }
    for (name, age) in &person {
        println!("{} is {} years old.", name, age);
    }
}
