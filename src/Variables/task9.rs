#[test]
fn main() {
    let (x, y); // Declare variables x and y
    (x, ..) = (3, 4); // Destructure to assign x
    [.., y] = [1, 2]; // Destructure to assign y

    // Fill the blank to make the code work
    assert_eq!([x, y], [3, 2]); // Fill in with [3, 2]

    println!("Success!");
}


