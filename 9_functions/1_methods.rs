// Methods are functions attached to objects
// Methods have access to the data & other methods of an object via the `self` keyword.
// Methods must be defined under an `impl` block.

struct Point {
    x: f64,
    y: f64,
}

// `Point` methods defined in this Implementation block.
impl Point {
    // This is a static method,
    // which doesn't need to be called by an instance
    // Often used as constructors
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Another static method
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // This is an instance method.
    // `&self` is sugar for `self: &Self` where `Self` is the type of the caller object,
    // in this case `Rectangle`
    fn area(&self) -> f64 {
        // `self` gives access to struct fields via the dot operator
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs` is a `f64` method, returns absolute value of caller
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        // Destructuring `Point` struct
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // if the caller object needs to be mutable can use:
    // `&mut self` which desugars to `self: &mut Self`
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}



// `Box` indicates that `Pair` owns two heap allocated integers
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // This "consumes" the caller object by taking ownership
    // `self` desugars to `self: Self`
    fn destroy(self) {
        // Destructure `self`
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // `first` and `second` go out of scope and are dropped
    }
}

fn main() {

    let rectangle = Rectangle {
        // Static methods are called using double colons
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // Instance methods are called using the dot operator
    // the first argument `&self` is implicitly passed
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // rectangle.translate(1.0, 0.0);
    // ^^ Will cause an Error, since `rectangle` is immutable,
    // and `translate` requires a mutable object

    // Square is mutable, can call mutable methods
    square.translate(1.0, 0.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // pair.destroy()
    // ^^ Error! pair has already been consumed/dropped

}