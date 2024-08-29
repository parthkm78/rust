fn main() { 
    // we use let to declare a variable 
    let variable = 2; 
    println!("{}", variable); 
    
    // we use let with mut to make the variable mutable 
    let mut variable = variable; 
    
    println!("{}", variable); 
    variable = 5 * 2; 
    println!("{}", variable); 
} 