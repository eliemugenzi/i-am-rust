use std::fmt::{ self, Formatter, Display };

struct City {
    name: &'static str,
    lat: f32, // Latitude
    lon: f32, // Longitude
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Display for City {
    // `f` is a buffer, and this method must write the formatted string to it
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >=0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >=0.0 { 'E' } else { 'W' };
        write!(f, "{}: {:.3}°{} {:.3}°{}", self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
    }
}

fn main() {
    println!("{} days", 21);
    // Positional arguments
    println!("{0}, this is {1}. {1}, this is {0}.", "Alice", "Bob");

    // Named arguments
    println!(
        "{name} is {age} years old",
        name = "Alice",
        age = 21
    );

    // Different formatting can be invoked by specifying the format character
    println!("Base 10: {}", 69420);
    println!("Base 2 (binary): {:b}", 69420);
    println!("Base 8 (octal): {:o}", 69420);
    println!("Base 16 (hex): {:x}", 69420);

    println!("{number:>5}", number = 1);
    println!("{number:0>5}", number = 1);
    println!("{number:0<5}", number = 1);
    println!("{number:0^5}", number = 1);
    println!("{number:.2}", number = 1.23456);
    println!("{number:.3}", number = 1.23456);

    println!("{number:0>width$}", number = 3, width = 5);

    for city in [
        City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
        City { name: "London", lat: 51.5085, lon: -0.1257 },
        City { name: "New York", lat: 40.712776, lon: -74.005974 },
    ] {
        println!("{}", city);
    }

    for color in [
        Color { red: 255, green: 0, blue: 0 },
        Color { red: 0, green: 255, blue: 0 },
        Color { red: 0, green: 0, blue: 255 },
    ] {
        println!("Color - R:{}, G:{}, B:{}", color.red, color.green, color.blue);
    }
}