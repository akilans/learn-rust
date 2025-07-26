// condition example

fn main() {
    println!("Hello, world!");

    let score = 97;

    if score >= 0 && score < 35 {
        println!("You failed in the exam!");
    }else if score >= 35 && score <= 50 {
        println!("You score average!");
    }else if score > 50 && score <= 75 {
        println!("You score good!");
    }else if score > 75 && score <=90 {
        println!("You scored well!");
    }else if score > 90 && score <= 100 {
        println!("Outstanding score!");
    }else{
        println!("Please enter a valid score  [ 0 - 100 ]");
    }
}
