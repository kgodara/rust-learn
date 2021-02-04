use std::fmt;

// Three types of structs an be made using the `struct` keyword.
// 1. Tuple structs, essentially named tuples
// 2. Classic C structs, which have named fields
// 3. Unit structs which don't have fields

// Tuple struct
struct Pair(i32, f32);

// Unit struct
struct Nil;

// C struct
#[derive(Debug)]
struct Person<'a> {
    // The 'a defines a lifetime
    name: &'a str,
    age: u8,
}

// A struct with two fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// Structs can be used in the fields of another struct
#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(rect: Rectangle) -> f32 {
    let height;
    let width;

    let Point { x: left_edge, y: top_edge } = rect.top_left;
    let Point { x:  right_edge, y: bottom_edge} = rect.bottom_right;

    height = top_edge - bottom_edge;
    width = right_edge - left_edge;

    return height * width;
}

fn square(point: &Point, edge_len: f32) -> Rectangle {

    let left_edge;
    let top_edge;

    let right_edge;
    let bottom_edge;

    left_edge = point.x;
    top_edge = point.y + edge_len;

    right_edge = point.x + edge_len;
    bottom_edge = point.y;

    return Rectangle { top_left: Point { x: left_edge, y: top_edge }, bottom_right: Point { x: right_edge, y: bottom_edge } };

}



fn main() {

    // Create struct with field init shorthand
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);

    // Instantiate a `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };

    // Access the fields of the Point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 5.2, ..point };

    // `bottom_right.y` will now be the same as `point.y` because we used that field from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using the `let` binding
    // NOTE: This is pretty cool, we mock the struct to extract from into new vars
    let Point { x: top_edge, y:left_edge } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point { x: left_edge, y: top_edge},
        bottom_right: bottom_right,
    };

    // Instantiate a unit struct
    let _nil = Nil;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access fields of tuple struct
    println!("pair contains {} and {}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    println!("Rect: {:?}", _rectangle);
    println!("Rect Area: {}", rect_area(_rectangle));

    let square_point = Point { x: 0.0, y: 0.0 };
    let edge_len = 5f32;

    println!("\nCreating Square with bottom-left {}, edge_len: {}\nSquare: {:?}", square_point, edge_len, square(&square_point, edge_len));


}