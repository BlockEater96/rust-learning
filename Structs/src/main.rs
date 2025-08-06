#![allow(unused)]
struct Car {
    owner: String,
    year: u32,
    fuel_level: f32,
    price: u32,
}

impl Car {
    fn display_car_info(&self) {
        println!(
            "Car owned by {}, year {}, fuel level {}, price {}",
            self.owner, self.year, self.fuel_level, self.price
        );
    }
    fn re_fuel(&mut self, gallons: f32) {
        self.fuel_level += gallons;
    }
    fn sell_car(self) -> Self {
        self
    }
}

fn main() {
    println!("=====Structs in Rust=====");
    let my_car = Car {
        owner: String::from("Alice"),
        year: 2020,
        fuel_level: 15.5,
        price: 20000,
    };
    let your_cat = Car {
        owner: String::from("Bob"),
        ..my_car
    };
    my_car.display_car_info();
    println!(
        "My car is owned by {}, year {}, fuel level {}, price {}",
        my_car.owner, my_car.year, my_car.fuel_level, my_car.price
    );
    println!(
        "Your car is owned by {}, year {}, fuel level {}, price {}",
        your_cat.owner, your_cat.year, your_cat.fuel_level, your_cat.price
    );

    // Using tuple structs explanation
    println!("=====Tuple Structs in Rust=====");
    let point = Point(10, 20);
    println!("Point coordinates: ({}, {})", point.0, point.1);
    struct Point(i32, i32);
    let another_point = Point(30, 40);
    println!(
        "Another point coordinates: ({}, {})",
        another_point.0, another_point.1
    );

    // Using unit-like structs explanation
    println!("=====Unit-like Structs in Rust=====");
    struct UnitStruct;
    let unit = UnitStruct;
    // println!("Unit-like struct created: {}", unit);

    // my_car.re_fuel(5.0);
    my_car.display_car_info();
    let sold_car = my_car.sell_car();
    sold_car.display_car_info()
}
