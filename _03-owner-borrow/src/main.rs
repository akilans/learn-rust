//In Rust, every value has a single owner. 
// When ownership moves, the original variable can no longer be used.

fn main() {
    println!("Hello, world!");
    let name: String = String::from("Akilan");
    println!("my name is {}",name);
    say_name(&name); // immutable borrow


    let new_name = name; // owner got changed
    println!("my new name is {}",new_name);
    //println!("my name is {}",name); // name no longer is accessable 

    let another_name = &new_name; // borrow value
    println!("my new name is {}",new_name); //new name is still accessable
    println!("my another name {}",another_name);

    let mut my_location: String = String::from("Tenkasi");
    add_location(&mut my_location);
    println!("My locations are {}",my_location);

    let mut my_bank = BankAccount{
        name: "Akilan".to_string(),
        balance: 10500.50
    };

    my_bank.check_balance();
    my_bank.withdraw(500.50);
    my_bank.check_balance();

 
}


// immutable borrow
fn say_name(name: &String){
    println!("Hello, {}",name);
}

// mutable borrow
fn add_location(location:&mut String){
    location.push_str(", Bangalore");
}

struct BankAccount{
    name: String,
    balance: f32
}

impl BankAccount {
    fn check_balance(&self) {
        println!("{} has {} balance in account",self.name, self.balance);
    }

    fn withdraw(&mut self, amount: f32){
        self.balance = self.balance - amount;
    }
}