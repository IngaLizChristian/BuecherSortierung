use std::io::{self, BufRead};

use rusqlite::{Connection, Result};
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
    //öffnen der connection zur DB
    let conn = Connection::open("bibDB.db").unwrap();
    //Erstellen der Büchertabelle
    conn.execute(
        "CREATE TABLE books (
            titel    TEXT,
            authors  BLOB,
            publishedDate  TEXT
        )",
        (), // empty list of parameters.
    )
    .unwrap();

    //Erstellen der Autorentabelle
    conn.execute(
        "CREATE TABLE authors (
            titel    TEXT,
            authors  BLOB,
            publishedDate  TEXT
        )",
        (), // empty list of parameters.
    )
    .unwrap();

    //erstellen der "Wer hat was geschrieben" - Tabelle
    conn.execute(
        "CREATE TABLE whoWroteWhat (
            titel    TEXT,
            authors  BLOB,
            publishedDate  TEXT
        )",
        (), // empty list of parameters.
    )
    .unwrap();

    //------------------------------------------------------------------------ENDE ERSTELLUNG DER DATENBANKEN------------------------------------------------------------------------
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
            "INSERT INTO books (titel, authors, publishedDate) VALUES (?1, ?2, ?3)",
            (
                &response.items[0].volumeInfo.title,
                &response.items[0].volumeInfo.authors[0],
                &response.items[0].volumeInfo.publishedDate,
            ),
        )
        .unwrap();
    }
}
