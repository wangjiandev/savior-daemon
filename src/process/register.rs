use crate::{
    channel::redis_tool::AppState,
    config::{GlobalConfig, CHANNEL_REGISTER, SEND_CLIENT_INFO_INTERVAL},
};
use anyhow::Result;
use if_addrs::get_if_addrs;
use redis::{Commands, RedisError};
use serde::{Deserialize, Serialize};
use std::{sync::Arc, thread, time::Duration};

/// Register the current client with the platform
/// ## Register Info
/// - The unique identifier of the current client
/// - The current client name in `settings.json` dept_name field
/// - Local IP address
///
/// ## Errors
///
///
pub fn register_process(state: &impl AppState) -> Result<thread::JoinHandle<Result<()>>> {
    let client = Arc::clone(state.client());
    let handle = thread::spawn(move || -> Result<()> {
        loop {
            let config = GlobalConfig::try_load()?;
            let ip = get_ip_address().unwrap_or("0.0.0.0".to_string());
            let register_info = RegisterInfo {
                id: config.client_id.clone(),
                name: config.client_name.clone(),
                ip,
            };
            let conn = client.get_connection();
            match conn {
                Ok(mut conn) => {
                    let json = serde_json::to_string(&register_info)?;
                    let ret: Result<(), RedisError> = conn.publish(CHANNEL_REGISTER, json);
                    match ret {
                        Ok(()) => {
                            println!("register success: {:?}", register_info);
                        }
                        Err(e) => {
                            println!("connection publish error: {:?}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("redis connection error: {:?}", e);
                }
            }
            thread::sleep(Duration::from_secs(SEND_CLIENT_INFO_INTERVAL));
        }
    });
    Ok(handle)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterInfo {
    pub id: String,
    pub name: String,
    pub ip: String,
}

fn get_ip_address() -> Result<String> {
    let addrs = get_if_addrs()?;
    for addr in addrs {
        if !addr.is_loopback() && addr.ip().is_ipv4() {
            return Ok(addr.ip().to_string());
        }
    }
    Err(anyhow::anyhow!("No non-loopback IP address found"))
}
