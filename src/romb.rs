#[test]
fn main() {
    let height = 5;

    for i in 0..height {
        let spaces = " ".repeat(height - i - 1);
        let stars = "*".repeat(2 * i + 1);
        println!("{}{}", spaces, stars);
    }

    for i in (0..height-1).rev() {
        let spaces = " ".repeat(height - i - 1);
        let stars = "*".repeat(2 * i + 1);
        println!("{}{}", spaces, stars);
    }
}
