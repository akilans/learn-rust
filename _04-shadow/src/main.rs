// shadow example
// declaring a new variable with the same name
use std::io;

fn main() {
    println!("Hello, world!");
    let x: i32 = 5;
     println!("Value of x initial block {}",x);
    let x: i32 = x + 1;
    {
        let x: i32 = x + 5;
        println!("Value of x inner block {}",x);
    }
    println!("Value of x outer block {}",x);

    // same name but different data types
    let spaces = "   ";
    println!("Spaces : {}",spaces);
    let spaces = spaces.len();
    println!("Spaces : {}",spaces);


    // cli app
    println!("Enter a number");
    let mut input: String = String::new();

    io::stdin()
    .read_line(&mut input)
    .expect("Provide input");

    let input = input.trim();
    let input: i32 = input.parse().expect("Please enter a valid number");
    println!("Your number is - {}",input);


}
