#![allow(unused)]

#[derive(PartialEq)]
struct Satelite {
    name: String,
    velocity: f64, // miles per second
}

fn main() {
    let hubble = Satelite {
        name: String::from("Hubble Telescope"),
        velocity: 4.72,
    };
    let gps = Satelite {
        name: String::from("GPS"),
        velocity: 2.42,
    };

    println!("hubble == gps is {}", hubble == gps);
}