use anyhow::Result;
use platform_dirs::AppDirs;
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

    // 获取当前用户的AppData/Roaming
    let app_dirs = AppDirs::new(Some("Savior"), false).unwrap();
    let home = app_dirs.data_dir;

    println!("home: {}", home.display());

    Ok(())
}

// #[cfg(windows)]
// fn get_app_data_dir() -> Result<PathBuf> {
//     let home = env::var("APPDATA")?;
//     let path = Path::new(&home).join("Savior");
//     println!("path: {}", path.display());
//     Ok(path)
// }

// #[cfg(not(windows))]
// fn get_app_data_dir() -> Result<PathBuf> {
//     let home = env::var("HOME")?;
//     let path = Path::new(&home).join(".Savior");
//     println!("path: {}", path.display());
//     Ok(path)
// }
