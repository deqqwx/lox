#[test]
fn main() {
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12); // x inside this block is 12
    }

    assert_eq!(x, 5); // Outside the block, x is still 5

    let x = 42;
    println!("{}", x); // Prints "42".
}
