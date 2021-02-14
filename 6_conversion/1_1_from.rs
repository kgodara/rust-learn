// Unlike primitive types which can be converted between each other through casting,
// conversion between custom types (structs, enums) uses Traits

// The From trait allows for a type to define how to create itself from another type
use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number{ value: item }
    }
}

fn main() {
    let num = Number::from(30);
    println!("My number is {:?}", num);
}