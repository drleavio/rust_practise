use std::io;

fn main() {
    
    let mut num1 = String::new();
    let mut num2 = String::new();

   
    println!("Enter the first number:");
    io::stdin().read_line(&mut num1).expect("Failed to read input");

    
    println!("Enter the second number:");
    io::stdin().read_line(&mut num2).expect("Failed to read input");

   
    let num1: i32 = num1.trim().parse().expect("Please enter a valid number");
    let num2: i32 = num2.trim().parse().expect("Please enter a valid number");

    
    let sum = num1 + num2;
    println!("The sum of {} and {} is: {}", num1, num2, sum);
}
