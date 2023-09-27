pub mod cli;
pub mod client;
pub mod service;

pub type Result<T> = std::result::Result<T, &'static str>;

const DEFAULT_CFG: &str = "/etc/kickable/config";
