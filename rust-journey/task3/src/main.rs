fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn celsius_to_kelvin(celsius: f64) -> f64 {
    celsius + 273.00
}

fn kelvin_to_celsius(kelvin: f64) -> f64 {
    kelvin - 273.00
}

fn kelvin_to_fahrenheit(kelvin: f64) -> f64 {
    celsius_to_fahrenheit(kelvin_to_celsius(kelvin))
}

fn fahrenheit_to_kelvin(fahrenheit: f64) -> f64 {
    celsius_to_kelvin(fahrenheit_to_celsius(fahrenheit))
}

fn main() {
    let celsius = 25.0;
    let fahrenheit = celsius_to_fahrenheit(celsius);
    let kelvin = celsius_to_kelvin(celsius);
    println!("Celsius: {}", celsius);
    println!("Fahrenheit: {}", fahrenheit);
    println!("Kelvin: {}", kelvin);
    let celsius_from_fahrenheit = fahrenheit_to_celsius(fahrenheit);
    let kelvin_from_fahrenheit = fahrenheit_to_kelvin(fahrenheit);
    println!("Celsius from Fahrenheit: {}", celsius_from_fahrenheit);
    println!("Kelvin from Fahrenheit: {}", kelvin_from_fahrenheit);    
}