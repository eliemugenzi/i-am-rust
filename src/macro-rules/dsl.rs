
macro_rules! calculate {
    (eval $e: expr) => {
        let val: usize = $e; // Force types to be unsigned integers
        println!("{} = {}", stringify!{$e}, val);
    };
}

fn main() {
    calculate! {
        eval 1 + 2 // `eval` is not a Rust keyword
    }

    calculate! {
        eval (1 + 2) * (3 / 4)
    }
}