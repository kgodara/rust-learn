// Compile with: rustc 2_using_library.rs --extern testlib=lib1_create_library.rlib --edition=2018 && ./2_using_library 
// extern crate lib1_create_library;

fn main() {
    testlib::public_function();

    // Error! `private_function` is private
    //testlib::private_function();

    testlib::indirect_access();
}