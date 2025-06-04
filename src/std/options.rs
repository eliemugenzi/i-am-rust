
// An integer division doesn't panic
fn checked_division(dividend: i32, divisor: i32)-> Option<i32> {
    if divisor == 0 {
        None
    } else {
        Some(dividend / divisor)
    }
}

// This function handles a division that may not succeed
fn try_division(dividend: i32, divisor: i32)  {
    // `Option` values can be pattern-matched, just like other enums
    match checked_division(dividend, divisor) {
        None => println!("{} / {} failed!", dividend, divisor),
        Some(result) => println!(
            "{} / {} = {}",
            dividend, divisor, result
        )
    }
}

fn main() {
    try_division(10, 0);
    try_division(10, 2);

    // Binding None to a variable needs to be type annotated
    let none: Option<i32> = None;
    let _equivalent_none = None::<i32>;
    let optional_float = Some(0f32);

    // Unwrapping a `Some` variant will extract the value wrapped.
    println!("{:?} unwraps to {:?}", optional_float, optional_float.unwrap());

    // Unwrapping a `None` variant will `panic!`
    println!("{:?} unwraps to {:?}", none, none.unwrap());
}