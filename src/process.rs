use calamine::{Reader as XlsxReader, Xlsx, open_workbook};
use csv::{Reader, StringRecord, Writer};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::path::Path;

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
        OutFormat::Csv | OutFormat::Markdown => {
            return Err(anyhow::anyhow!(
                "CSV和Markdown格式不支持在process_csv函数中使用"
            ));
        }
    };

    fs::write(output, content)?;
    Ok(())
}

/// 处理XLSX文件，将每个工作表转换为指定格式
pub fn process_xlsx(
    input: &str,
    output_dir: &str,
    format: OutFormat,
    remove_empty_rows: bool,
    trim_whitespace: bool,
) -> anyhow::Result<()> {
    // 打开XLSX文件
    let mut workbook: Xlsx<_> = open_workbook(input)?;

    // 获取文件名（不含扩展名）
    let input_path = Path::new(input);
    let file_stem = input_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output");

    // 确保输出目录存在
    fs::create_dir_all(output_dir)?;

    // 获取所有工作表名称
    let sheet_names = workbook.sheet_names().to_owned();

    for sheet_name in sheet_names {
        println!("正在处理工作表: {}", sheet_name);

        // 读取工作表数据
        if let Ok(range) = workbook.worksheet_range(&sheet_name) {
            let mut data = Vec::new();
            let mut headers = Vec::new();
            let mut first_row = true;

            for row in range.rows() {
                let mut row_data = Vec::new();
                let mut is_empty_row = true;

                for cell in row {
                    let mut cell_value = cell.to_string();

                    // 去除空格处理
                    if trim_whitespace {
                        cell_value = cell_value.trim().to_string();
                    }

                    if !cell_value.is_empty() {
                        is_empty_row = false;
                    }

                    row_data.push(cell_value);
                }

                // 跳过空行处理
                if remove_empty_rows && is_empty_row {
                    continue;
                }

                if first_row {
                    headers = row_data;
                    first_row = false;
                } else {
                    data.push(row_data);
                }
            }

            // 生成输出文件名
            let output_filename = format!("{}_{}.{}", file_stem, sheet_name, format);
            let output_path = Path::new(output_dir).join(output_filename);

            // 根据格式生成文件
            match format {
                OutFormat::Csv => {
                    generate_csv(&output_path, &headers, &data)?;
                }
                OutFormat::Markdown => {
                    generate_markdown(&output_path, &headers, &data)?;
                }
                OutFormat::Json => {
                    generate_json(&output_path, &headers, &data)?;
                }
                OutFormat::Yaml => {
                    generate_yaml(&output_path, &headers, &data)?;
                }
                OutFormat::Toml => {
                    generate_toml(&output_path, &headers, &data)?;
                }
            }

            println!("已生成文件: {}", output_path.display());
        }
    }

    Ok(())
}

/// 生成CSV文件
fn generate_csv(
    output_path: &Path,
    headers: &[String],
    data: &[Vec<String>],
) -> anyhow::Result<()> {
    let mut writer = Writer::from_path(output_path)?;

    // 写入表头
    writer.write_record(headers)?;

    // 写入数据行
    for row in data {
        writer.write_record(row)?;
    }

    writer.flush()?;
    Ok(())
}

/// 生成Markdown表格文件
fn generate_markdown(
    output_path: &Path,
    headers: &[String],
    data: &[Vec<String>],
) -> anyhow::Result<()> {
    let mut content = String::new();

    // 写入表头
    content.push_str("| ");
    for header in headers {
        content.push_str(&format!("{} | ", header));
    }
    content.push('\n');

    // 写入分隔行
    content.push_str("| ");
    for _ in headers {
        content.push_str("--- | ");
    }
    content.push('\n');

    // 写入数据行
    for row in data {
        content.push_str("| ");
        for (i, cell) in row.iter().enumerate() {
            if i < headers.len() {
                // 转义Markdown特殊字符
                let escaped_cell = cell.replace('|', "\\|").replace('\n', "<br>");
                content.push_str(&format!("{} | ", escaped_cell));
            }
        }
        content.push('\n');
    }

    fs::write(output_path, content)?;
    Ok(())
}

/// 生成JSON文件
fn generate_json(
    output_path: &Path,
    headers: &[String],
    data: &[Vec<String>],
) -> anyhow::Result<()> {
    let mut records = Vec::new();

    for row in data {
        let mut record = serde_json::Map::new();
        for (i, header) in headers.iter().enumerate() {
            let value = row.get(i).unwrap_or(&String::new()).clone();
            record.insert(header.clone(), Value::String(value));
        }
        records.push(Value::Object(record));
    }

    let json_content = serde_json::to_string_pretty(&records)?;
    fs::write(output_path, json_content)?;
    Ok(())
}

/// 生成YAML文件
fn generate_yaml(
    output_path: &Path,
    headers: &[String],
    data: &[Vec<String>],
) -> anyhow::Result<()> {
    let mut records = Vec::new();

    for row in data {
        let mut record = serde_json::Map::new();
        for (i, header) in headers.iter().enumerate() {
            let value = row.get(i).unwrap_or(&String::new()).clone();
            record.insert(header.clone(), Value::String(value));
        }
        records.push(Value::Object(record));
    }

    let yaml_content = serde_yaml::to_string(&records)?;
    fs::write(output_path, yaml_content)?;
    Ok(())
}

/// 生成TOML文件
fn generate_toml(
    output_path: &Path,
    headers: &[String],
    data: &[Vec<String>],
) -> anyhow::Result<()> {
    let mut records = Vec::new();

    for row in data {
        let mut record = serde_json::Map::new();
        for (i, header) in headers.iter().enumerate() {
            let value = row.get(i).unwrap_or(&String::new()).clone();
            record.insert(header.clone(), Value::String(value));
        }
        records.push(Value::Object(record));
    }

    // TOML 顶层不能直接是数组，包一层结构
    #[derive(Serialize)]
    struct Wrapper<'a> {
        records: &'a [Value],
    }

    let toml_content = toml::to_string(&Wrapper { records: &records })?;
    fs::write(output_path, toml_content)?;
    Ok(())
}
