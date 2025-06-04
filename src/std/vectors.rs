
fn main() {
    // Iterators can be collected into vectors
    let collected_iterator: Vec<i32> = (0..10).collect();
    println!("Collected (0..10) into {:?}", collected_iterator);

    // The `vec!` macro can be used to initialize a vector
    let mut xs = vec![1i32, 2, 3];
    println!("Initial vector: {:?}", xs);

    // Insert a new element at the end of the vector
    xs.push(4);
    println!("Vector after push: {:?}", xs);

    // The `len` method yields the number of elements currently stored in a vector
    println!("Vector length: {:?}", xs.len());

    // Indexing is done using the square brackets (indexing starts at 0)
    println!("Second element: {:?}", xs[1]);

    // `pop` removes the last element from the vector and returns it
    println!("Pop last element: {:?}", xs.pop());

    // Vectors can be easily iterated over
    println!("Contents of xs:\n");
    for x in xs.iter() {
        println!(">{}", x);
    }
    println!();

    // A vector can also be iterated over while the iteration count
    // is enumerated in a separate variable (`i`)
    for (i, x) in xs.iter().enumerate() {
        println!("In position {}, we have value: {}", i, x);
    }

    // Thanks to iter_mut, mutable `Vector`s can also be iterated over
    // in a way that allows modifying each value
    for x in xs.iter_mut() {
        *x *= 3;
    }
    println!();
    println!("Updated vector: {:?}", xs);

}