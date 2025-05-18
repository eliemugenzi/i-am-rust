#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

fn main() {
    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the actor {actor:?} in {movie:?}.", "Slater", "Christian", actor = "Keanu Reeves", movie = "John Wick");
    println!("Now {:?} will print!", Structure(3));
    println!("Now {:?} will print!", Deep(Structure(7)));

    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };
    println!("{:#?}", peter);
}