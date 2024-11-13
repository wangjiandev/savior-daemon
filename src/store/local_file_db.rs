use anyhow::Result;
use serde_json::Value;
use std::fs::File;
use std::path::Path;

/// 读取指定路径的json文件, 并返回一个json对象
pub fn read_json_file<P: AsRef<Path>>(path: P) -> Result<Value> {
    let file = File::open(path)?;
    let value = serde_json::from_reader(file)?;
    Ok(value)
}

/// 写入json对象到指定路径的文件
pub fn write_json_file<P: AsRef<Path>>(path: P, value: &Value) -> Result<()> {
    let file = File::create(path)?;
    serde_json::to_writer(file, value)?;
    Ok(())
}
