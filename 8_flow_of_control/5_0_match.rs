
// Rust provides pattern matching via `match`
// which can be used like a C `switch`
// first matching arm is evaluated
// all possible values must be covered (exhaustive)

fn main() {
    let number = 13;

    println!("Tell me about {}", number);
    match number {
        // Match a single value
        1 => println!("One!"),

        // Match several values
        2 | 3 | 5 | 7 | 11 | 13 => println!("This is a prime"),

        // Match an inclusive range
        14..=19 => println!("A teen"),

        // Handle the rest of cases
        _ => println!("Ain't special"),
        // ^ all possible cases must be covered
    }

    let boolean = true;
    // Match is an expression too
    let binary = match boolean {
        false => 0,
        true => 1,
    };

    println!("{} -> {}", boolean, binary);
}
