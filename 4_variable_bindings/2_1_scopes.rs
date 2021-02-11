

fn main() {
    
    let long_lived_binding = 1;

    // This is a block, whose scope will be a subset of the main function's
    {
        // This binding will only exist within its parent block
        let short_lived_binding = 2;

        println!("inner short: {}", short_lived_binding);
    }
    // End of the block

    // Creates compile-time Error, `short_lived_binding` doesn't exist in this scope
    // println!("outer block, short_lived_binding: {}", short_lived_binding);

    println!("outer bloc, long_lived_binding: {}", long_lived_binding);
    


}