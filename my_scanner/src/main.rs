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
    author: Vec<String>,
    publishedDate: String,
}

fn main() {
    //Öffnen der connection zur DB
    let conn = Connection::open("bibDB.db").unwrap();
    //Erstellen der Büchertabelle
    conn.execute(
        "CREATE TABLE books (
            isbn                    INTEGER NOT NULL,
            copy_number             INTEGER NOT NULL,
            titel                   TEXT,
            page_count              INTEGER,
            release_date            DATE,
            copyright_date          DATE,
            cover_type              VARCHAR(50),
            mature_rating           VARCHAR(255),
            language                CHAR(2),
            cover_image_name        VARCHAR(255),
            cover_image_path        TEXT,
            liked_rating            SMALLINT,
            description             TEXT,
            place_of_discovery      VARCHAR(255),
            date_of_aquirement      DATE,

            PRIMARY KEY (isbn, copy_number),
            UNIQUE(isbn),
            UNIQUE(copy_number)
        )",
        (), // empty list of parameters.
    )
    .unwrap();
    //-------------------------------------------WICHTIGE HINWEISE ZU DEN BÜCHERN----------------------------------------------------------------
    /*
        Bild einfügen
            INSERT INTO books (cover_image_name, cover_image_path)
            VALUES ('logo', LOAD_FILE('/pfad/zum/bild.png'));

        Bild auslesen
            SELECT cover_image_name, cover_image_path FROM books WHERE isbn = XXXXX;

        bild anzeigen, bsp php
            echo '<img src="' . $row['cover_image_path'] . '" alt="Titelbild">';
    */
    //-------------------------------------------------------------------------------------------------------------------------------------------

    //Erstellen der Autorentabelle
    conn.execute(
        "CREATE TABLE author (
            author_id               INTEGER NOT NULL,
            author_first_name       VARCHAR(255),
            author_last_name        VARCHAR(255),
            author_sex              CHAR(1),
            PRIMARY KEY (author_id),
            UNIQUE(author_id)
        )",
        (), // empty list of parameters.
    )
    .unwrap();

    //erstellen der "Wer hat was geschrieben" - Tabelle
    conn.execute(
        "CREATE TABLE wrote (
            titel    TEXT,
            author  BLOB,
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
            "INSERT INTO books (titel, author, publishedDate) VALUES (?1, ?2, ?3)",
            (
                &response.items[0].volumeInfo.title,
                &response.items[0].volumeInfo.author[0],
                &response.items[0].volumeInfo.publishedDate,
            ),
        )
        .unwrap();
    }
}
