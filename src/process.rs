use csv::{Reader, StringRecord};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;

use crate::opts::OutFormat;

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

pub fn process_csv(input: &str, output: String, format: OutFormat) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    let headers = reader.headers()?.clone();
    for result in reader.records() {
        let record: StringRecord = result?;
        let value = headers.iter().zip(record.iter()).collect::<Value>();
        ret.push(value);
    }

    let content: String = match format {
        OutFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutFormat::Yaml => serde_yaml::to_string(&ret)?,
        OutFormat::Toml => {
            // TOML 顶层不能直接是数组，包一层结构生成 [[records]] 表数组
            #[derive(Serialize)]
            struct Wrapper<'a> {
                records: &'a [Value],
            }
            toml::to_string(&Wrapper { records: &ret })?
        }
    };

    fs::write(output, content)?;
    Ok(())
}
