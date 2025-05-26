// Unlike C/C++, there's no restriction on the order of function definitions in Rust.

struct Point {
    x: f64,
    y: f64,
}

impl Point {
    // This is an associated function because this function is associated
    // with a particular type, that is, Point

    // Associated functions don't need to be called with an instance
    // These functions are generally used like constructors
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Another associated function, taking two arguments
    fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    fn area(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;
        ((x2 - x1) * (y2 - y1)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;
        2.0 * ((x2 - x1).abs() + (y2 - y1).abs())
    }

    fn translate(&mut self, dx: f64, dy: f64) {
        self.p1.x += dx;
        self.p1.y += dy;
        self.p2.x += dx;
        self.p2.y += dy;
    }
}

// Pair owns resources: two heap-allocated i32 values
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // This function takes ownership of the Pair
    fn destroy(self) {
        let Pair(a, b) = self;
        // The destructor is called automatically when the object goes out of scope
        println!("Destroying Pair({}, {})", a, b);
    }
}

fn main() {
    // We can use this function here, and define it somewhere later
    fizbuzz_to(100);

    let rectangle = Rectangle {
        p1: Point::origin(),
        p2: Point::new(10.0, 5.0),
    };

    // Methods are called with a dot operator
    // Note that the first element `&self` is passed implicitly
    // i.e `rectangle.area()` is equivalent to `Rectangle::area(&rectangle)`
    println!("Area: {}", rectangle.area());
    println!("Perimeter: {}", rectangle.perimeter());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(5.0, 5.0),
    };

    // Error! rectangle is immutable
    // rectangle.translate(1.0, 1.0);

    // Okay! Mutable objects can call mutable methods
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));
    pair.destroy();
}

fn is_divisible_by(n: u32, d: u32) -> bool {
    if d == 0 {
        return false;
    }
    // This is an expression, the `return` keyword is not needed
    n % d == 0
}

fn fizbuzz(n: u32) -> () {
    if is_divisible_by(n, 15) {
        println!("fizzbuzz");
    } else if is_divisible_by(n, 5) {
        println!("buzz");
    } else if is_divisible_by(n, 3) {
        println!("fizz");
    } else {
        println!("{}", n);
    }
}

fn fizbuzz_to(n: u32) -> () {
    for i in 1..=n {
        fizbuzz(i);
    }
}
