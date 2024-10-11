#[test]
fn main() {
    // Integer addition
    assert!(1u32 + 2 == 3); // 3

    // Integer subtraction
    assert!(1i32 - 2 == -1); // -1
    assert!(1u8.wrapping_sub(2) == 255); // 255

    assert!(3 * 50 == 150); // 150

    assert!(9.6 / 3.2 == 3.0); // correct

    assert!(24 % 5 == 4); // 4
    // Short-circuiting boolean logic
    assert!(true && false == false); // false
    assert!(true || false == true); // true
    assert!(!true == false); // false

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101); // 0001
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101); // 0011
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101); // 0010
    println!("1 << 5 is {}", 1u32 << 5); // 32
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2); // 0x20
}
