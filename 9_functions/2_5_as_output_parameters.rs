
// Closures can also be returned as output parameters
// However, since anonymous closure types are unknown
// `impl Trait` must be used to return them

// The valid traits for returning a closure are: Fn, FnMut, FnOnce

// The `move` keyword must also be used to signal that all captures by value
// This is required because any capture by reference would be dropped when the function ended

fn create_fn() -> impl Fn() {
    let text = "Fn".to_owned();

    move || println!("This is a: {}", text)
}

fn create_fnmut() -> impl FnMut() {
    let text = "FnMut".to_owned();

    move || println!("This is a: {}", text)
}

fn create_fnonce() -> impl FnOnce() {
    let text = "FnOnce".to_owned();

    move || println!("This is a: {}", text)
}

fn main() {

    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();
    let fn_once = create_fnonce();

    fn_plain();
    fn_mut();
    fn_once();

}