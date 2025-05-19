use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("First element of the slice is {}", slice[0]);
    println!("The slice has {} elements", slice.len());
}

fn main() {
    // Fixed-size array (type signature is superfluous)
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // All elements are initialized to the same value
    let ys: [i32; 500] = [38;500];

    // Indexing starts at 0
    println!("first element of xs is {}", xs[0]);
    println!("second element of xs is {}", xs[1]);
    println!("third element of ys is {}", ys[2]);

    let xs_len = xs.len();
    println!("length of xs is {}", xs_len);
    println!("length of ys is {}", ys.len());
    println!("size of xs in bytes is {}", mem::size_of_val(&xs));
    println!("size of ys in bytes is {}", mem::size_of_val(&ys));

    // Arrays can be automatically borrowed as slices
    analyze_slice(&xs[1..4]);
    analyze_slice(&ys[1..4]);

    // Example of an empty slice `&[]`
    let empty_slice: &[i32] = &[];
    let empty_array: [i32; 0] = [];
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]); // Same but more verbose

    // Arrays can be safely accessed using the `get` method, which returns an `Option`. This can be matched as shown below, or used with `expect()`
    // If you would like the program to exit with a nice message instead of happily continue

    for i in 0..xs.len() + 1 { // Oops, one element too far!
        match xs.get(i) {
            Some(xval) => println!("xs[{}] = {}", i, xval),
            None => println!("Slown down! {} is too far!", i),
        }
    }

    // Out of bound indexing on array with constant value causes compilation error
    // println!("xs[5] = {}", xs[5]); // This will cause a panic at runtime
    // println!("xs[5] = {}", xs.get(5)); // This will cause a panic at runtime
    // println!("{}, xs[..][5]");
}