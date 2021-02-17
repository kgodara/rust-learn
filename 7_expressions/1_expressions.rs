#![allow(unused_must_use)]
#![allow(path_statements)]


// Rust programs are largely composed of series of statements, the most common being expressions
fn main() {

    // The most common statements are declaring a variable binding and using `;` with an expression
    
    // Variable binding
    let x = 5;

    // expression;
    x;
    x + 1;
    15;

    // Blocks are also expressions, and thus can be used as values in assignments
    // The last expression in the block will be assigned to the place expression, e.g. a local variable
    // If the last expression ends with a semicolon, the return value will be ()

    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cubed = x_squared * x;

        // This expression will be assigned to 'y'
        x_cubed + x_squared + x
    };

    let z = {
        // `;` means `()` is assigned to z
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);

}