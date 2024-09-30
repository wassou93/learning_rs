#![allow(unused)]

use std::mem;

#[derive(Debug)]
struct Rectangle<T, U> {
    width: T,
    height: U,
}

impl<T, U> Rectangle<T, U> {
    fn get_width(&self) -> &T {
        &self.width
    }
}

impl Rectangle<u8, u16> {
    fn get_perimeter(&self) -> u8 {
        (self.width + self.height as u8) * 2
    }
}

fn get_biggest<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

struct Shuttle {
    name: String, // 8+8+8 = 24 byte
    crew_size: u8, // 1 byte
    propellant: f64, // 8 bytes
} // 24 + 1 + 8 = 35 bytes after padding (crew_size: u8 +7 bytes) -> 40 bytes

fn main() {
    let rect = Rectangle {
        width: 1u8,
        height: 3u16,
    };
    println!("rect is {:?}", rect);
    println!("width is {}", rect.get_width());
    println!("perimeter is {}", rect.get_perimeter());

    let biggest = get_biggest(100, 200);
    println!("The biggest value is: {}", biggest);


    // Init Shuttle object
    let vehicle = Shuttle {
        name: String::from("Atlantis"),
        crew_size: 7,
        propellant: 835958.0,
    };
    println!("Vehicle size on stack: {} bytes", mem::size_of_val(&vehicle));

    let boxed_vehicle = Box::new(vehicle);
    println!("Boxed Vehicle size on stack: {} bytes", mem::size_of_val(&boxed_vehicle)); // 8 bytes
    println!("Boxed Vehicle size on heap: {} bytes", mem::size_of_val(&*boxed_vehicle)); // 40 bytes

    let unboxed_vehicle = *boxed_vehicle;
    println!("Unboxed Vehicle size on stack: {} bytes", mem::size_of_val(&unboxed_vehicle));
}

