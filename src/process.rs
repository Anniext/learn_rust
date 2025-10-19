use csv::Reader;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Record {
    name: String,     // 姓名
    position: String, // 位置
    #[serde(rename = "DOB")]
    dob: String, // 出生日期
    nationality: String, // 国籍
    #[serde(rename = "Kit Number")]
    kit: u8, // 球衣号码
}

pub fn process_csv(input: &str, output: &str) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    for result in reader.deserialize() {
        let record: Record = result?;
        ret.push(record);
    }

    let json = serde_json::to_string_pretty(&ret)?;
    fs::write(output, json)?;
    Ok(())
}
