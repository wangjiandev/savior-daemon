use crate::store::local_file_db::{read_json_file, write_json_file};
use anyhow::Result;
use platform_dirs::AppDirs;
use serde::{Deserialize, Serialize};
use serde_json::from_value;
use std::{env, path::PathBuf};
use uuid::Uuid;

/// 用于注册客户端的redis通道
pub const CHANNEL_REGISTER: &str = "ch_server_register";
/// 用于接收客户端信息的redis通道
pub const CHANNEL_NOTIFICATION: &str = "ch_server_notification";
/// 发送客户端信息间隔（秒）
pub const SEND_CLIENT_INFO_INTERVAL: u64 = 5;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GlobalConfig {
    pub root_home: String,
    pub code: String,
    pub name: String,
    pub place: String,
    pub client_id: String,
    pub client_name: String,
    pub fep_address: String,
}

impl GlobalConfig {
    /// Prioritize reading local configuration files.
    /// If there is no configuration file, read system environment variables.
    /// - SAVIOR_ORG_CODE: The organization code of the current user. default is "100000".
    /// - SAVIOR_ORG_NAME: The name of the organization. default is "得之医疗".
    /// - SAVIOR_ORG_PLACE: The place of the organization. default is "1".
    /// - SAVIOR_FEP_ADDRESS: The address of the fep server. default is "192.168.1.124:6379".
    /// - SAVIOR_FILE_PATH: The path of the file storage. default is "/data/savior".
    pub fn try_load() -> Result<Self> {
        let savior_file_path = get_app_data_dir()?;
        let settings_path = savior_file_path.join("settings.json");
        if let Ok(data) = read_json_file(&settings_path) {
            from_value::<GlobalConfig>(data)
                .map_err(|_| anyhow::anyhow!("读取配置settings.json文件失败"))
        } else {
            let id = Uuid::new_v4();
            let config = GlobalConfig {
                root_home: savior_file_path.display().to_string(),
                code: env::var("SAVIOR_ORG_CODE").unwrap_or_default(),
                name: env::var("SAVIOR_ORG_NAME").unwrap_or_default(),
                place: env::var("SAVIOR_ORG_PLACE").unwrap_or_default(),
                fep_address: env::var("SAVIOR_FEP_ADDRESS").unwrap_or_default(),
                client_id: id.to_string(),
                client_name: "".to_string(),
            };
            config.save()?;
            Ok(config)
        }
    }

    fn save(&self) -> Result<()> {
        let settings_path = PathBuf::from(&self.root_home).join("settings.json");
        let json_value = serde_json::to_value(self)?;
        write_json_file(&settings_path, &json_value)?;
        Ok(())
    }
}

fn get_app_data_dir() -> Result<PathBuf> {
    let app_dirs = AppDirs::new(Some("Savior"), false).unwrap();
    let home = app_dirs.data_dir;
    Ok(home)
}
