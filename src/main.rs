//dziala!
use rusqlite::{Connection, Result};

use serde_derive::{Deserialize, Serialize};
use serde_rusqlite::from_rows;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Person {
    id: i32,
    name: String,
    surname: String,
}

impl Person {
    fn print_id(&self) {
        println!("dupa: {:?}", self.id);
    }
}

fn print_database() -> Result<()> {
    let conn = Connection::open("Persons.db")?;

    // first way:
    // let mut query = conn.prepare("SELECT id,name,surname from Person")?;
    // let state_iter = query.query_map(NO_PARAMS, |row| {
    //     Ok(Person {
    //         id: row.get(0)?,
    //         name: row.get(1)?,
    //         surname: row.get(2)?,
    //     })
    // })?;
    
    //second way
    let mut query = conn.prepare("SELECT * from Person")?;
    let map = from_rows::<Person>(query.query([])?);
    for data in map {
        let person = data.unwrap();
        println!("{:?}", person);
        person.print_id();
    }
    Ok(())
}

fn main() {
    print_database().unwrap();
}
