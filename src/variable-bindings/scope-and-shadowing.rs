
fn main() {
    let long_lived_binding = 1u32;

    // This is a block, and has a smaller scope than the main function
    {
        // This binding only exists in this block
        let short_lived_binding = 2u32;
        println!("Inner short: {}", short_lived_binding);
    } // End of the block

    // Error! `short_lived_binding` doesn't exist in this scope
    // println!("short_lived_binding: {}", short_lived_binding);

    println!("Outer long: {}", long_lived_binding);

    let shadowed_binding = 1;

    {
        println!("Before being shadowed: {}", shadowed_binding);

        let shadowed_binding = "abc";
        println!("Shadowed in inner block: {}", shadowed_binding);
    }

    println!("Outside inner block: {}", shadowed_binding);

    // This binding shadows the previous one
    let shadowed_binding = 3.14;
    println!("Shadowed in outer block: {}", shadowed_binding);


    /**
     * DECLARE FIRST, INITIALIZE LATER
     */
    let a_binding;

    {
        let x = 2;
        a_binding = x * x;
    }
    println!("a_binding: {}", a_binding);

    let another_binding;
    // Error! `another_binding` is not initialized
    // println!("another_binding: {}", another_binding);
    another_binding = 3;
    println!("another_binding: {}", another_binding);

    /**
     * FREEZING
     */

    let mut _mutable_integer = 7i32;
    {
        // Shadowing by immutable `_mutable_integer`
        let _mutable_integer = _mutable_integer; // `_mutable_integer` is frozen
        // Error! Cannot assign to frozen variable

        // _mutable_integer = 50; // Error! Cannot assign to frozen variable
        println!("Inner _mutable_integer: {}", _mutable_integer);

        // `_mutable_integer` goes out of scope
    }

    _mutable_integer = 3; // `_mutable_integer` is mutable again
    println!("Outer _mutable_integer: {}", _mutable_integer);
}