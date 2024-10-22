fn main() {
    println!("Hello, world!");

    let fahrenheit = convert_celsius_to_fahrenheit(75);
    let celsius = convert_fahrenheit_to_celsius(fahrenheit);

    println!("{celsius}")
}

fn convert_fahrenheit_to_celsius(fahrenheit: i32) -> i32 {
    let c = fahrenheit - 32 * (5 / 9);

    return c;
}

fn convert_celsius_to_fahrenheit(celsius: i32) -> i32 {
    let f = celsius * (9 / 5) + 32;

    return f;
}
