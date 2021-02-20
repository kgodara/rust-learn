// Note: No restriction on order of function definitions
fn main() {
    // Can use functions defined later
    fizzbuzz_to(100);
}

// Returns a bolean value
fn is_divisble_by(lhs: u32, rhs: u32) -> bool {
    if rhs == 0 {
        return false;
    }

    // This expression will be returned, as it is the last in the function
    lhs % rhs == 0
}

// Functions without an explicit return value, actually return the unit type `()`
fn fizzbuzz(n: u32) -> () {
    if is_divisble_by(n, 15) {
        println!("fizzbuzz");
    } else if is_divisble_by(n, 3) {
        println!("fizz");
    } else if is_divisble_by(n, 5) {
        println!("buzz");
    } else {
        println!("{}", n);
    }
}

// Example of a function without an explicit return type
fn fizzbuzz_to(n: u32) {
    for n in 1..n + 1 {
        fizzbuzz(n);
    }
}