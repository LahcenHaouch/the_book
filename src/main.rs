mod temperature;

use temperature::{convert_to_celsius, convert_to_fahrenheit};

fn main() {
    let fahrenheit = 50;
    let celsius = convert_to_celsius(fahrenheit);

    println!("f: {fahrenheit} -> c:{celsius}");

    let celsius = 20;
    let fahrenheit = convert_to_fahrenheit(celsius);

    println!("c: {celsius} -> f:{fahrenheit}");
}
