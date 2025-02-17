use std::io;


fn sum(){
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
fn num(){
    let mut x=10;
    print!("the value of x is: {}",x);
    x=11;
    print!("changed value of x id: {}",x);
}
fn newfn(){
    let x=5;
    let x=x+1;
    {
        let x=x*2;
        print!("value of x inside the scope is: {x}");
    }
    print!("value of x outside the scope is: {x}");
}
fn greet(name: &str) {
    println!("Hello, {}!", name);
}


fn main() {
    // sum();
    // num();
    // newfn();
    let mut name=String::new();
    println!("what is your name");
    io::stdin().read_line(&mut name).expect("error to take input");
    greet(&name);
   
}
