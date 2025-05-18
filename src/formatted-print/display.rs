use std::fmt;

struct Structure(i32);

#[derive(Debug)]
struct MinMax(i64, i64);

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

// Define a structure named `List` that contains a vector of integers
struct List(Vec<i32>);


// Implementing the Display trait for the MinMax struct
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

// Implementing the Display trait for the Point2D struct
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// Implementing the Display trait for the List struct
impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        // Extract the value using tuple destructuring and create a reference to the vector
        let vec = &self.0;
        write!(f, "[")?;

        // Iterate over `v` in `vec` while enumerating the iteration count in `count`
        for (count, v) in vec.iter().enumerate() {
            if count != 0 {  write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }

        write!(f, "]")?;
        Ok(())
    }
}

fn main() {
    let minmax = MinMax(0, 14);
    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {big} and the small range is {small}.", big = big_range, small = small_range);

    let point = Point2D { x: 3.3, y: 7.2 };
    println!("Compare structures:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    let v = List(vec![1, 2, 3]);
    println!("Display: {}", v);

}
