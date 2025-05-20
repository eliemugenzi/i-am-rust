// Globals are declared outside all other scopes
static LANGUAGE: & str = "Rust";
static VERSION: & str = "1.0.0";
const THRESHOLD: i32 = 10; // Constants are declared using `const` keyword

fn is_big(n: i32) -> bool {
    // Access constant in some function/scope
    n > THRESHOLD
}

fn main() {
    let n = 20;

    // Access the constant in the main thread
    println!("This is {} version {}", LANGUAGE, VERSION);
    println!("Is {} big? {}", n, is_big(n));
}