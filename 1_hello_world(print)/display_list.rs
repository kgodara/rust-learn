use std::fmt;

struct List(Vec<i32>);

impl fmt::Display for List {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;

        // Try `write!` to see if it errors. If it errors, return
        // the error. Otherwise continue.
        write!(f, "[")?;

        for (count, v) in vec.iter().enumerate() {

            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}: {}", count, v)?;
        }

        // return a fmt::Result value.
        write!(f, "]")

    }

}

fn main() {

    let v = List(vec![1, 2, 3]);
    println!("{}", v);

}