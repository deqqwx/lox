#[test]
fn main() {
    let x = define_x(); // Call define_x to get the value of x
    println!("{}, world", x);
}

fn define_x() -> &'static str {
    let x = "hello"; // x is a string literal with static lifetime
    x // Return x
}
