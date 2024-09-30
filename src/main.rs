#![allow(unused)]

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
}

