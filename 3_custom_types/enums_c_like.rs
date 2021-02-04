// An attribute to hide warnings for unused code
#![allow(dead_code)]

// enum with implicit discriminator (starts at 0)
// Note: what is an implicit discriminator? - Implicitly begins at 0
// Subsequent Unit enums are incremented by 1, depending on the previous enum's value
enum Number {
    Zero,
    One = 4,
    Two,
}

// enum with explicit discriminator
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn main() {
    // `enums` can be cast as integers
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);
    println!("two is {}", Number::Two as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);

}