//Important concepts of Rust lang

fn main() {
    println!("Hello, Akilan!, Welcome to Rust!");

    // Primitive data types - int, float, bool, char

    //signed integer - i8, i16, i32, i64, i128
    //unsigned integer - u8, u16, u32, u64, u128

    let age: i32 = 35;
    let profit: i64 = -10;
    let experience: u8 = 12;

    println!("Age is {}",age);
    println!("Profit is {}",profit);
    println!("Experience is {}",experience);

    println!("i32 max number is {}",std::i32::MAX);

    let weight: f32 = 81.50;
    println!("Weight is {}",weight);

    let is_raining: bool = true;
    println!("Is raining? - {}",is_raining);

    let my_fav_letter: char = 's';
    println!("My favourite letter is - {}",my_fav_letter);


    // Compound data types - arrays, tuples, slices, strings ( slice strings)

    // arrays - collection of same data types

    let numbers: [i32; 5] = [1,2,3,4,5];
    println!("Numbers {:?}",numbers);

    // tuples - collection of different data types

    let my_details: (String, i32, String) = ("Akilan".to_string(),35,"Bangalore".to_string());
    // Akilan & Bangalore are not string - that is string slice
    println!("My details {:?}",my_details);

    // Slices

    let my_num_slice: &[i32] = &[1,2,3,4,5];
    println!("My number slice {:?}",my_num_slice);

    let my_friends: &[&str] = &["Alex","Kumar"];
    println!("My friends {:?}",my_friends);


    // String and String slices (&str)
    let friend_details: (&str, i32, &str) = ("Alex",36,"Tenkasi");
    println!("Friend details {:?}",friend_details);

    let location: &str = "Bangalore";
    println!("My location is {}",location);


    
}
