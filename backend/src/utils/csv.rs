use anyhow::Result;
use std::io::Read;
use csv::{Reader, Writer};
use serde::{Serialize, Deserialize};

pub fn read_csv<T: for<'de> Deserialize<'de>>(data: &[u8]) -> Result<Vec<T>> {
    let mut reader = Reader::from_reader(data);
    let mut records = Vec::new();

    for result in reader.deserialize() {
        let record: T = result?;
        records.push(record);
    }

    Ok(records)
}

pub fn write_csv<T: Serialize>(data: &[T]) -> Result<Vec<u8>> {
    let mut buffer = Vec::new();
    {
        let mut writer = Writer::from_writer(&mut buffer);

        for record in data {
            writer.serialize(record)?;
        }

        writer.flush()?;
    }

    Ok(buffer)
}