// Here, Rust infers a lifetime that is as short as possible.
// The two references are then coerced to that lifetime
fn multiply<'a>(x: &'a i32, y: &'a i32) -> i32 {
    x * y
}

// <'a: 'b, 'b> reads as lifetime `'a` is at least as long as `'b`
// Here, we take in an `&'a i32` and return a `&'b i32` as a result of coercion.
fn choose_first<'a: 'b, 'b>(first: &'a i32, second: &'b i32) -> &'b i32 {
    first
}
fn main() {
 let first = 2; // Longer lifetime
    {
        let second = 3; // Shorter lifetime
        println!("The product is {}", multiply(&first, &second));
        println!("The first is {}", choose_first(&first, &second));
    }
}