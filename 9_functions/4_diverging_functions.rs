// Diverging functions never return and are marked by `!` which is an empty type.
// `!` cannot be instantiated because it has no possible values,
// unlike `()` which has one possible value.

// This concept is useful and handy, due to the fact that this type
// can be cast to any other one, and thus used in places where an exact type is required such as match branches.

// Functions like loop {} and exit() also use this return type

fn main() {
    fn sum_odd_numbers(up_to: u32) -> u32 {
        let mut acc = 0;
        for i in 0..up_to {
            // Notice that the return type of this match expression must be u32
            // because of the type of the "addition" variable.
            let addition: u32 = match i%2 == 1 {
                // The "i" variable is of type u32, which is perfectly fine.
                true => i,
                // On the other hand, the "continue" expression does not return
                // u32, but it is still fine, because it never returns and therefore
                // does not violate the type requirements of the match expression.
                false => continue,
            };
            acc += addition;
        }
        acc
    }
    println!("Sum of odd numbers up to 9 (excluding): {}", sum_odd_numbers(9));
}