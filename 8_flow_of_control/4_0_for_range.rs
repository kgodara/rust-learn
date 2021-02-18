
// `for in` can be used to iterate through an Iterator
// an easy way to create an iterator is using range notation, e.g. `a..b`
// a (inclusive) -> b (exclusive)

fn main() {

    // Fizzbuzz with a range operator
    // Note: interesting that 'n' doesn't have to be declared
    // like before when using while loop

    // Either variation of the iterator can be used
    for n in 1..=100 /*1..101*/ {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        }
        else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }


}