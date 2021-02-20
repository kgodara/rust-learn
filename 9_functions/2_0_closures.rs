
// Closures are functions that can capture the enclosing environment.
// Closures are very convenient for on the fly usage
// and can be called exactly like functions
// both input and return types can be inferred and input variable numes must be specified.

// Other Closure characteristics:
// use `||` instead of () around input variables
// optional body delimination `{}` for a single expression
// can capture outer environment variables

fn main() {
    // Increment via closures and functions
    fn function (i: i32) -> i32 { i + 1 }

    // Closure are anonymous, binding to references here
    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred = |i| i + 1;
    
    let i = 1;
    // Call function and closures
    println!("function: {}", function(i));
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", closure_inferred(i));

    // A closure which takes no args
    // returns an inferred `i32`
    let one = || 1;
    println!("closure returning one: {}", one());

}