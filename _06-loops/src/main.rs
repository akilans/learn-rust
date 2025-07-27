// Example of loops
// loop, multi loops, while, for

fn main() {
    // loop 1 example
    let mut counter = 1;
    loop {
        println!("Counting {counter}");
        if counter == 10 {
            break;
        }
        counter += 1;
    }

    // loop with return value
    counter = 0;

    let mut value_from_loop = loop{
        counter += 1;

        if counter == 5{
            break counter * counter;
        }
    };

    println!("Value from loop is - {value_from_loop}");

    // while loop example
    
    while value_from_loop != 0 {
        println!("While - {value_from_loop}");
        value_from_loop -= 1;
    }
    
    // for loop example 1

    let friends = ["Alex","Kumar","Kathir"];

    for friend in friends{
        println!("Hello {friend}");
    }

    // for loop example 2

    for num in 1..10 {
        println!("For {num}");
    }

    // for loop example 3
    let arr = [10, 20, 30, 40];

    for (index, value) in arr.iter().enumerate() {
        println!("Index: {}, Value: {}", index, value);
    }
}
