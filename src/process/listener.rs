use crate::{channel::redis_tool::AppState, config::CHANNEL_NOTIFICATION};
use anyhow::Result;
use redis::{ControlFlow, PubSubCommands};
use std::{sync::Arc, thread};

pub fn listener_process(state: &impl AppState) -> Result<thread::JoinHandle<Result<()>>> {
    let client = Arc::clone(state.client());
    Ok(thread::spawn(move || -> Result<()> {
        let conn = client.get_connection();
        match conn {
            Ok(mut conn) => {
                let _: () = conn.subscribe(CHANNEL_NOTIFICATION, |msg| {
                    let payload = msg.get_payload::<String>();
                    match payload {
                        Ok(payload) => {
                            println!("payload: {:?}", payload);
                            ControlFlow::Continue
                        }
                        Err(e) => {
                            println!("get payload error: {:?}", e);
                            ControlFlow::Break(())
                        }
                    }
                })?;
            }
            Err(e) => {
                println!("redis connection error: {:?}", e);
            }
        }
        Ok(())
    }))
}
