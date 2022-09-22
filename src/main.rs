extern crate csv;

use std::collections::{HashMap, HashSet};
use std;
use std::env;
use std::io::Error;
use std::fs::File;
use serde::Deserialize;

#[derive(Deserialize)]
struct Record {
    user_id: usize,
    user_screen_name: String
}

fn main() -> Result<(), Error> {

    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);

    let mut map = HashMap::new();

    for result in rdr.deserialize() {
        let record: Record = result?;

        let screen_name_set = map
            .entry(record.user_id)
            .or_insert(HashSet::new());
        screen_name_set.insert(record.user_screen_name);
    }

    let mut wrt = csv::Writer::from_writer(std::io::stdout());
    wrt.write_record(&["user_id", "user_screen_name"])?;
    for (id, screen_name_set) in map.iter() {
        for screen_name in screen_name_set.iter(){
            wrt.serialize((id, screen_name))?;
        }
    }
    wrt.flush()?;

    Ok(())
}

