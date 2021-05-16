// 'crate_type' attribute: tells compiler whether the crate is a binary or one of multiple possible types of libraries
// 'crate_name' attribute: sets the name of the crate

// However, since 'crate_type' and 'crate_name' attributes have no effect when using Cargo
// their real-world uses are relatively limited.

// This crate is a library
#![crate_type = "lib"]
// The library is named "rary"
#![crate_name = "rary"]

pub fn public_function() {
    println!("called rary's `public_function()`");
}

fn private_function() {
    println!("called rary's `private_function()`");
}

pub fn indirect_access() {
    print!("called rary's `indirect_access()`, that\n> ");

    private_function();
}

