 // Nanosecond, Inch, and U64 are all type aliases for u64
type NanoSecond = u64;
type Inch = u64;
type U64  = u64;

fn main() {
    let nanoseconds: NanoSecond = 5 as u64;
    let inches: Inch = 2 as u64;

    // Note that type aliases don't provide any extra type safety,
    // because aliases are not new types
    println!("{} nanoseconds + {} inches = {} unit?", 
        nanoseconds, inches, nanoseconds + inches);
    // This is a valid operation because both are u64
}