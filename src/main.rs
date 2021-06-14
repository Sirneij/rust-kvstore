use std::io::prelude::*;
use std::{collections::HashMap, path::PathBuf};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    // let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);
    let key = &args[1];
    let value = &args[2];
    // let key = args.next().expect("Key was not found");
    // let value = args.next().expect("Value could not be found.");
    let mut database = Database::new().expect("Corrupt database");
    database.insert(key.to_string(), value.to_string());
}

/// This serves as the struct or structure
/// for the database
struct Database {
    /// Only map field is involved which is
    /// of type HashMap since we want a key-value pair
    map: HashMap<String, String>,
}

/// Implementation of the Database struct
impl Database {
    /// Returns a `Result<Database, std::io::Error>` which means a database
    /// or a standard error is returned. An alias to this return type is
    /// `std::io::Result<()>`
    ///
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// let database = Database::new().expect("Corrupt database");
    /// ```
    fn new() -> Result<Database, std::io::Error> {
        let mut map: HashMap<String, String> = HashMap::new();
        // read the content of the file
        // Some other way to have done this is
        // ```
        // let contents = match std::fs::read_to_string("kv.db") {
        //    Ok(c) => c,
        //     Err(error) => {
        //        return Err(error);
        //    }
        //};
        // ```

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
    std::fs::write("kv.db", contents).expect("Unable to create kv.db");
    Ok(())
}
