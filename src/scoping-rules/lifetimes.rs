// `print_refs` takes two references to i32 which have different
// lifetimes `'a` and `'b`. These two lifetimes must both be at
// least as long as the function `print_refs`
fn print_refs<'a, 'b> (x: &'a i32, y: &'b i32) {
    println!("x is {} and y is {}", x, y);
}

// A function which takes no arguments but has a lifetime parameter `'a`
fn failed_borrow<'a>() {
    let _x = 12;
    // ERROR: `_x` does not live long enough
    // let _y: &'a i32 = &_x;
    // EXPLANATION: This code fails because:
    // 1. The variable `_x` is local to the function and has a lifetime limited to the function's scope.
    // 2. The reference `_y` is annotated with lifetime `'a`, which is a generic lifetime parameter.
    // 3. In Rust, generic lifetime parameters like `'a` can potentially live for any duration,
    //    including beyond the function's scope.
    // 4. Since `_y` is a reference to `_x`, Rust requires that `_x` lives at least as long as `_y`.
    // 5. But `_x` only lives until the end of the function, while `_y` with lifetime `'a` 
    //    could potentially live longer.
    // 6. This creates a lifetime mismatch - a short lifetime (`_x`) cannot be coerced into a longer one (`'a`).
    // 7. The borrow checker prevents this to ensure memory safety and avoid dangling references.
    // A short lifetime cannot be coerced into a longer one
}

// One input reference with a lifetime parameter `'a`
// which must live at least as long s the function
fn print_one<'a>(x: &'a i32) {
    println!("print_one: x is {}", x);
}

// Mutable references are possible with lifetimes as well
fn add_one<'a>(x: &'a mut i32) {
    *x += 1;
}

// Multiple elements with different lifetimes. In this case, it
// would be fine for both to have the same lifetime `'a`, but in more
// complex cases, different lifetimes may be required.
fn print_multi<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("print_multi: x is {} and y is {}", x, y);
}

// Returning references that have been passed in is acceptable.
// However, the correct lifetime must be returned
fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32 {
    x
}

// fn invalid_output<'a>() -> &'a String {&String::from("hello")}
// The above is invalid: `'a` must live longer than the function.
// Here, `&String::from('hello')` would create a `String`, followed by a
// reference. Then the data is dropped upon exiting the scope, leaving
// a reference to invalid data to be returned


struct Owner(i32);

impl Owner {
    fn add_one<'a>(&'a mut self) {
        self.0 += 1;
    }

    fn print<'a>(&'a self) {
        println!("print: x is {}", self.0);
    }
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

    let (four, nine) = (4, 9);

    print_refs(&four, &nine);

    failed_borrow();
    // failed_borrow contains no references to force `'a` to be
    // longer than the lifetime of the function, but `'a` is longer
    // Because the lifetime is never constrained, it defaults to `'static`

    let x = 7;
    let y = 9;
    print_one(&x);
    print_multi(&x, &y);

    let z = pass_x(&x, &y);
    print_one(z);

    let mut t = 3;
    add_one(&mut t);
    print_one(&t);

    let mut owner = Owner(13);
    owner.add_one();
    owner.print();

} // lifetime ends
