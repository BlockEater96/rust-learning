#![allow(unused)]
struct Point<T, U> {
    x: T,
    y: U,
}
impl<T, U> Point<T, U> {
    fn new(x: T, y: U) -> Point<T, U> {
        Point { x, y }
    }
}
impl Point<i32, i32> {
    fn printing(&self) {
        println!("Point with i32: ({}, {})", self.x, self.y);
    }
}

fn main() {
    println!("=====Generics in Rust=====");
    let p1 = Point { x: 10, y: 20 };
    let p2 = Point { x: 30.0, y: 40.0 };
    p1.printing();
}
