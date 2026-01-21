use std::io::{self, BufRead};

fn main() {
    println!("ISBN einlesen \n");
    let stdin = io::stdin();
    let isbn = stdin.lock().lines().next().unwrap().unwrap();

    let url = format!(
        "https://www.googleapis.com/books/v1/volumes?q=isbn:{}",
        isbn
    );
    let response_text = reqwest::blocking::get(url)
        .expect("couldnt make request")
        .text()
        .expect("could not read");

    println!("\n");
    println!("ResponseText: {}", response_text);
}
