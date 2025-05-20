use std::convert::{ TryFrom, TryInto };
use std::fmt;
use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(format!("{} is not an even number", value))
        }
    }
}

#[derive(Debug)]
struct Circle {
    radius: f32,
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle with radius: {}", self.radius)
    }
}

impl FromStr for Circle {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<f32>() {
            Ok(radius) => Ok(Circle { radius }),
            Err(e) => Err(e),
        }
    }
}

fn main() {
    // Try from
    assert_eq!(EvenNumber::try_from(4), Ok(EvenNumber(4)));
    assert_eq!(EvenNumber::try_from(5), Err("5 is not an even number".to_string()));

    // Try into
    let even_number: Result<EvenNumber, _> = 6.try_into();
    assert_eq!(even_number, Ok(EvenNumber(6)));

    // Convert to String
    let circle = Circle { radius: 5.0 };
    println!("{}", circle.to_string());

    // Parsing a string
    let parsed: i32 = "42".parse().unwrap();
    let turbo_parsed = "38".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("The sum of {} and {} is {}", parsed, turbo_parsed, sum);

    let radius = "    3";
    let circle_: Circle = radius.parse().unwrap();
    println!("Parsed circle: {:?}", circle_);
}