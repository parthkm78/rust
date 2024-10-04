fn main() {
    // Define a vector containing different types of items
    let items: Vec<(&str, i32, char, bool)> = vec![("apple", 10, 'A', true), ("banana", 20, 'B', false)];

    // Iterate over the vector and print each item
    for (name, quantity, grade, available) in &items {
        println!("Name: {}, Quantity: {}, Grade: {}, Available: {}", name, quantity, grade, available);
    }
}
