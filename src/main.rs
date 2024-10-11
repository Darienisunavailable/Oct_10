use std::fs::File;
use std::io::{Write, BufReader, BufRead};
use std::path::Path;

struct Book {
    title: String,
    author: String,
    year: u16,
}

fn save_books(books: &Vec<Book>, filename: &str) {
    // TODO: Implement this function
    // Hint: Use File::create() and write!() macro

    // Creates file named filename
    let mut file = File::create(filename).unwrap();

    // For each book in the books vector, it will write
    // the title, author, and year of each book to the
    // file
    for book in books {
        writeln!(file, "{},{},{}", book.title, book.author, book.year).unwrap();
    }
}

fn load_books(filename: &str) -> Vec<Book> {
    // Following notes from GitHub
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut books: Vec<Book> = Vec::new();


    // For each line that will be read from file,
    // create a unique vector
    for line in reader.lines() {
        let line = line.unwrap();

        // This vector will be able to determine what words to put in
        // each slot because of the split at commas
        let each: Vec<&str> = line.split(',').collect();

        // Assign each element of a book the words
        // read from the file
        let title = each[0].to_string();
        let author = each[1].to_string();
        let year = each[2].parse::<u16>().unwrap();

        // Push each book with its characteristics
        // to the vector called books
        books.push(Book { title, author, year });
    }

    // Return the vector
    books
}

fn main() {
    let books = vec![
        Book { title: "1984".to_string(), author: "George Orwell".to_string(), year: 1949 },
        Book { title: "To Kill a Mockingbird".to_string(), author: "Harper Lee".to_string(), year: 1960 },
    ];

    save_books(&books, "books.txt");
    println!("Books saved to file.");

    let loaded_books = load_books("books.txt");
    println!("Loaded books:");
    for book in loaded_books {
         println!("{} by {}, published in {}", book.title, book.author, book.year);
    }
}