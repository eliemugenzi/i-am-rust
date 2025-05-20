#![allow(overflowing_literals)]

fn main() {
    let decimal = 65.4321_f32;
    // Error! No implicit conversion
    // let integer: u8 = decimal;

    // Explicit conversion
    // The `as` operator is used to convert between types
    let integer = decimal as u8;
    let character = integer as char;
    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    // When casting any value to an unsigned type, T
    // T::MAX + 1 is added or subtracted until the value fits into the new type
    println!("1000 as a u16 is: {}", 1000 as u16);
    println!("1000 as a u8 is: {}", 1000 as u8);

    // -1 + 256 = 255
    println!("-1 as a u8 is: {}", (-1i8) as u8);

    // For positive numbers, this is the same as the value modulo 256
    println!("1000 mod 256 is: {}", 1000 % 256);

    /**
     * When casting to a signed type, the bitwise result is the same as
     * first casting to the corresponding unsigned type.
     * If the most significant bit of that value is 1, the value is negative.
     * Unless it already fits, of course.
    */
    println!("128 as i16 is: {}", 128 as i16);

    // In boundary case 128 value in 8-bit two's complement representation is -128
    println!("128 as i8 is: {}", 128 as i8);

    // Repeating the example above
    // 1000 as u8 is 1000 mod 256 = 232
    println!("1000 as u8 is: {}", 1000 as u8);

    // and the value of 232 in 8-bit two's complement representation is -24
    println!("232 as i8 is: {}", 232 as i8);

    /**
     * Since Rust 1.45, the `as` keyword performs a `saturating cast`
     * When casting from float to int. If the floating point value is upper bound or is less than the lower bound,
     * the returned value will be equal to the bound crossed.
     */

    // 300.0  aa u8 is 255
    println!("300.0 as u8 is: {}", 300.0_f32 as u8);
    // -100.0 as u8 is 0
    println!("-100.0 as u8 is: {}", -100.0_f32 as u8);

    // NaN as u8 is 0
    println!("NaN as u8 is: {}", f32::NAN as u8);

    /**
     * This behaviour incurs a small runtime cost and can be avoided
     * with unsafe methods, however the results might overflow and
     * return *unsound values*. Use these methods wisely.
     */

    unsafe {
         // 300.0 as u8 is 44
        println!(" 300.0 as u8 is : {}", 300.0_f32.to_int_unchecked::<u8>());
        // -100.0 as u8 is 156
        println!("-100.0 as u8 is : {}", (-100.0_f32).to_int_unchecked::<u8>());
        // nan as u8 is 0
        println!("   nan as u8 is : {}", f32::NAN.to_int_unchecked::<u8>());
    }
   
}