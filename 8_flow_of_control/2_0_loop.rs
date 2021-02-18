
// The `loop` keyword indicates an infinite loop
// The `break` statement can be used to exit a loop
// The `continue` statement skips the rest of the current iteration to begin a new iteration of the current loop.


fn main() {
    let mut count = 0u32;

    println!("Let's count until infinity!");

    // Infinite loop
    loop {
        count += 1;

        if count == 3 {
            println!("three");

            // Skip the rest of this iteration
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");

            // Exit this loop
            break;
        }
    }

}