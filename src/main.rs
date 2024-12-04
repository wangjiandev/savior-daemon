use anyhow::Result;
use savior_daemon::{
    channel::redis_tool::Ctx,
    process::{listener::listener_process, register::register_process},
};
use std::{
    env,
    path::{Path, PathBuf},
};

fn main() -> Result<()> {
    // let ctx = Ctx::new();
    // let register_handle = register_process(&ctx)?;
    // let listener_handle = listener_process(&ctx)?;
    // let _ = register_handle.join().unwrap();
    // let _ = listener_handle.join().unwrap();

    // 获取当前用户的AppData/Roaming
    get_app_data_dir()?;

    // #[allow(deprecated)]
    // let home = home_dir().unwrap();
    // println!("home_dir is {:?}", home);
    Ok(())
}

#[cfg(windows)]
fn get_app_data_dir() -> Result<PathBuf> {
    let home = env::var("APPDATA")?;
    let path = Path::new(&home).join("Savior");
    println!("path: {}", path.display());
    Ok(path)
}

#[cfg(not(windows))]
fn get_app_data_dir() -> Result<PathBuf> {
    let home = env::var("HOME")?;
    let path = Path::new(&home).join(".Savior");
    println!("path: {}", path.display());
    Ok(path)
}
