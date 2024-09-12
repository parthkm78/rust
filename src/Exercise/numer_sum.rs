use std::io;

fn main() {
    println!("Enter First number");
    // create String instance
    let mut input1 = String::new();
    // taking from commandline
    io::stdin().read_line(&mut input1).expect("Error while input");
    // converting input to number
    let num1: f64 = input1.trim().parse().expect("Please input numeric number");
     println!("First number is {}", num1);
     println!("Enter second number:");
     let mut input2 = String::new();
     io::stdin().read_line(&mut input2).expect("Error while input");
     let num2: f64 = input2.trim().parse().expect("Please input numeric number");
     println!("Second input is: {}", num2);
     
     let sum = num1 + num2;
     println!("Sum is {}", sum);
   }