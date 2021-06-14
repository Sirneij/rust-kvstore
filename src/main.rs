use std::io::prelude::*;
use std::{collections::HashMap, path::PathBuf};

fn main() {
    let mut args = std::env::args().skip(1);
    let key = args.next().expect("Key was not found");
    let value = args.next().expect("Value could not be found.");
    let mut database = Database::new().expect("Corrupt database");
    database.insert(key, value);
}

/// This serves as thestruct or structure
/// for the database
struct Database {
    /// Only map field is involved which is
    /// of type HashMap
    map: HashMap<String, String>,
}

/// Implementation of the Database struct
impl Database {
    /// new function ("constructor" in other languages)
    /// It returns Result<T, Error> where T is the Database
    fn new() -> Result<Database, std::io::Error> {
        // read the content of the file
        // let contents = match std::fs::read_to_string("kv.db") {
        //     Ok(c) => c,
        //     Err(error) => {
        //         return Err(error);
        //     }
        // };
        let mut map: HashMap<String, String> = HashMap::new();
        let mut contents = String::new();
        let path = PathBuf::from("kv.db");
        if path.exists() {
            let file = std::fs::File::open(path)?;
            let mut buf_reader = std::io::BufReader::new(file);
            buf_reader.read_to_string(&mut contents)?;
        } else {
            std::fs::File::create("kv.db")?;
        }
        for line in contents.lines() {
            let (key, value) = line.split_once("\t").expect("Corrupt database");
            map.insert(key.to_string(), value.to_string());
        }
        Ok(Database { map })
    }

    fn insert(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }
    // fn flush(self) -> std::io::Result<()> {
    //     do_flush(&self)
    // }
}

impl Drop for Database {
    fn drop(&mut self) {
        let _ = do_flush(self);
    }
}

fn do_flush(database: &Database) -> std::io::Result<()> {
    let mut contents = String::new();
    for (key, value) in &database.map {
        contents.push_str(key);
        contents.push('\t');
        contents.push_str(value);
        contents.push('\n');
    }

    Ok(std::fs::write("kv.db", contents).expect("Unable to create kv.db"))
}
