use std::io::{BufRead, Result};
use std::iter;
use std::vec::IntoIter;
fn parse_csv_document<R: BufRead>(src: R) -> Result<Vec<Vec<String>>> {
    src.lines()
        .map(|line| {
            // For each line in the source
            line.map(|line | {
                // If the line was read successfully, process it, if not, return the error
                line.split(',') // Split the line separated by commas
                    .map(|entry| String::from(entry.trim()))// Remove leading and trailing whitespace
                    .collect() // Collect all strings in a row into a Vec<String>
            })
        }).collect() // Collect all lines into a Vec<Vec<String>>
}


// This function combines two `Vec<i32>` and returns an iterator over it.
// Look how complicated its return type is!
fn combine_vecs_explicit_return_type(v: Vec<i32>, u: Vec<i32>) -> iter::Cycle<iter::Chain<IntoIter<i32>, IntoIter<i32>>> {
    v.into_iter().chain(u.into_iter()).cycle()
}


// This is the exact same function, but its return type uses `impl Trait`
// Look how much simpler it is!
fn combine_vecs(v: Vec<i32>, u: Vec<i32>) -> impl Iterator<Item=i32> {
    v.into_iter().chain(u.into_iter()).cycle()
}

fn make_adder_function(y: i32) -> impl Fn(i32) -> i32 {
    move |x| x + y
}

fn double_positives<'a>(numbers: &'a Vec<i32>) -> impl Iterator<Item = i32> + 'a {
    numbers.iter().filter(|&&x| x > 0).map(|&x| x * 2)
}

fn main() {
    let v1 = vec![1,2,3];
    let v2 = vec![4,5];

    let mut v3 = combine_vecs(v1, v2);
    assert_eq!(v3.next(), Some(1));
    assert_eq!(v3.next(), Some(2));

    let plus_one = make_adder_function(1);
    println!("{}", plus_one(2));

    let singles = vec![-3, -2, 2, 3];
    let doubles = double_positives(&singles);
    assert_eq!(doubles.collect::<Vec<i32>>(), vec![ 4, 6]);
}

