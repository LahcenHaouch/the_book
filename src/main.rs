mod temperature;

use temperature::{convert_temperature, TemperatureType};

fn main() {
    let fahrenheit = 50;
    let celsius = convert_temperature(fahrenheit, TemperatureType::FAHRENHEIT);

    println!("f: {fahrenheit} -> c:{celsius}");

    let celsius = 20;
    let fahrenheit = convert_temperature(celsius, TemperatureType::CELSIUS);

    println!("c: {celsius} -> f:{fahrenheit}");
}
