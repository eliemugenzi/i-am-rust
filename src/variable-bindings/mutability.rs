

fn main() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    // Copy `an_integer` to a new variable
    let copied_integer = an_integer;

    println!("an_integer: {}", an_integer);
    println!("a_boolean: {}", a_boolean);
    println!("unit: {:?}", unit);

    // The compiler warns about unused variable bindings; these warnings can be silenced by prefixing the variable name with an underscore
    let _unused_variable = 3u32;

    let noisy_unused_variable = 2u32;
    // FIXME ^ Prefix with an underscore to suppress the warning

    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);

    // Ok
    mutable_binding += 1;
    println!("After mutation: {}", mutable_binding);

    // Error! Cannot assign a new value to immutable variable
    // _immutable_binding += 1;
}