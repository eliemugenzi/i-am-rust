use std::mem;

fn main() {
    // Suffixed literals, their types are known at initialization
    let x = 1u8;
    let y = 2u32;
    let z = 3u32;

    // Unsuffixed literals, their types depend on how they are used
    let a = 1; // i32
    let f = 1.0; // f64

    // `size_of_val` returns the size of a variable in bytes
    println!("size of x is {}", mem::size_of_val(&x));
    println!("size of y is {}", mem::size_of_val(&y));
    println!("size of z is {}", mem::size_of_val(&z));
    println!("size of a is {}", mem::size_of_val(&a));
    println!("size of f is {}", mem::size_of_val(&f));

}