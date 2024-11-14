use anyhow::Result;
use savior_daemon::{
    channel::redis_tool::Ctx,
    process::{listener::listener_process, register::register_process},
};

fn main() -> Result<()> {
    let ctx = Ctx::new();
    let register_handle = register_process(&ctx)?;
    let listener_handle = listener_process(&ctx)?;
    let _ = register_handle.join().unwrap();
    let _ = listener_handle.join().unwrap();
    Ok(())
}
