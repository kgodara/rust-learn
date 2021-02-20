// Because closures capture variables from enclosing scopes,
// generics are required for closures when they are used as function parameters

//When a closure is defined, the compiler implicitly
// creates a new anonymous struct to store the captured variables inside,
// meanwhile implementing the functionality via one of the traits: Fn, FnMut, or FnOnce for this unknown type.
// This type is assigned to the variable which is stored until calling.

// Since this new type is of unknown type, any usage in a function will require generics.
// However, an unbounded type parameter <T> would still be ambiguous and not be allowed.
// Thus, bounding by one of the traits: Fn, FnMut, or FnOnce (which it implements) is sufficient to specify its type.

// `<F>` signifies a "generic type parameter"
// the where clause bounds it by a trait: Fn, FnMut, or FnOnce
// the where clause is required since an unbounded type parameter `<T>` would be ambiguous and not allowed
// `F` implements `Fn` the most restrictive trait,
// since the `print` closure has no inputs and returns nothing
fn apply<F>(f: F) where
    F: Fn() {
    f();
}

fn main() {
    let x = 7;

    // Capture `x` into an anonymous type and implement
    // `Fn` for it. Store it in `print`
    let print = || println!("{}", x);

    apply(print);
}