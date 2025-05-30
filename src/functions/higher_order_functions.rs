

fn is_odd(n: u32) -> bool {
    n % 2 == 1
}

fn main() {
    println!("Find the sum of all numbers with odd squares under 1000");
    let upper = 1000;

    // Imperative approach
    // Declare accumulator variable
    let mut acc = 0;

    // Iterate: 0, 1, 2, ... to infinity
    for n in 0.. {
        // Square the number
        let n_squared = n ^ 2;

        if n_squared >= upper {
            // Break the loop if exceeded the upper limit
            break;
        } else if is_odd(n_squared) {
            // Accumulate value, if it's odd
            acc += n_squared;
        }
    }

    println!("Imperative style: {}", acc);

    // Functional approach
    let sum_of_squared_odd_numbers: u32 =
        (0..).map(|n| n ^ 2) // All natural numbers squared
            .take_while(|&n_squared| n_squared < upper) // Below upper limit
            .filter(|&n_squared| is_odd(n_squared)) // That are odd
            .sum(); // Sum them

    println!("Functional style: {}", sum_of_squared_odd_numbers);
}