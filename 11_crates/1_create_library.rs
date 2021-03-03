// Compile as library with: rustc --crate-type=lib 1_hello_world

// Libraries get prefixed with "lib", and by default they get named after their crate file, 
// Can be overriden with `--crate-name` on CLI or `crate_name` attribute

pub fn public_function() {
    println!("called 1_create_library's `public_function()`");
}

fn private_function() {
    println!("called 1_create_library's `private_function()`");
}

pub fn indirect_access() {
    print!("called 1_create_library's `indirect_access()`, that\n> ");

    private_function();
}