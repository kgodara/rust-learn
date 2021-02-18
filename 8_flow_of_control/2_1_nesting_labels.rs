

// By annotating loops with labels using 'label,
// it becomes possible to break and continue outer loops when working within nested loops


// The loops must be annotated with some 'label,
// which must be passed to the break/continue statement.

#![allow(unreachable_code)]
#![allow(unused_labels)]

fn main() {
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            // break outer loop
            break 'outer;

        }

        println!("This point will never be reached");
    }

    println!("Exited the outer loop");


}