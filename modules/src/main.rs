#![allow(dead_code, unused_variables)]
mod shapes;

use shapes::Type::Linear;
use shapes::Type::Circle;

fn main() {
    println!("Hello, world!");
    let s = shapes::new_shape(45.0, 20.0, Linear);
    let area = s.get_area();
    println!("The area is {area:>10}");

    let s2 = shapes::new_shape(7.0, 0.0, Circle);
    let area = s2.get_area();
    println!("The area is {area:>10}");
}

