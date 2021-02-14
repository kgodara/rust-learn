// The Into trait is the reciprocal of the From trait, meaning
// that if the From trait has been implemented for the relevant type, it will be called
// Into will typically require the type to convert to to be explicitly specified

use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    let int = 5;
    // Try removing the type declaration
    let num: Number = int.into();
    println!("My number is {:?}", num);
}