// Closures are flexible and can adapt to
// do what makes the closure work without annotation

// This allows for closures to adapt to capturing variables:
// by reference `&T`, mutable reference `&mut T`, value `T`

// They preferentially capture by reference and go lower when required

fn main() {
    use std::mem;

    let color = String::from("green");

    // This closure immediately borrows (`&`) `color` and
    // stores the borrow and closure in the `print` variable. It will remain
    // borrowed until `print` is used the last time.

    // Since `println!` only requires an immutable reference,
    // borrowing does not need to be mutable
    let print = || println!("`color`: {}", color);

    // Call the closure using the borrow.
    print();

    // `color` can be immutably again,
    // because the closure only has an immutable reference
    let _reborrow = &color;
    print();

    // A move or reborrow is allowed after the final use of `print`
    let _color_moved = color;

    let mut count = 0;
    // A closure to increment `count` could take either `&mut count` or `count`
    // but `&mut count` is less restrictive so it takes that. Immediately
    // borrows `count`.
    //
    // A `mut` is required on `inc` because a `&mut` is stored inside. Thus,
    // calling the closure mutates the closure which requires a `mut`.
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    }

    // Call using a mutable borrow
    inc();

    // The closure is still mutably borrowing `count`,
    // because the last call of `inc()` has not occurred,
    // so `count` cannot be reborrowed
    // let _reborrow = &count;
    // ^^ Error!
    inc();

    // The closure no longer needs to borrow `&mut count`
    // Therefore it is possible to reborrow without an error.
    let _count_reborrowed = &mut count;

    // A non-copy type - meaning it doesn't implement the Copy trait,
    // and cannot be duplicated simply by copying bits
    // Copy also happens implicitly
    let movable = Box::new(3);

    // `mem::drop` requires `T` so this must take by value. A copy type
    // would copy into the closure leaving the original untouched.
    // A non-copy must move and so `movable` immediately moves into
    // the closure.
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };

    // `consume` consumes the variable so this can only be called once.
    consume();
    // consume();
    // ^^ Error! Can't call again



}