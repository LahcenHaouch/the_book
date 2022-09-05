pub fn convert_to_celsius(temperature: i32) -> i32 {
    (temperature - 32) * 5 / 9
}

pub fn convert_to_fahrenheit(temperature: i32) -> i32 {
    (temperature * 9 / 5) + 32
}
