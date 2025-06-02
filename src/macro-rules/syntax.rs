use std::cmp::min;
macro_rules! say_hello {
    // `()` indicates that the macro takes no argument
    () => {
        // The macro will expand into the contents of this block
        println!("Hello, world!");
    };
}

macro_rules! create_function {
    // This macro takes an argument of designator `ident` and
    // creates a function named `$func_name`
    // The `ident` designator is used for variable/function names
    ($func_name: ident) => {
        fn $func_name() {
            println!("You called {:?}", stringify!($func_name));
        }
    };
}

macro_rules! print_result {
    ($expression: expr) => {
        println!("{:?}={:?}", stringify!($expression), $expression);
    };
}

create_function!(foo);
create_function!(bar);


// `test!` will compare `$left` and `$right`
// in different ways depending on how you invoke it:
macro_rules! test {

    // Arguments don't need to be separated by a comma.
    // Any template can be used!
    ($left: expr; and $right: expr ) => {
        println!("{:?} and {:?}={:?}", stringify!($left), stringify!($right), $left && $right);
    };

    // Each arm must end with a semicolon
    ($left: expr; or $right: expr ) => {
        println!("{:?} or {:?}={:?}", stringify!($left), stringify!($right), $left || $right);
    };
}

macro_rules! find_min {
    // Base case:
    ($x: expr) => ($x);
    // `$x` followed by at least one `$y`
    ($x: expr, $($y:expr), +) => (
        // Call `find_min!` on the tail `$y`
        min($x, find_min!($($y),+))
    )
}

fn main() {
    // This call will expand into println!("Hello");
    say_hello!();

    foo();
    bar();
    print_result!(1u32 + 1);

    // Recall that blocks are expressions too
    print_result!({
        let x = 1u32;
        x * x + 2 * x - 1
    });

    test!(1i32 + 1 == 2i32; and 2i32 + 2 == 4i32);
    test!(true; or false);

    println!("{}", find_min!(1));
    println!("{}", find_min!(1 + 2, 2));
    println!("{}", find_min!(5, 2 * 3, 4));
}