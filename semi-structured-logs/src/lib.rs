/// various log levels
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
    Debug,
}
/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    match level{
        LogLevel::Info => return info(message),
        LogLevel::Warning => return warn(message),
        LogLevel::Error => return error(message),
        LogLevel::Debug => return debug(message),
    }
}
pub fn info(message: &str) -> String {
    "[INFO]: ".to_owned() + message
}
pub fn warn(message: &str) -> String {
    "[WARNING]: ".to_owned() + message
}
pub fn error(message: &str) -> String {
    "[ERROR]: ".to_owned() + message
}
pub fn debug(message: &str) -> String {
    "[DEBUG]: ".to_owned() + message
}
