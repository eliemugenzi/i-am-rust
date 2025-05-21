
struct Foo {
    x: (u32, u32),
    y: u32,
}

struct Bar {
    foo: Foo,
}

#[allow(dead_code)]
enum Temperature {
  Celsius(i32),
  Fahrenheit(i32),
}

enum Color {
    // These 3 are specified solery by their name
    Red,
    Green,
    Blue,
    // These likewise tie `u32` tuples to different names: color models
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
}

fn some_number() -> Option<u32> {
    Some(42)
}

fn age() -> u32 {
    13
}

fn main() {
    let number = 13;
    println!("Tell me about {}", number);

    match number {
        // match a single value
        1 => println!("One!"),
        // match multiple values
        2 | 3 | 5 | 7 |11 => println!("A prime!"),
        // match a range of values
        13..=19 => println!("A teen!"),
        20..=29 => println!("A twenty something!"),
        30..=39 => println!("A thirty something!"),
        // Handle the rest of cases
        _ => println!("Ain't special!"),

    }

    let boolean = true;

    // Match is an expression too
    let binary = match boolean {
        true => 1,
        false => 0,
    };
    println!("{} is {}", boolean, binary);

    /**
     * Destructuring
     */

    // Destructuring a tuple
    let triple = (0, -2, 3);
    println!("Tell me about {:?}", triple);

    match triple {
        // Destructure the second and third elements
        (0, y, z) => println!("y: {}, z: {}", y, z),
        // .. can be used to ignore the rest of the elements
        (1, ..) => println!("First element is 1"),
        (.., 2) => println!("Last element is 2"),
        (3, .., 4) => println!("First element is 3 and last element is 4"),
        _ => println!("It doesn't matter what they are"),
    }
    // Destructuring an array/slice
    let array = [1, -2, 6];

    match array {
        [0, second, third] => println!("array[0]=0, array[1]={}, array[2]={}", second, third),
        [1, _, third] => println!("array[0]=1array[0]=1, array[2]={}", third),
        [-1, second, ..] => println!("array[0]=-1, array[1]={}", second),
        
        [3, second, tail @ ..] => {
            println!("array[0]=3, array[1]={}", second);
            println!("The tail is {:?}", tail);
        },
        // Combining these patterns, we can, for example, bind the first and
        // last values, and store the rest in a slice:
        [first, middle @ .., last] => {
            println!("array[0]={}, array[1..]={:?}, array[last]={}", first, middle, last);
        },
        _ => println!("It doesn't matter what they are"),
    }

    // Destructuring enum

    let color = Color::RGB(122, 17, 40);
    println!("What color is it?");

    // An `enum` can be destructured in a match statement
    match color {
        Color::Red => println!("Red"),
        Color::Green => println!("Green"),
        Color::Blue => println!("Blue"),
        Color::RGB(r, g, b) => println!("RGB({},{},{})", r, g, b),
        Color::HSV(h, s, v) => println!("HSV({},{},{})", h, s, v),
        Color::CMYK(c, m, y, k) => println!("CMYK({},{},{},{})", c, m, y, k),
        Color::HSL(h, s, l) => println!("HSL({},{},{})", h, s, l),
        Color::CMY(c, m, y) => println!("CMY({},{},{})", c, m, y),
    }

    // Destructuring pointers/refs

    let reference = &4;
    match reference {

        /**
         * If reference is pattern matched against &val, it results in a comparison like:
         * `&i32`
         * `&val`
         * We see that if the matching `&` are dropped, then the `i32` should be assigned to `val`
         */
        &val => println!("Got a value: {}", val),
    }

    // To avoid the `&`, you deference before matching
    match *reference {
        // If reference is pattern matched against val, it results in a comparison like:
        // `i32`
        // `val`
        val => println!("Got a value: {}", val),
    }

    // What if you don't start with a reference? refetence was a &
    // because the right side was already a reference. This is not a reference because the right side is not one.
    let _not_a_reference = 3;

    // Rust provides ref for exactly this purpose. It modifies the assignment so that
    // a reference is created for the element; this reference is assigned.
    let ref _is_a_reference = 3;

    // Accordingly, by defining 2 values without references, 
    // references can be retrieved via `ref` and `ref mut`

    let value = 5;
    let mut mut_value = 6;

    match value {
        ref v => {
            println!("value = {}", v);
        }
    }

    match mut_value {
        ref mut m => {
            // Got a reference. Gotta dereference befote we can do anything to it
            *m += 1;
            println!("mut_value = {}", m);
        }
    }

    // Destructuring structs
    let foo = Foo { x: (1, 2), y: 3 };

    match foo {
        Foo { x: (1, b), y } => {
            println!("x: (1, {}), y: {}", b, y);
        },
        // You can destructure structs and rename the variables
        // the order is not important
        Foo { y: 2, x: i } => {
            println!("x: {:?}, y: {}", i, 2);
        },

        // You can also ignore some variables
        Foo { y, .. } => {
            println!("y: {}", y);
        },    
    }

    let faa = Foo { x: (1, 2), y: 3 };

    // You don't need a match block to destructure a struct
    let Foo { x: x0, y: y0 } = faa;
    println!("x0: {:?}, y0: {}", x0, y0);

    let bar = Bar {foo: faa };
    let Bar {  foo: Foo { x: nested_x, y: nested_y } } = bar;
    println!("nested_x: {:?}, nested_y: {:?}", nested_x, nested_y);

    /*
     * Guards
     */
    let temperature = Temperature::Celsius(-1);

    match temperature {
        Temperature::Celsius(t) if t < 0 => println!("Freezing!"),
        Temperature::Celsius(t) if t > 100 => println!("Boiling!"),
        Temperature::Celsius(t) => println!("Celsius: {}", t),
        Temperature::Fahrenheit(t) if t < 32 => println!("Freezing!"),
        Temperature::Fahrenheit(t) if t > 212 => println!("Boiling!"),
        Temperature::Fahrenheit(t) => println!("Fahrenheit: {}", t),
    }

    let number: u8 = 4;
    match number {
        i if i==0 => println!("Zero!"),
        i if i%2 == 0 => println!("Even!"),
        i if i%2 == 1 => println!("Odd!"),
        _ => println!("Not a number!"),
    }

    /*
    * Binding
    */ 
    match some_number() {
        Some(n @ 42) => println!("The answer is {}", n),
        Some(n) => println!("Not interesting...{}", n),
        None => println!("None"),
    }

    match age() {
        0 => println!("Baby"),
        n @ 1..=12 => println!("Child"),
        n @ 13..=19 => println!("Teen"),
        n @ 20..=29 => println!("Young adult"),
        n @ 30..=39 => println!("Adult"),
        n => println!("Old"),
    }
    
    
}