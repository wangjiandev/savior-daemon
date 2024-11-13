use std::path::Path;

use anyhow::{Ok, Result};
use if_addrs::get_if_addrs;
use serde_json::json;
use uuid::Uuid;

use crate::store::local_file_db::read_json_file;
/// Register the current client with the platform
/// ## Register Info
/// - The unique identifier of the current client
/// - The current client name in `settings.json` dept_name field
/// - Local IP address
///
/// ## Errors
///
///
pub fn register_process() -> Result<RegisterInfo> {
    let info = RegisterInfo::try_load()?;
    Ok(info)
}

#[derive(Debug)]
pub struct RegisterInfo {
    pub client_id: String,
    pub client_name: String,
    pub ip_address: String,
}

impl RegisterInfo {
    pub fn try_load() -> Result<Self> {
        let client_id = get_client_id()?;
        let ip_address = get_ip_address()?;
        let client_name = get_client_name()?;
        Ok(Self {
            client_id,
            client_name,
            ip_address,
        })
    }
}

fn get_client_id() -> Result<String> {
    let org =
        read_json_file("org.json").unwrap_or_else(|_| json!({ "id": Uuid::new_v4().to_string() }));
    let client_id = org.get("id");
    Ok(client_id.unwrap().as_str().unwrap().to_string())
}

fn get_client_name() -> Result<String> {
    let settings_path = Path::new("settings.json");
    if !settings_path.exists() {
        return Ok("未命名".to_string());
    }
    let settings = read_json_file(settings_path)?;
    let client_name = settings.get("dept_name");
    if client_name.is_none() {
        return Ok("未命名".to_string());
    }
    Ok(client_name.unwrap().as_str().unwrap().to_string())
}

fn get_ip_address() -> Result<String> {
    let addrs = get_if_addrs()?;
    for addr in addrs {
        if !addr.is_loopback() {
            return Ok(addr.ip().to_string());
        }
    }
    Err(anyhow::anyhow!("No non-loopback IP address found"))
}
