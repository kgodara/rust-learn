

// if/else branching is similar to other languages
// Parentheses not needed for boolean condition
// if/else branches must return the same type

fn main() {

    let n = 5;
    
    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n)
    }

    let big_n = 
        if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold");
            
            // This branch will return an `i32`
            10 * n
        } else {
            println!(", and is a big number, halve the number");

            // This branch needs to return an `i32` in order to match the `if` branch
            n / 2
        };
        // ^ All let bindings need a semicolon

        println!("{} -> {}", n, big_n);
    
}