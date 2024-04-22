use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;

use crate::opts::OutputFormat;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Player {
    name: String,
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

pub fn process_csv(input: &str, output: String, format: OutputFormat) -> Result<()> {
    let mut rdr = csv::Reader::from_path(input).unwrap();
    let mut ret = Vec::with_capacity(128);
    let headers = rdr.headers()?.clone();
    for record in rdr.records() {
        let record = record?;
        let value = headers
            .iter()
            .zip(record.iter())
            .collect::<serde_json::Value>();
        ret.push(value);
    }

    let res = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
    };
    fs::write(output, res)?;

    Ok(())
}
