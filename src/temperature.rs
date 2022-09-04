pub enum TemperatureType {
    FAHRENHEIT,
    CELSIUS,
}

pub fn convert_temperature(temperature: i32, temperature_type: TemperatureType) -> i32 {
    match temperature_type {
        TemperatureType::CELSIUS => (temperature * 9 / 5) + 32,
        TemperatureType::FAHRENHEIT => (temperature - 32) * 5 / 9,
    }
}
