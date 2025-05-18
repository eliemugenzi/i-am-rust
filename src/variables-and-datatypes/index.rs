struct Number {
    odd: bool,
    value: i32,
}

impl Number {
    fn is_odd(&self) -> bool {
        self.odd
    }

    fn is_posititive(&self) -> bool {
        self.value > 0
    }
}

fn literals_and_operators() {
    let logical: bool = true;
    let a_float: f64 = 1.0; // Regular annotation
    let an_integer = 5i32; // Suffix annotation
    let default_float = 3.0; // `f64`
    let default_integer = 7; // `i32`

    // Mutable variables
    let mut inferred_type = 12; // Type i64 is inferred from another line
    inferred_type = 35454545;

    let mut mutable = 12;
    mutable = 21;
    // mutable = true; // Error!

    let pair: (char, i32) = ('a', 38); // Tuple
    let (x, y) = pair; // Destructuring

    println!(" Values of X and Y: {} - {}", x, y);

    let another_pair = ('b', 17);
    println!(
        " Values of another_pair: {} - {}",
        another_pair.0, another_pair.1
    );

    let x = "out";

    {
        let x = "in";
        println!(" Inner X: {}", x);
    }

    println!(" Outer X: {}", x);

    let x = 42;
    let y = { 13 };

    println!(" Values of X and Y: {} - {}", x, y);

    println!("Logical: {}", fair_dice_roll());
    print_number(Number {
        odd: true,
        value: 1,
    });

    let z = 5 + /* 90 + */ 5;
    println!("Iz `z` 10 or 100? z= {}", z);
}

fn fair_dice_roll() -> i32 {
    8
}

fn print_number(n: Number) {
    println!("Is positive: {}", n.is_posititive());
    println!("Is odd: {}", n.is_odd());
    match n.value {
        1 => println!("One"),
        2 => println!("Two"),
        _ => println!("Other: {}", n.value),
    }
}

fn main() {
    literals_and_operators();
    // println!("Hello, world!");
}
