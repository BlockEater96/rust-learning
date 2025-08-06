struct Point {
    x: i32,
    y: i32,
}

fn main() {
    println!("=====Pattern Matching in Rust=====");
    let number = 5;
    match number {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        4 => println!("Four"),
        5 => println!("Five"),
        _ => println!("Not in the range of 1 to 5"),
    }

    let x = 3;
    if let 3 = x {
        println!("x is three");
    } else {
        println!("x is not three");
    }
    //while let
    let mut count = 0;
    while let 0 = count {
        println!("Count is zero, incrementing...");
        count += 1;
        if count == 1 {
            println!("Count is now one, breaking the loop.");
            break;
        }
    }

    //let binding
    let (a, b) = (1, 2);
    match (a, b) {
        (1, 2) => println!("a is 1 and b is 2"),
        (1, _) => println!("a is 1 and b is something else"),
        (_, 2) => println!("a is something else and b is 2"),
        _ => println!("a and b are something else"),
    }

    //functions with pattern matching
    // let result = add(2, 3);
    // println!("The result of adding 2 and 3 is: {}", result);
    //====================DeStructuring Structs Parameters====================
    let point = Point { x: 10, y: 20 };
    match point {
        Point { x, y } if x == y => println!("Point is on the line y = x at ({}, {})", x, y),
        Point { x, y } if x > y => println!("Point is above the line y = x at ({}, {})", x, y),
        Point { x, y } if x < y => println!("Point is below the line y = x at ({}, {})", x, y),
        _ => println!(
            "Point is not on the line y = x at ({}, {})",
            point.x, point.y
        ),
    }
}
