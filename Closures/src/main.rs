struct User {
    name: String,
    age: u8,
    salary: f64,
}
// fn validate_user(name: &str) -> bool {
//     name.len() != 0
// }
fn is_valid_user(name: &str, age: u8, simple: fn(&str) -> bool, advanced: fn(u8) -> bool) -> bool
// where
//     V1: Fn(&str) -> bool,
//     V2: Fn(u8) -> bool,
{
    simple(name) && advanced(age)
}

fn validate_user_advance(age: u8) -> bool {
    age > 18
}
fn validate_user(name: &str) -> bool {
    name.len() != 0
}
fn main() {
    println!("=====Closures in Rust=====");
    let user = User {
        name: String::from("Alice"),
        age: 30,
        salary: 50000.0,
    };
    // let validate_user = |name: &str| name.len() != 0;
    // let validate_user_advance = |age: u8| age > 18;
    // println!("User validation result: {}", validate_user(&user.name));

    let is_valid = is_valid_user(&user.name, user.age, validate_user, validate_user_advance);
    println!("Is valid user: {}", is_valid);
}
