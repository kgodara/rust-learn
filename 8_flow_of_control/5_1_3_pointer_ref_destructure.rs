

// Destructuring vs dereferencing:
// Dereferencing uses `*`
// Destructuring uses `&`, `ref`, `ref mut`

fn main() {
    // Assign a reference of type `i32`
    // `&` signifies a reference being assigned
    let reference = &4;

    match reference {
        // If `reference` is pattern matched against `&val`
        // comparison:
        // `&i32`
        // `&val`
        // So, with matching `&`s dropped, i32 will be assigned to `val`.
        &val => println!("Got a value via destructuring: {:?}", val),
    }

    // Can avoid needing `&` by dereferencing before matching
    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }

    // This is not a reference
    let _not_a_reference = 3;

    // Rust provides `ref` to modify the assignment and create a reference for the element;
    // the reference is assigned
    let ref _is_a_reference = 3;

    // for the 2 values defined below,
    // references can respectively retrieved via:
    // `ref`
    // `ref mut`
    let value = 5;
    let mut mut_value = 6;

    // `ref` keyword creates a reference
    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    // `ref mut` can be used similarly
    match mut_value {
        ref mut m => {
            // need to dereference our reference in oreder to modify
            *m += 10;
            println!("We added 10. `mut_value`: {:?}", m);
        }
    }
}