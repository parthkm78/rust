fn main() {
    // Declare an integer variable 'temp' to store the temperature in Celsius
    let temp_celsius: i32 = 20;

    // Convert the temperature from Celsius to Fahrenheit
    let temp_fahrenheit: f64 = celsius_to_fahrenheit(temp_celsius);

    // Print the result
    println!("Temperature in Celsius: {:.2}°C", temp_celsius);
    println!("Temperature in Fahrenheit: {:.2}°F", temp_fahrenheit);
}

// Define a function to convert temperature from Celsius to Fahrenheit
fn celsius_to_fahrenheit(celsius: i32) -> f64 {
    // Calculate the temperature in Fahrenheit using the formula: F = (C * 9/5) + 32
    let fahrenheit = (celsius as f64 * 9.0 / 5.0) + 32.0;
    fahrenheit // Return the temperature in Fahrenheit
}