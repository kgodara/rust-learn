// Two different operators allow for conditional checks
//     the 'cfg' attribute: '#[cfg(...)]' in attribute position
//     the 'cfg!' macro: 'cfg!(...)' in boolean expressions
// the former nables conditional compilation, 
// the latter evaluates to 'true' or 'false' literals llowing for checks at run-time.

// This function only gets compiled if the target OS is linux
#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("You are running linux!");
}

// And this function only gets compiled if the target OS is *not* linux
#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("You are *not* running linux!");
}

fn main() {
    are_you_on_linux();

    println!("Are you sure?");
    if cfg!(target_os = "linux") {
        println!("Yes. It's definitely linux!");
    } else {
        println!("Yes. It's definitely *not* linux!");
    }
}
