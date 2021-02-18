
// The 'for in' constructor by default applies the into_iter function to the collection
// However, other methods exist to convert collections into iterators.

// into_iter, iter and iter_mut provide different ways to convert collections into iterators

// Note: the underscore in match arms functions as a
// wildcard pattern which matches anything but does not bind to the value



fn main() {

    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            // Since .iter() was used,
            // must be &"TEXT", matching against a borrowed reference
            _ => println!("Hello {}", name),
        }
    }

    println!("names: {:?}", names);

    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            // .into_iter() consumes the collection, so the exact data is being matched against
            _ => println!("Hello {}", name),
        }
    }

    // Can't print the names, consumed by the loop
    // println!("names: {:?}", names);

    let mut names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rutacean among us!",
            _ => "Hello",
        }
    }

    println!("names: {:?}", names);
    
}