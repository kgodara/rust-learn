
// Loops can return values through break statements
// Below: `counter * 2` will be returned by the loop expression

fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);
}