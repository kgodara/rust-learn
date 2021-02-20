// Functions can also be passed as arguments to functions
// that take closures as parameters, as long as the trait bound 
// of the parameter is satisfied by the provided function

// generic argument `F` bounded by `Fn`
fn call_me<F: Fn()>(f: F) {
    f();
}

// function that satisfies `Fn` bound
fn function() {
    println!("I'm a function!");
}

fn main() {
    // Define a closure satisfying the `Fn` bound
    let closure = || println!("I'm a closure!");

    call_me(closure);
    call_me(function);

}