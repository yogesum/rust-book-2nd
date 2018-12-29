#!/usr/bin/env run-cargo-script
// cargo-deps: rusqlite, postgres

extern crate rusqlite;
extern crate postgres;


fn create_sqlite_db() -> rusqlite::Result<()> {
    use rusqlite::{Connection, NO_PARAMS};

    let conn = Connection::open("cats.db")?;

    conn.execute(
        "create table if not exists cat_colors (
            id integer primary key,
            name text not null
        )",
        NO_PARAMS,
    )?;

    conn.execute(
        "create table if not exists cats (
            id integer primary key,
            name text not null,
            date_of_birth datetime,
            color_id integer not null references cat_colors(id)
        )",
        NO_PARAMS,
    )?;

    Ok(())
}

fn create_postgres() -> Result<(), postgres::Error> {
    use postgres::{Connection, TlsMode};

    let conn = Connection::connect(
        "postgresql://pguser:pguser@localhost/library",
        TlsMode::None,
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS author (
            id SERIAL PRIMARY KEY,
            name VARCHAR NOT NULL,
            country VARCHAR NOT NULL
        )",
        &[],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS book (
            id SERIAL PRIMARY KEY,
            title VARCHAR NOT NULL,
            author_id INTEGER NOT NULL REFERENCES author
        )",
        &[],
    )?;

    Ok(())
}

struct Author {
    id: i32,
    name: String,
    country: String,
}

fn query_postgres() -> Result<(), postgres::Error> {
    use std::collections::HashMap;
    use postgres::{Connection, TlsMode};

    let conn = Connection::connect(
        "postgresql://pguser:pguser@localhost/library",
        TlsMode::None,
    )?;

    let mut authors = HashMap::new();
    authors.insert(String::from("Chinua Achebe"), "Nigeria");
    authors.insert(String::from("Rabindranath Tagore"), "India");
    authors.insert(String::from("Anita Nair"), "India");

    for (key, value) in &authors {
        let author = Author {
            id: 0,
            name: key.to_string(),
            country: value.to_string(),
        };

        conn.execute(
            "INSERT INTO author (name, country) VALUES ($1, $2)",
            &[&author.name, &author.country],
        )?;
    }

    for row in &conn.query("SELECT id, name, country FROM author", &[])? {
        let author = Author {
            id: row.get(0),
            name: row.get(1),
            country: row.get(2),
        };

        println!("Author {} is from {}", author.name, author.country);
    }

    Ok(())
}

fn main() {
    create_sqlite_db().unwrap();
    create_postgres().unwrap();
    query_postgres().unwrap();
}