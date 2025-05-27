
use std::fmt::{Debug, Display}; // A trait which implements the print marker `{:?}`

struct Cardinal;
struct Bluejay;
struct Turkey;

trait Red{}
trait Blue{}

impl Red for Cardinal{}
impl Blue for Bluejay{}

// These functions are only valid for types which implement these traits
// The fact that the traits are empty is irrelevant
fn red<T: Red>(_: &T) -> &'static str {"red"}
fn blue<T: Blue>(_: &T) -> &'static str {"blue"}
trait HasArea {
    fn area(&self) -> f64;
}

#[derive(Debug)]
struct Rectangle { length: f64, height: f64 }

#[allow(dead_code)]
struct Triangle { length: f64, height: f64 }

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.length * self.height
    }
}

// The generic T must implement Debug. Regardless
// of the type, this will work properly
fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}

// T must implement HasArea. Any type which meets
// the bound can access HasArea's function area
fn area<T: HasArea>(t: &T) -> f64 {t.area()}

// Multiple bounds
fn compare_prints<T: Debug + Display>(t: &T) {
    println!("Debug: `{:?}`", t);
    println!("Display: `{}`", t);
}

fn compare_types<T: Debug, U: Debug>(t: &T, u: &U) {
    println!("t: `{:?}`", t);
    println!("u: `{:?}`", u);
}


// Generics on where clauses
trait PrintInOption {
    fn print_in_option(self);
}

// Because we would otherwise have to express this as `T: Debug`
// or use another method of indirect approach, this requires a `where` clause:
impl<T> PrintInOption for T where Option<T>: Debug {
    // We want Option<T> as our bound because that is what's
    // being printed. Doing otherwise would be using the wrong bound
    fn print_in_option(self) {
        println!("{:?}", Some(self));
    }
}
fn main() {
    let rectangle = Rectangle { length: 3.0, height: 4.0 };
    let _triangle = Triangle { length: 3.0, height: 4.0 };

    print_debug(&rectangle);
    println!("The area: {}", area(&rectangle));

    //print_debug(&_triangle);
    //println!("Area: {}", area(&_triangle));
    // ^ TODO: Try uncommenting these.
    // | Error: Does not implement either `Debug` or `HasArea`.


    let cardinal = Cardinal;
    let blue_jay = Bluejay;
    let _turkey = Turkey;

    // red() won't work on a blue jay nor vice-versa
    // because of the bounds
    println!("A cardinal is {}", red(&cardinal));
    println!("A blue jay is {}", blue(&blue_jay));
    //println!("A turkey is {}", red(&_turkey));
    // ^ TODO: Try uncommenting this line.

    let string = "words";
    let array = [1,2,3];
    let vec = vec![1,2,3];

    compare_prints(&string);
    // compare_prints(&array);
    // TODO ^ Try uncommenting this.
    compare_types(&array, &vec);
    vec.print_in_option();
    string.print_in_option();
}
