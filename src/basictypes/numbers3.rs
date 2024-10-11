#[test]
fn main() {
    let x = 5;
    assert_eq!("i32".to_string(), type_of(&x)); // Замість "u32", має бути "i32"

    println!("Success!");
}

// Отримуємо тип змінної та повертаємо його як рядок
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
