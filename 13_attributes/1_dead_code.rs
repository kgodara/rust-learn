// An attribute is metadata applied to some module, crate, or item.
// Metadata can/is used for:
//     conditional compilation of code
//     set crate version, name and type (binary or library)
//     disable lints (warnings)
//     enable compiler features (macros, glob imports, etc)
//     link to a foreign library
//     mark functions as unit tests
//     mark functions that will be part of a benchmark

// Attributes that apply do whole crates: #![crate_attribute]
// Attributes that apply to a module or an item: #[item_attribute]

// Attribute argument syntax examples:
//     #[attribute = "value"]
//     #[attribute(key = "value")]
//     #[attribute(value)]

// Attributes can also have multiple values and be separated over multiple lines
//     #[attribute(value, value2)]
//     #[attribute(value, value2, value3,
//                 value4, value5)]


// an attribute can be used to disable the 'dead_code' lint provided by the compiler
// normally this lint would warn about unused functions
fn used_function() {}

#[allow(dead_code)]
fn unused_function() {}

fn noise_unused_function() {}
// ^ Note: Can add an attribute to suppress warning this function will generate

fn main() {
    used_function();
}
