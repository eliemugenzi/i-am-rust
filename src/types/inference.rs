

fn main() {
    // Because of the annotation, the compiler knows that elem is a u8
    let elem = 5u8;

    // Create an empty vector(a growable array)
    let mut vec = Vec::new();

    // At this point the compiler doesn't know the exact type of `vec`, 
    // it just knows that it's a vector of something (Vec<_>)
    vec.push(elem);
    vec.push(6u8);

    // Aha! Now the compiler knows that `vec` is a vector of `u8s` (Vec<u8>)
    println!("vec: {:?}", vec);
}