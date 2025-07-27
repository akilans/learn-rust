// struct example
// define struct
// inside main function also possible but i want to use outside function

#[derive(Debug)] // to print struct
struct Book {
    title: String,
    author: String,
    pages: u32,
    available: bool,
}

fn main() {
    // create variable from struct

    let mut book1 = Book {
        title: String::from("book1"),
        author: String::from("author1"),
        pages: 250,
        available: true,
    };

    println!("Book1 - {:#?}", book1);
    book1.available = false;
    println!("Book1 - {:#?}", book1);

    let mut book2 = generate_book("book2".to_string(),"author2".to_string(),450,true);
    println!("Book2 - {:#?}", book2);

    // pass struct - update title
    update_book_title("book2-new".to_string(), &mut book2);
    println!("Book2 - {:#?}", book2);

    // tuple struct
    #[derive(Debug)] // to print struct
    struct Color(i32,i32,i32);
    
    let black = Color(0,0,0);
    let white = Color(255,255,255);

    println!("black - {:?}",black);
    println!("white - {:?}",white);
}

// function to return struct
fn generate_book(title: String, author: String, pages: u32,available: bool) -> Book {
    return Book {
        title: title,
        author: author,
        pages: pages,
        available: available,
    };
}


// pass struct
fn update_book_title(title: String, book: &mut Book){
    book.title = title;
}