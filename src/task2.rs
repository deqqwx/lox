#[test]
fn main() {
    let x: i32 = 5; // Initialize x
    let _y: i32; // Still unused, but no error

    assert_eq!(x, 5);
    println!("Success!");
}
