extern crate rand;
use rand::{Fill};
use std::fmt::Debug;

fn print_it(input: impl Debug + 'static) {
    println!("'static value passed in is: {:?}", input);
}
// make a constant with `'static` lifetime
static NUM: i32 = 18;

// Returns a reference to `NUM` where its 'static lifetime is coerced to that of the input argument
fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM
}

fn random_vec() -> &'static [usize; 100] {
    let mut rng = rand::thread_rng();
    let mut boxed = Box::new([0; 100]);
    boxed.try_fill(&mut rng).unwrap();
    Box::leak(boxed)
}

// `elided_input` and `annotated_input` essentially have identical signatures
// because the lifetime of `elided_input` is interred by the compiler:

fn elided_input(x: &i32) {
    println!("`elided_input` x: {}", x);
}

fn annotated_input<'a>(x: &'a i32) {
    println!("`annotated_input` x: {}", x);
}


// Similarly, `elided_pass` and `annotated_pass` have identical signatures
// because the lifetime is added implicitly to `elided_pass`

fn elided_pass(x: &i32) -> &i32 { x }
fn annotated_pass<'a>(x: &'a i32) -> &'a i32 { x }

fn main() {
    {
        // Make a string literal and print it
        let static_string = "I am in read-only memory";
        println!("static_string: {}", static_string);

        // When `static_string` goes out of scope, the reference can no longer be used
        // but the data remains in the binary
    }

    {
        // Make an integer to use for `coerce_static`
        let lifetime_num = 9;

        // Coerce `NUM` to lifetime of `lifetime_num`
        let coerced_static = coerce_static(&lifetime_num);
        println!("coerced_static: {}", coerced_static);
    }

    println!("NUM: {} stays accessible", NUM);

    let first: &'static [usize; 100] = random_vec();
    let second: &'static [usize; 100] = random_vec();
    assert_ne!(first, second);
    
    // i is owned and contains no references, thus it's 'static
    let i = 5;
    print_it(i);
    
    let x = 3;
    elided_input(&x);
    annotated_input(&x);
    
    println!("elided_pass: {}", elided_pass(&x));
    println!("annotated_pass: {}", annotated_pass(&x));
}
