#[test]
fn main() {
    // Використовуємо checked_add для u8
    let v1 = 251_u8.checked_add(8).unwrap_or_else(|| {
        println!("Відбулося переповнення у v1!");
        0 // Повертаємо значення за замовчуванням, якщо сталося переповнення
    });

    // Використовуємо checked_add для u8 і обробляємо переповнення
    let v2 = 251_u8.checked_add(8);

    match v2 {
        Some(value) => println!("{}, {}", v1, value),
        None => println!("{}, відбулося переповнення!", v1),
    }
}
