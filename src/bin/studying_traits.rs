#![allow(unused)]

use std::fmt::Display;

struct Satelite {
    name: String,
    velocity: f64, // miles per second
}

struct SpaceStation {
    name: String,
    crew_size: u8,
    altitude: u32, // miles
}

trait Description {
    fn describe(&self) -> String {
        String::from("This is an object flying through space!")
    }
}

impl Description for Satelite {} // Using default implementation

impl Description for SpaceStation {
    fn describe(&self) -> String {
    format!("The {} flying {} miles high with {} crew members aboard!", self.name, self.altitude, self.crew_size)
    }
}

fn main() {
    let hubble = Satelite {
        name: String::from("Hubble Telescope"),
        velocity: 4.72,
    };
    let iss = SpaceStation {
        name: String::from("International Space Station"),
        crew_size: 6,
        altitude: 254,
    };

    println!("{}", hubble.describe());
    println!("{}", iss.describe());
}