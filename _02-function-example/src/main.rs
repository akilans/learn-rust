// Function examples

fn main() {
    greet(); // no return function
    say_details("Akilan",35,"Bangalore"); // fucnction with params
    let result: i32 = add(3, 7); // function with return value
    println!("Sum of {} and {} is {}",3,7,result);
}


// no return function
fn greet(){
    println!("Welome to Rust!");
}

// function with params
fn say_details(name: &str,age: u32,location: &str){
    println!("Hello, my name is {}, I'm {} years old & living in {}",name,age,location);
}

// function with return value
fn add(a: i32, b: i32) -> i32{
    //return a + b;
    a + b // no semicolon at the end
}