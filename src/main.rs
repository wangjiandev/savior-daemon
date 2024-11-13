use anyhow::Result;
use savior_daemon::process::register::register_process;
// use if_addrs::get_if_addrs;
// use std::env;

fn main() -> Result<()> {
    let info = register_process()?;
    println!("info: {:?}", info);

    // for (key, value) in env::vars() {
    //     println!("{}: {}", key, value);
    // }

    // println!(
    //     "RUSTUP_HOME: {}",
    //     env::var("RUSTUP_HOME").unwrap_or("adawdawd".to_string())
    // );

    // let data = read_json_file("data.json").unwrap_or_else(|e| {
    //     println!("读取json文件失败: {}", e);
    //     serde_json::Value::default()
    // });
    // if data.is_null() {
    //     return Ok(());
    // }
    // println!("data: {}", data);

    // loop {
    //     println!("守护进程运行中...");
    //     thread::sleep(Duration::from_secs(5));
    // }

    Ok(())
}
