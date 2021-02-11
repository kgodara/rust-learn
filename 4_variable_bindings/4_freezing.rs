
fn main() {
    let mut _mutable_integer = 7i32;

    {
        // Shadowing by immutable `_mutable_integer`
        let _mutable_integer = _mutable_integer;

        // compile-time Error, _mutable_integer frozen in this scope
        // _mutable_integer = 50;

        // `_mutable_integer` goes out of scope
    }

    // `_mutable_integer` no longer frozen
    _mutable_integer = 3;
}
