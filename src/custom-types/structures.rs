// An atribute to suppress warnings about unused code
#![allow(dead_code)]
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
struct Reactangle {
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 30;
    let peter = Person { name, age };

    // Print debug struct
    // The `{:?}` format specifier is used to print the struct in debug format
    println!("Peter: {:?}", peter);

    // Instantiate a `Point`
    let point: Point = Point { x: 5.2, y: 0.4 };
    let another_point = Point { x: 0.4, y: 5.2 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    let bottom_right = Point { x: 10.3, ..another_point };

    // Destructure the point using a `let` binding
    let Point {
        x: left_edge,
        y: top_edge,
    } = point;

    let _rectangle = Reactangle {
        // Struct instantiation is an expression too
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right: bottom_right,
    };

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let _pair = Pair(1, 0.1);

    // Access the fields of the tuple struct
    println!("pair coordinates: ({:?}, {:?})", _pair.0, _pair.1);

    // Destructure a tuple struct
    let Pair(x, y) = _pair;
    println!("pair coordinates: ({:?}, {:?})", x, y);
}
