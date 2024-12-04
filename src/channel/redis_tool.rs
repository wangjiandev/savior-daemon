use crate::config::GlobalConfig;
use redis::Client;
use std::sync::Arc;

pub trait AppState {
    fn client(&self) -> &Arc<Client>;
}

#[derive(Debug)]
pub struct Ctx {
    pub client: Arc<Client>,
    pub config: GlobalConfig,
}

impl Ctx {
    pub fn new() -> Ctx {
        let config = GlobalConfig::try_load().expect("load config error");
        let redis_password = String::from("");
        let redis_conn_url = format!("redis://:{}@{}", redis_password, &config.fep_address);
        let client = Client::open(redis_conn_url).expect("redis client open error");
        Ctx {
            client: Arc::new(client),
            config,
        }
    }
}

impl Default for Ctx {
    fn default() -> Self {
        Self::new()
    }
}

impl AppState for Ctx {
    fn client(&self) -> &Arc<Client> {
        &self.client
    }
}
