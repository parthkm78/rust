use std::io; // Import the standard library's input/output module

fn main() {
    println!("Please enter your birth year:"); // Prompt the user to enter their birth year

    let mut input = String::new(); // Create a mutable string to store user input

    io::stdin().read_line(&mut input).expect("Failed to read line"); // Read the user's input

    let birth_year: u32 = input.trim().parse().expect("Invalid input"); // Parse the input as an unsigned 32-bit integer

    let current_year = 2024; // Define the current year

    let age = current_year - birth_year; // Calculate the age by subtracting the birth year from the current year

    println!("Your age is {} years old.", age); // Print the calculated age
}