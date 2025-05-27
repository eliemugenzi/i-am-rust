// `print_refs` takes two references to i32 which have different
// lifetimes `'a` and `'b`. These two lifetimes must both be at
// least as long as the function `print_refs`
fn print_refs<'a, 'b> (x: &'a i32, y: &'b i32) {
    println!("x is {} and y is {}", x, y);
}

// A function which takes no arguments, but has a lifetime parameter `'a`
fn failed_borrow<'a>() {
    let _x = 12;
    // ERROR: `_x` does not live long enough
    let _y: &'a i32 = &_x;
}

// Lifetimes are annotated below with lines denoting the creation
// and destruction of each variable
// `i` has the longest lifetime because its scope entirely encloses
// both `borrow1` and `borrow2`. The duration of `borrow1` compared
// to `borrow2` is irrelevant since they are disjoint.

fn main() {
    let i = 3; // The lifetime for `i` starts
    {
        let borrow1 = &i; // `borrow1` lifetime starts
        println!("borrow1: {}", borrow1);
    } // `borrow1` ends
    {
        let borrow2 = &i; // `borrow2` lifetime starts
        println!("borrow2: {}", borrow2);
    } // `borrow2` ends
} // lifetime ends