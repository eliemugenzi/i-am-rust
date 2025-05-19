
use std::fmt::Display;

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // `let` can be used to bind members of a tuple to variables
    let (int_param, bool_param) = pair;
    return (bool_param, int_param);
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl Matrix {
    fn transpose(&self) -> Matrix {
        Matrix(self.0, self.2, self.1, self.3)
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})\n({}, {})", self.0, self.1, self.2, self.3)
    }
}

fn main() {
    // A tuple with a bunch different types
    let long_tuple = (1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64,'a',  true, false);

    // Values can be extracted from tuple using tuple indexing
    println!("long_tuple.0 = {}", long_tuple.0);
    println!("long_tuple.1 = {}", long_tuple.1);

    let touple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);
    println!("touple_of_tuples = {:?}", touple_of_tuples);

    let pair = (1, true);
    println!("pair = {:?}", pair);
    println!("reverse(pair) = {:?}", reverse(pair));

    // To create one element tuples, the comma is required to thell them apart from a literal surrounded by parentheses
    let one_element_tuple = (5u32,);
    println!("one_element_tuple = {:?}", one_element_tuple);
    println!("Just an integer = {:?}", (5u32));

    let tuple = (1, "hello", 4.5, true);
    let (a, b, c, d) = tuple;
    println!("a = {:?}, b = {:?}, c = {:?}, d = {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    let transposed_matrix = matrix.transpose();
    println!("matrix =\n{}", matrix);
    println!("transposed_matrix =\n{}", transposed_matrix);
}