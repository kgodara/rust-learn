// Suppress warnings from overflowing casts
#![allow(overflowing_literals)]

fn main() {

    let decimal = 65.4321_f32;

    // Error! Rust does not provide implicit conversion
    // let integer: u8 = decimal;

    // Explicit conversion
    let integer = decimal as u8;
    let character = integer as char;

    // Error! Cannot directly convert float to char, a way to do this would be to float -> int -> char
    // Note: attempting to rebind character
    // let character = decimal as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    // When casting into an unsigned type, T,
    // T::MAX + 1 is added or subtracted until the value fits into the new type

    // 1000 already fits into u16, so no increment/decrement
    println!("1000 as a u16 is: {}", 1000 as u16);

    // 1000 - 256 - 256 - 256 = 232
    // the first 8 least signficant bits are kept, the rest towards the most significant bit are truncated
    // 0000001111101000 (1000) -> 11101000 (232)
    println!("1000 as a u8 is : {}", 1000 as u8);
    // -1 + 256 = 255
    println!("-1 as a u8 is : {}", (-1i8) as u8);

    // For positive numbers, this is the same as the modulus
    // if the modulus is T::MAX
    println!("1000 mod 256 is : {}", 1000 % 256);

    // When casting to a signed type, the (bitwise) result is the same as
    // first casting to the corresponding unsigned type. If the most significant
    // bit of that value is 1, then the value is negative.

    // Unless it already fits, of course.
    println!("128 as a i16 is: {}", 128 as i16);
    // 128 as u8 -> 128, whose two's complement in eight bits is:
    // Explanation: 10000000, u8 (128) -> 10000000, i8 (-128)
    println!("128 as a i8 is : {}", 128 as i8);

    // repeating the example above
    // 1000 as u8 -> 232
    println!("1000 as a u8 is : {}", 1000 as u8);
    // and the two's complement of 232 is -24    
    println!(" 232 as a i8 is : {}", 232 as i8);

    // the `as` keyword performs a *saturating cast* when casting from float to int.  
    // If the floating point value exceeds the upper bound or is less than the lower bound, the returned value will be equal to the bound crossed.

    // 300.0 is 255
    println!("300.0 is {}", 300.0_f32 as u8);
    // -100.0 as u8 is 0
    println!("-100.0 as u8 is {}", -100.0_f32 as u8);
    // nan as u8 is 0 - interesting
    println!("nan as u8 is {}", f32::NAN as u8);

    // saturating casts incur a small runtime cost and can be avoided with unsafe methods, however the results might overflow and return **unsound values**. Use these methods wisely:
    unsafe {
        // 300.0 is 44 - saturating cast: 255
        println!("300.0 is {}", 300.0_f32.to_int_unchecked::<u8>());
        // -100.0 as u8 is 156, saturating cast: 0
        println!("-100.0 as u8 is {}", (-100.0_f32).to_int_unchecked::<u8>());
        // nan as u8 is 0, saturating cast: 0
        println!("nan as u8 is {}", f32::NAN.to_int_unchecked::<u8>());
    }


    


    

}