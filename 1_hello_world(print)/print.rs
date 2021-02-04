// Examples of basic fmt::Display printing in Rust

fn main() {

    println!("{} days", 31);

    // Without a suffix, 31 becomes an i32. You can change what type 31 is
    // by providing a suffix.

    println!("{0} this is {1}. {1} this is {0}.", "Alice", "Bob");

    // Named arguments
    println!("{subject} {verb} {object}",
                object="the lazy dog.",
                subject="The quick brown fox",
                verb="jumps over"
            );
    
    // Special formatting can be specified after a `:`.
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);
    
    // You can right-align text with a specified width. This will output
    // "     1". 5 white spaces and a "1".
    println!("{number:>width$}", number=1, width=6);

    // You can pad numbers with extra zeroes. This will output "000001".
    println!("{number:>0width$}", number=1, width=6);

    println!("My name is {1}, {0} {1}", "James", "Bond");

    #[allow(dead_code)]
    struct Structure(i32);
    
    // Custom types, such as structs require more complicated handling than this
    // println!("This struct `{}` won't print...", Structure(3));

    let pi = 3.141592;

    println!("pi ~= {pi:.prec$}", pi=pi, prec=4);

}