use std::fmt::Display;
mod controller;

use controller::{
    init_controller, is_descriptor_valid, is_xpub_valid, Controller, Decrypt, Encrypt,
};

#[cxx::bridge]
#[allow(clippy::unnecessary_box_returns)]
pub mod bed {

    #[derive(Debug, Clone, Copy)]
    pub enum LogLevel {
        Off,
        Error,
        Warn,
        Info,
        Debug,
        Trace,
    }
    extern "Rust" {
        fn init_rust_logger(level: LogLevel);
        fn log_info(s: String);
        fn log_error(s: String);
        fn log_debug(s: String);
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Notification {
        None,
        UpdateEncrypt,
        UpdateDecrypt,
    }

    extern "Rust" {
        fn notif_to_string(notif: Notification) -> String;
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Mode {
        Encrypt,
        Decrypt,
    }

    #[rust_name = Screen]
    #[derive(Debug, Clone)]
    pub struct RustScreen {
        keys: Vec<String>,
        valid: Vec<bool>,
        selected: Vec<bool>,
        descriptor: String,
        descriptor_valid: bool,
        ciphertext: Vec<u8>,
        btn_enabled: bool,
        mode: Mode,
    }

    extern "Rust" {
        type Encrypt;
        fn edit_xpub(&mut self, index: usize, xpub: String);
        fn add_xpub(&mut self);
        fn remove_xpub(&mut self, index: usize);
        fn set_selected(&mut self, index: usize, selected: bool);
        fn reset(&mut self);
        fn set_descriptor(&mut self, descriptor: String);
        fn try_encrypt(&mut self);
        fn save(&self, path: String);
    }

    extern "Rust" {
        type Decrypt;
        fn edit_xpub(&mut self, index: usize, xpub: String);
        fn add_xpub(&mut self);
        fn remove_xpub(&mut self, index: usize);
        fn set_selected(&mut self, index: usize, selected: bool);
        fn reset(&mut self);
        fn try_decrypt(&mut self);
        fn save(&self, path: String);
    }

    extern "Rust" {
        #[rust_name = Controller]
        type RustController;
        fn encrypt(&mut self) -> &mut Encrypt;
        fn decrypt(&mut self) -> &mut Decrypt;
        fn poll(&self) -> Notification;
        fn error(&self) -> String;
        fn encrypt_screen(&mut self) -> Screen;
        fn decrypt_screen(&mut self) -> Screen;
        fn drag_n_drop(&mut self, file_path: String, mode: Mode);
    }

    extern "Rust" {
        fn init_controller() -> Box<Controller>;
        fn is_xpub_valid(xpub: &str) -> bool;
        fn is_descriptor_valid(xpub: &str) -> bool;
    }
}

use bed::{LogLevel, Mode, Notification, Screen};

#[allow(clippy::derivable_impls)]
impl Default for Screen {
    fn default() -> Self {
        Self {
            keys: Default::default(),
            valid: Default::default(),
            selected: Default::default(),
            descriptor: Default::default(),
            descriptor_valid: false,
            ciphertext: Default::default(),
            btn_enabled: Default::default(),
            mode: Default::default(),
        }
    }
}

impl Default for Mode {
    fn default() -> Self {
        Self::Decrypt
    }
}

impl Display for Notification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Notification::{self:?}")
    }
}

impl Default for Notification {
    fn default() -> Self {
        Self::None
    }
}

impl From<Option<Self>> for Notification {
    fn from(value: Option<Self>) -> Self {
        value.unwrap_or_default()
    }
}

impl From<LogLevel> for log::LevelFilter {
    fn from(value: LogLevel) -> Self {
        match value {
            LogLevel::Off => Self::Off,
            LogLevel::Error => Self::Error,
            LogLevel::Warn => Self::Warn,
            LogLevel::Info => Self::Info,
            LogLevel::Debug => Self::Debug,
            LogLevel::Trace => Self::Trace,
            _ => unreachable!(),
        }
    }
}

impl From<log::LevelFilter> for LogLevel {
    fn from(value: log::LevelFilter) -> Self {
        match value {
            log::LevelFilter::Off => Self::Off,
            log::LevelFilter::Error => Self::Error,
            log::LevelFilter::Warn => Self::Warn,
            log::LevelFilter::Info => Self::Info,
            log::LevelFilter::Debug => Self::Debug,
            log::LevelFilter::Trace => Self::Trace,
        }
    }
}

pub fn init_rust_logger(level: LogLevel) {
    let level = level.into();
    env_logger::builder().filter_level(level).init();
    log::info!("init_rust_logger()");
}

#[allow(clippy::needless_pass_by_value)]
pub fn log_info(s: String) {
    log::info!("{s}");
}
#[allow(clippy::needless_pass_by_value)]
pub fn log_error(s: String) {
    log::error!("{s}");
}
#[allow(clippy::needless_pass_by_value)]
pub fn log_debug(s: String) {
    log::debug!("{s}");
}

#[must_use]
pub fn notif_to_string(notif: Notification) -> String {
    notif.to_string()
}
