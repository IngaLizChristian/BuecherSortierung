use std::io::{self, BufRead};

use rusqlite::{Connection, Result};
use serde::Deserialize;
mod create_tables;

#[derive(Deserialize, Debug)]
struct TestingSerde {
    items: Vec<Item>,
}
#[derive(Deserialize, Debug)]
struct Item {
    volume_info: VolumeInfo,
}
#[derive(Deserialize, Debug)]
struct VolumeInfo {
    title: String,
    author: Vec<String>,
    published_date: String,
}

fn main() {
    //Öffnen der connection zur DB
    let conn = Connection::open("bibDB.db").unwrap();

    //---------------------------------------------------------------------einkommentieren wenn benötigt-----------------------------------------------------------------------------
    //create_tables::create();
    //-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
    println!("ISBN einlesen");
    //-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
    //------------------------------------------------------------------------ANFANG DATEN EINLESEN----------------------------------------------------------------------------------

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
        println!("ResponseText: {response_text}");
        println!("{response:?}");

        //Datenbank befüllen
        assert!(response.items.len() == 1);
        conn.execute(
            "INSERT INTO books (titel, author, publishedDate) VALUES (?1, ?2, ?3)",
            (
                &response.items[0].volume_info.title,
                &response.items[0].volume_info.author[0],
                &response.items[0].volume_info.published_date,
            ),
        )
        .unwrap();
    }
}
