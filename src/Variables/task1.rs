#[test]
fn main() {
    let x: i32 = 5; // Initialize x
    let y: i32; // Still uninitialized but unused, so just a warning

    assert_eq!(x, 5);
    println!("Success!");
}
