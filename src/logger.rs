use std::fs;
use std::fs::File;
use std::io::Write;
use chrono::{ Local };
use zip::write::{FileOptions, ZipWriter};

enum Type {
    INFO,
    WARN,
    ERROR
}

pub fn info(message_str: &str) {

    let message = message_str.to_string();
    log_message(message, Type::INFO);

}

pub fn warn(message_str: &str) {

    let message = message_str.to_string();
    log_message(message, Type::WARN);

}

pub fn error(message_str: &str) {

    let message = message_str.to_string();
    log_message(message, Type::ERROR);

}

fn log_message(message: String, log_type: Type) -> std::io::Result<()> {

    // Logging to console

    let now = Local::now();
    let time = now.format("%Y:%m:%d %H:%M").to_string();
    let color = get_color(&log_type);

    println!("{}[{}] {}\x1b[0m", color, time, message);

    // Logging to file

    let log_message = format!("[{}] [{}] {}", time, get_type(&log_type), message);
    let root = dirs::home_dir().expect("Could not find home directory");
    let logs_path = root.join(".ferris/launcher/logs");
    let log_path = logs_path.join("latest.log");
    let current_log = fs::read_to_string(&log_path)?;

    fs::create_dir_all(&logs_path)?;

    fs::write(&log_path, format!("{}\n{}", current_log, log_message))?;

    Ok(())

}

pub fn zip_latest() -> std::io::Result<()> {

    let now = Local::now();

    let root = dirs::home_dir().expect("Could not find home directory");
    let logs_path = root.join(".ferris/launcher/logs");
    let log_path = logs_path.join("latest.log");

    fs::create_dir_all(&logs_path)?;

    if (log_path.exists()) {

        let mut buffer = Vec::new();

        let log_date = now.format("%Y-%m-%d_%H-%M").to_string();
        let zip_path = logs_path.join(format!("{}.zip", log_date));
        let zip_file = File::create(&zip_path)?;
        let mut zip = ZipWriter::new(zip_file);

        let options: FileOptions<()> = FileOptions::default().compression_method(zip::CompressionMethod::Xz);

        zip.start_file(format!("{}.log", log_date), options)?;
        zip.write_all(&buffer);

        zip.finish()?;

        fs::remove_file(&log_path)?;

    }

    File::create(&log_path)?;

    Ok(())

}

fn get_color(log_type: &Type) -> String {

    let value;

    match log_type {
        Type::INFO => value = "\x1b[0m",
        Type::WARN => value = "\x1b[33m",
        Type::ERROR => value = "\x1b[31m",
        _ => value = "\x1b[0m",
    }

    value.to_string()

}

fn get_type(log_type: &Type) -> String {

    match log_type {
        Type::INFO => String::from("INFO"),
        Type::WARN => String::from("WARN"),
        Type::ERROR => String::from("ERROR"),
        _ => String::from("?"),
    }

}