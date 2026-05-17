use rusqlite::Connection;

pub fn create() {
    let conn = Connection::open("bibDB.db").unwrap();
    //-------------------------------------------AUTOREN_TABELLEN----------------------------------------------------------------

    //Erstellen der Autorentabelle
    conn.execute(
        "CREATE TABLE author (
            author_id               INTEGER NOT NULL,

            author_first_name       VARCHAR(255),
            author_last_name        VARCHAR(255),
            author_sex              CHAR(1),
            author_birth_date       DATE,
            author_death_year       DATE,

            PRIMARY KEY (author_id),
            UNIQUE(author_id)
        )",
        (), // empty list of parameters.
    )
    .unwrap();

    //erstellen der "Wer hat was geschrieben" - Tabelle, reine JOIN tabelle
    conn.execute(
        "CREATE TABLE wrote (
            author_id           INTEGER,
            isbn                INTEGER,

            FOREIGN KEY(author_id)  REFERENCES author(author_id),
            FOREIGN KEY(isbn)       REFERENCES book(isbn)
        )",
        (), // empty list of parameters.
    )
    .unwrap();

    //-------------------------------------------BÜCHER_TABELLEN----------------------------------------------------------------

    //----------------------------WICHTIGE HINWEISE ZU DEN BÜCHERN----------------------------
    /*
        Bild einfügen
            INSERT INTO books (cover_image_name, cover_image_path)
            VALUES ('logo', LOAD_FILE('/pfad/zum/bild.png'));

        Bild auslesen
            SELECT cover_image_name, cover_image_path FROM books WHERE isbn = XXXXX;

        Bild anzeigen, bsp php
            echo '<img src="' . $row['cover_image_path'] . '" alt="Titelbild">';

        -------------------------------------------

        liked_rating goes from 1-10, like a star system

        -------------------------------------------

        reading_state exampels: - currently reading
                                - read
                                - to be read
                                - didn't finish

        -------------------------------------------

        owned_state exampels:   - owned
                                - borrowed
                                - lend
                                - wishlist
                                - sold
                                - lost
                                - gifted

        -------------------------------------------

        mature_rating can be set individually, for example if it can be read by small children, read by small children under supervision, read by teenagers without any problems,
        is only for adults, mostly will be used as a yes no thingy, but is if it has specific limitations by whom it should be read
    */
    //----------------------------------------------------------------------------------------

    //Erstellen der Büchertabelle
    conn.execute(
        "CREATE TABLE book (
            isbn                    INTEGER NOT NULL,
            copy_number             INTEGER NOT NULL,

            titel                   TEXT,
            page_count              INTEGER,
            release_date            DATE,
            copyright_date          DATE,
            language                CHAR(2),
            description             TEXT,
            
            cover_type              VARCHAR(50),
            cover_image_name        VARCHAR(255),
            cover_image_path        TEXT,
            
            mature_rating           VARCHAR(255),
            liked_rating            SMALLINT,
            place_of_discovery      VARCHAR(255),
            date_of_aquirement      DATE,
            reading_state           VARCHAR(50),
            owned_state             VARCHAR(100),

            PRIMARY KEY (isbn, copy_number),
            UNIQUE(isbn),
            UNIQUE(copy_number)
        )",
        (), // empty list of parameters.
    )
    .unwrap();

    //Erstellen der has_tag Tabelle
    conn.execute(
        "CREATE TABLE has_tag (
            isbn                    INTEGER,
            copy_number             INTEGER,
            tag_id                  INTEGER,

            FOREIGN KEY(isbn)           REFERENCES book(isbn),
            FOREIGN KEY(copy_number)    REFERENCES book(copy_number),
            FOREIGN KEY(tag_id)         REFERENCES tag(tag_id)
        )",
        (), // empty list of parameters.
    )
    .unwrap();

    //Erstellen der tag Tabelle
    conn.execute(
        "CREATE TABLE tag (
            tag_id                 INTEGER NOT NULL,
            tag_name               VARCHAR(255),

            PRIMARY KEY(tag_id),
            UNIQUE(tag_id)
        )",
        (), // empty list of parameters.
    )
    .unwrap();

    //Erstellen der has_genre Tabelle
    conn.execute(
        "CREATE TABLE has_genre (
            isbn                    INTEGER,
            genre_id                INTEGER,
            primary_genre           BOOLEAN,

            FOREIGN KEY(isbn)               REFERENCES book(isbn),
            FOREIGN KEY(genre_id)           REFERENCES genre(genre_id)
        )",
        (), // empty list of parameters.
    )
    .unwrap();

    //Erstellen der genre Tabelle
    //---------------------------------
    /*
        Genre-Verschachtelung Beispiele:

        Horror  - Psycho - Horror
                - hack'n'slash
                - kosmischer - Horror
        Möglichst nie tiefer als 2 Ebenen runter gehen, maybe durch ein dropdown festsetzen?
    */
    //---------------------------------
    conn.execute(
        "CREATE TABLE genre (
            genre_id                INTEGER NOT NULL,
            parent_genre_id         INTEGER,
            genre_name              VARCHAR(255),
            
            PRIMARY KEY(genre_id),
            UNIQUE(genre_id),
            FOREIGN KEY(parent_genre_id)     REFERENCES genre(genre_id)
        )",
        (), // empty list of parameters.
    )
    .unwrap();

    //Erstellen der is_part_of Tabelle
    conn.execute(
        "CREATE TABLE is_part_of (
            isbn                    INTEGER,
            series_id               INTEGER,

            season                  INTEGER,
            position                INTEGER,

            FOREIGN KEY(isbn)               REFERENCES book(isbn),
            FOREIGN KEY(series_id)          REFERENCES series(series_id)
        )",
        (), // empty list of parameters.
    )
    .unwrap();

    //Erstellen der series Tabelle
    conn.execute(
        "CREATE TABLE series (
            series_id               INTEGER NOT NULL,

            series_titel            VARCHAR(255),
            series_description      TEXT,

            PRIMARY KEY(series_id),
            UNIQUE(series_id)
        )",
        (), // empty list of parameters.
    )
    .unwrap();
    //------------------------------------------------------------------------ENDE ERSTELLUNG DER DATENBANKEN------------------------------------------------------------------------
}
