#![allow(unreachable_code, unused_labels)]

fn main() {
    let mut count = 0u32;
    println!("Let's count until infinity!");
    loop {
        count += 1;
        if count == 3 {
            println!("Three!");

            // Skip the rest of this iteration
            continue;
        }

        println!("count is {}", count);
        if count == 5 {
            println!("OK, that's enough!");
            // Exit the loop
            break;
        }
    }

    'outer: loop {
        println!("Outer loop.");
        'inner: loop {
            println!("Inner loop.");
            break 'outer;
        }
        // This point will never be reached
        println!("This point will never be reached");
    }

    println!("Exited outer loop.");

    // Returning from a loop
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            // Return the value from the loop
            break counter * 2;
        }
    };
    println!("The result is {}", result);

    // While loop

    // Counter variable
    let mut n = 1;

    while n < 101 {
        if n % 15 ==0 {
            println!("FizzBuzz");
        } else if n % 3 == 0 {
            println!("Fizz");
        } else if n % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", n);
        }
        // Increment the counter
        n += 1;
    }

    println!("\n");


    /**
     * For and range
     */

    // `n` will take the values from 1 to 100
    for n in 1..101 {
        if n % 15 ==0 {
            println!("FizzBuzz");
        } else if n % 3 == 0 {
            println!("Fizz");
        } else if n % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", n);
        }
    }

    println!("\n");

    /**
     * Alternatively, a..=b can be used for a range that is inclusive of both ends
     */

    for m in 1 ..=100 {
        if m % 15 ==0 {
            println!("FizzBuzz");
        } else if m % 3 == 0 {
            println!("Fizz");
        } else if m % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", m);
        }
    }

    println!("\n");

    /**
     * For and iterators
     */

    let names = vec!["Peter", "Paul", "Mary"];

    for name in names.iter() {
        match name {
            &"Peter" => println!("Found Peter!"),
            &"Paul" => println!("Found Paul!"),
            _ => println!("Found someone else!"),
        }
    }

    println!("names = {:?}", names);

    /**
     * iter_mut() can be used to iterate over mutable references to the elements of a vector
     */

    let mut other_names = vec!["Victor", "Tom", "Adam"];
    for name in other_names.iter_mut() {
        *name = match name {
            &mut "Victor" => "Vicky",
            &mut "Tom" => "Thomas",
            _ => name,
        };
    }

    println!("other_names = {:?}", other_names);

}