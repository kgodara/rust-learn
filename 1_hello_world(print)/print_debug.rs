// Learning printing with fmt::Debug in Rust

// Derive the `fmt::Debug` implementation for `Structure`. `Structure`
// is a structure which contains a single `i32`.
#[derive(Debug)]
struct Structure(i32);

// Put a `Structure` inside of the structure `Deep`. Make it printable
// also.
#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}


fn main() {

    // Printing with `{:?}` is similar to with `{}`.
    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
            "Slater",
            "Christian",
            actor="actor's");

    // `Structure` is printable!
    println!("Now {:?} will print!", Deep(Structure(3)));

    // Pretty printing with fmt::Debug (whitespace formatting)
    let name = "Peter";
    let age = 25;
    let peter = Person { name, age };

    println!("{:#?}", peter);

}