// A unit struct without resources
#[derive(Debug, Clone, Copy)]
struct Unit;

// A tuple struct with resources that implements the `Clone` trait
#[derive(Debug, Clone)]
struct Pair(Box<i32>, Box<i32>);

fn main() {
    let unit = Unit;

    // Copy `Unit`, there are no resources to move
    let copied_unit = unit;

    println!("original: {:?}", unit);
    println!("copied: {:?}", copied_unit);

    let pair = Pair(Box::new(1), Box::new(2));
    let moved_pair = pair;
    // println!("original: {:?}", pair);
    println!("copied pair: {:?}", moved_pair);

    let cloned_pair = moved_pair.clone();
    drop(moved_pair);



}