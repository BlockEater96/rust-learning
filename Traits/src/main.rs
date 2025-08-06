// struct Square {
//     side: f32,
//     line_width: u8,
//     color: String,
// }
// struct Rectangle {
//     length: f32,
//     width: f32,
//     line_width: u8,
//     color: String,
// }
// // impl Square {
// //     fn calculate_area(&self) {
// //         println!(
// //             "Area of square with side {} is {}",
// //             self.side,
// //             self.side * self.side
// //         );
// //     }
// // }
// // impl Rectangle {
// //     fn area(&self) -> f32 {
// //         self.length * self.width
// //     }
// // }

// trait Shape + SomeTrait + OtherTrait {
//     fn calculate_area(&self) -> f32;
//     fn perimeter(&self) -> f32 {
//         println!("Calculating perimeter...");
//         0.0 // Default implementation, can be overridden
//     }
// }
// impl Shape for Rectangle {
//     fn calculate_area(&self) -> f32 {
//         let area_of_rectangle = self.length * self.width;
//         println!(
//             "Area of rectangle with length {} and width {} is {}",
//             self.length, self.width, area_of_rectangle
//         );
//         area_of_rectangle
//     }

//     fn perimeter(&self) -> f32 {
//         let perimeter_of_rectangle = 2.0 * (self.length + self.width);
//         println!(
//             "Perimeter of rectangle with length {} and width {} is {}",
//             self.length, self.width, perimeter_of_rectangle
//         );
//         perimeter_of_rectangle
//     }
// }
// impl Shape for Square {
//     fn calculate_area(&self) -> f32 {
//         let area_of_square = self.side * self.side;
//         println!(
//             "Area of square with side {} is {}",
//             self.side, area_of_square
//         );
//         area_of_square
//     }
// }

// fn shape_properties<T: Shape>(object: T) {
//     object.calculate_area();
//     println!("Perimeter: {}", object.perimeter());
// }

// fn return_shape() -> impl Shape {
//     Square {
//         side: 4.0,
//         line_width: 1,
//         color: String::from("Green"),
//     }
// }
// struct Circle {
//     radius: f32,
// }

// trait Draw {
//     fn draw(&self);
// }

// impl Draw for Circle {
//     fn draw(&self) {
//         println!("Drawing a circle with radius {}", self.radius);
//     }
// }
// trait OtherTrait {}
// impl OtherTrait for Circle {}
// impl OtherTrait for Square {}
// impl OtherTrait for Rectangle {}

// trait SomeTrait {}
// impl SomeTrait for Circle {}
// impl SomeTrait for Square {}
// impl SomeTrait for Rectangle {}

// fn shape_properties_static<T: Shape>(object: T)
// where
//     T: Shape + SomeTrait + OtherTrait,
// {
//     object.calculate_area();
//     println!("Perimeter: {}", object.perimeter());
// }

// fn shape_properties_dynamic(object: Box<dyn Shape + SomeTrait + OtherTrait>) {
//     object.calculate_area();
//     object.perimeter();
// }

//derived traits
#[derive(Debug, PartialEq)]
struct Student {
    name: String,
    age: u8,
    sex: char,
}

fn main() {
    println!("=====Traits in Rust=====");
    // let square = Square {
    //     side: 5.0,
    //     line_width: 2,
    //     color: String::from("Red"),
    // };
    // let rectangle = Rectangle {
    //     length: 10.0,
    //     width: 5.0,
    //     line_width: 3,
    //     color: String::from("Blue"),
    // };
    // square.calculate_area();
    // rectangle.calculate_area();
    // println!("Perimeter of square: {}", square.perimeter());
    // println!("Perimeter of rectangle: {}", rectangle.perimeter());
    // let cd: Circle = Circle { radius: 3.0 };

    // let circle_shape: Box<dyn Shape + SomeTrait + OtherTrait> = Box::new(cd);
    // shape_properties_dynamic(circle_shape);
    // shape_properties_static(square);
    // shape_properties_static(rectangle);
    // shape_properties_static(return_shape());
    // let circle = Circle { radius: 2.0 };
    // circle.draw();
    let student = Student {
        name: String::from("Alice"),
        age: 20,
        sex: 'M',
    };
    let student2 = Student {
        name: String::from("Bob"),
        age: 22,
        sex: 'M',
    };
    println!("Student: {:?}", student);
    println!("student and student2 are equal: {}", student == student2);
}
