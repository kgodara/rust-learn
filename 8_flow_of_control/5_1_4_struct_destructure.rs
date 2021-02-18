fn main() {
    struct Foo {
        x: (u32, u32),
        y: u32,
    }

    let foo = Foo { x: (1, 2), y: 2 };

    match foo {
        Foo { x: (2, b), y } => println!("First of x is 1, b = {},  y = {} ", b, y),

        // ^ Note: tuple is being destructured from within struct as well

        // you can destructure structs and rename the variables,
        // the order is not important
        Foo { y: 2, x: i } => println!("y is 2, i = {:?}", i),
        // ^^ don't have to destructure tuple field

        // and you can also ignore some variables:
        Foo { y, .. } => println!("y = {}, we don't care about x", y),
        
        //Foo { y } => println!("y = {}", y),
        // ^ Note: this will give an error: pattern does not mention field `x`
        
    }
}