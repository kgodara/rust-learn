
// Variable bindings are immutable by default, overriden with `mut` modifier

fn main() {

    // Underscore removes compiler warning for unused variable
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);

    // Ok
    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);

    // Error!
    // _immutable_binding += 1;

}