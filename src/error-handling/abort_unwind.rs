fn drink(beverage: &str)  {
    if beverage == "lemonade" {
        if cfg!(panic == "abort") {
            panic!("lemonade is not available");
        }
        else {
            println!("Spit it out!");
        }
    } else {
        println!("Drinking {}...", beverage);
    }
}

#[cfg(panic = "unwind")]
fn ah() {
    println!("Spit it out!!");
}

#[cfg(not(panic = "unwind"))]
fn ah() {
    println!("This is not your party. Run!!!");
}
fn main() {
    drink("coffee");
}