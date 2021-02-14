// To convert a type to a string the ToString trait can be implemented
// However, it's preferable to use fmt::Display which will automagically provide ToString, and allow printing the type


use std::fmt;

struct Circle {
    radius: i32
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

fn main() {
    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());


    // Parsing strings into numbers:

    // These will succeed so long as the FromStr type is implemented for the types the Strings are being parsed into

    // type inference
    let parsed: i32 = "5".parse().unwrap();

    // 'turbofish' syntax
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);

}
