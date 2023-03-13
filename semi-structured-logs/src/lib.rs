#![allow(unused)]

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}

pub fn log(level: LogLevel, message: &str) -> String {
    match level {
        LogLevel::Info => info(message),
        LogLevel::Warning => warn(message),
        LogLevel::Error => error(message),
    }
}

pub fn info(message: &str) -> String {
    format!("[INFO]: {}", message).to_string()
}

pub fn warn(message: &str) -> String {
    format!("[WARNING]: {}", message).to_string()
}

pub fn error(message: &str) -> String {
    format!("[ERROR]: {}", message).to_string()
}
