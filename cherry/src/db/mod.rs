use std::fs::File;
use std::io::BufReader;

use crate::model::CherryData;

/// Directory used to store data files
const DATA_DIR: &str = ".data";
/// Name of the file of cherry data
const CHERRY_DATA_FILE_NAME: &str = "cherry_data";

pub(crate) fn load_data_from_disk() -> eyre::Result<CherryData> {
    // if the directory doesn't exist then create it
    std::fs::create_dir_all(DATA_DIR)?;
    let coffees: CherryData = match File::open(format!("{DATA_DIR}/{CHERRY_DATA_FILE_NAME}")) {
        Ok(file) => {
            let reader = BufReader::new(file);
            ciborium::from_reader(reader)?
        }
        Err(_) => CherryData { coffees: vec![] },
    };
    Ok(coffees)
}

pub(crate) fn write_date_to_disk(cherry_data: &CherryData) -> eyre::Result<()> {
    // save the state to disk for loading next time
    let file = File::create(format!("{DATA_DIR}/{CHERRY_DATA_FILE_NAME}"))?;
    ciborium::into_writer(cherry_data, std::io::BufWriter::new(file))?;
    Ok(())
}
