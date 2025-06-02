
struct Fibonacci {
    curr: u32,
    next: u32,
}

// Implement `Iterator` for Fibonacci
// The `Iterator` trait only requires a method to be defined for the `next` element,
// and an `associated type` to declare the return type of the iterator
impl Iterator for Fibonacci {
    // We can refer to this type using Self::Item
    type Item = u32;


    // Here, we define the sequence using `.curr` and `.next`
    // The return type is `Option<T>`
    // When the `Iterator` is finished, `None` is returned
    // We use Self::Item in the return type, so we can change the type
    // without having to update the function signatures
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr;
        self.curr = self.next;
        self.next = current + self.next;

        // Since there's no endpoint to a Fibonacci sequence, the Iterator
        // will never return `None` and `Some` is always returned
        Some(current)
    }
}

fn fibonacci() -> Fibonacci {
    Fibonacci { curr: 0, next: 1 }
}

fn main() {
    // `0..3` is an Iterator that generates 0,1, and 2
    let mut sequence = 0..3;
    println!("Four consecutive `next` calls on 0..3");
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());

    // The `take(n)` method reduces an `Iterator` to its first `n` terms
    for i in fibonacci().take(10) {
        println!("Next fibonacci number: {}", i);
    }

    // The `skip(n)` method shortens an Iterator by dropping its first `n` terms
    for i in fibonacci().skip(4).take(10) {
        println!(">: {}", i);
    }

    let array = [1u32, 3, 3, 7];

    // The iter method produces an `Iterator` over an array/slice
    println!("Iterate the following array {:?}", &array);
    for i in array.iter() {
        println!("> {:?}", i);
    }

    // `for` works through an `Iterator` until it returns None
    // Each `Some` value is unwrapped and bound to a variable (here, `i`)
    println!("Iterate through 0..3 using `for`");
    for i in 0..3 {
        println!("> {:?}", i);
    }
}