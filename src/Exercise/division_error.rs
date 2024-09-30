fn main() {
    // Define dividend and divisor
    let dividend = 10;
    let divisor = 0; // Change to 0 to simulate division by zero error

    // Perform the division operation
    let result = divide(dividend, divisor);

    // Match on the result to handle success and failure cases
    match result {
        // If division is successful, print the result
        Ok(value) => println!("Result of division: {}", value),
        // If division fails (division by zero), print an error message
        Err(error) => println!("Error: {}", error),
    }
}

// Define a function named 'divide' that performs division and returns a Result
fn divide(dividend: i32, divisor: i32) -> Result<i32, &'static str> {
    // Check if divisor is zero
    if divisor == 0 {
        // If divisor is zero, return an error
        Err("Division by zero is not allowed!")
    } else {
        // If divisor is non-zero, perform the division and return the result
        Ok(dividend / divisor)
    }
}