use std::io::{self, BufRead};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct TestingSerde {
    items: Vec<Item>,
}
#[derive(Deserialize, Debug)]
struct Item {
    volumeInfo: VolumeInfo,
}
#[derive(Deserialize, Debug)]
struct VolumeInfo {
    title: String,
    authors: Vec<String>,
    publishedDate: String,
}

fn main() {
    println!("ISBN einlesen");

    for isbn in io::stdin().lock().lines() {
        let isbn = isbn.unwrap();
        if isbn == "X001RPRVCZ" || isbn == "X001RPRVCY" {
            break;
        }
        let url = format!("https://www.googleapis.com/books/v1/volumes?q=isbn:{isbn}");

        let response_text = reqwest::blocking::get(url)
            .expect("couldnt make request")
            .text()
            .expect("could not read");

        let response: TestingSerde = serde_json::from_str(&response_text).unwrap();

        dbg!(response_text);
        println!("{response:?}");
    }
}
