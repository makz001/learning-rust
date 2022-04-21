use std::io;

fn main() {
    let mut fahrenheit = String::new();

    println!("Fahrenheits: ");

    io::stdin()
        .read_line(&mut fahrenheit)
        .expect("Failed to read line");

    let fahrenheit: f64 = fahrenheit.trim().parse()
        .expect("Type a valid value");

    // (fahrenheit − 32) × 5/9 = celsius

    let celsius = (fahrenheit - 32f64) * 0.55555555555;

    println!("Celsius: {}", celsius);
}
