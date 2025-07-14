use colored::{ColoredString, Colorize};
use std::sync::OnceLock;
use std::time::SystemTime;

use fern::colors::{Color, ColoredLevelConfig};

pub struct Logger {
    pub is_init: bool,
}

pub enum LogColor {
    TRACE,
    DEBUG,
    INFO,
    WARN,
    ERROR,
}

impl LogColor {
    fn value(&self) -> (u8, u8, u8) {
        match self {
            LogColor::TRACE => (145, 145, 145),
            LogColor::DEBUG => (255, 255, 255),
            LogColor::INFO => (0, 255, 47),
            LogColor::WARN => (232, 255, 0),
            LogColor::ERROR => (173, 70, 61),
        }
    }
}

pub fn color_text(s: &String, log_color: LogColor) -> ColoredString {
    let (r, g, b) = log_color.value();
    s.truecolor(r, g, b)
}

pub static LOGGER_INSTANCE: OnceLock<Logger> = OnceLock::new();

pub fn init() -> Option<&'static Logger> {
    let inst = LOGGER_INSTANCE.get_or_init(|| match setup_logger() {
        Ok(..) => Logger { is_init: true },
        Err(e) => {
            println!("{}", e);
            Logger { is_init: false }
        }
    });

    if inst.is_init {
        return Some(inst);
    } else {
        return None;
    }
}

fn setup_logger() -> Result<(), fern::InitError> {
    let colors = ColoredLevelConfig::new()
        .trace(Color::BrightBlack) // Grey for trace
        .debug(Color::White)
        .info(Color::Green)
        .warn(Color::Yellow)
        .error(Color::Red);

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                colors.color(record.level()),
                record.target(),
                message,
            ))
        })
        .level(log::LevelFilter::max())
        .level_for("wgpu", log::LevelFilter::Warn)
        .level_for("wgpu_core", log::LevelFilter::Warn)
        .level_for("wgpu_hal", log::LevelFilter::Warn)
        .level_for("naga", log::LevelFilter::Warn)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;

    Ok(())
}

#[macro_export]
macro_rules! trace_core{
    ($format_string:expr, $($arg:tt)*) => {
        {
            use $crate::sf_log::{LogColor, color_text};
            use log::{trace};
            let formatted_message = format!($format_string, $($arg)*);
            let s = color_text(&format!("CORE {}", formatted_message), LogColor::TRACE);
            trace!("{}", s)
        }
    };
    ($arg:tt) => {
        {
            use $crate::sf_log::{LogColor, color_text};
            use log::{trace};
            let s = color_text(&format!("CORE {}", $arg), LogColor::TRACE);
            trace!("{}", s)
        }
    };
}

#[macro_export]
macro_rules! debug_core{
    ($format_string:expr, $($arg:tt)*) => {
        {
            use $crate::sf_log::{LogColor, color_text};
            use log::{debug};
            let formatted_message = format!($format_string, $($arg)*);
            let s = color_text(&format!("CORE {}", formatted_message), LogColor::DEBUG);
            debug!("{}", s)
        }
    };
    ($arg:tt) => {
        {
            use $crate::sf_log::{LogColor, color_text};
            use log::{debug};
            let s = color_text(&format!("CORE {}", $arg), LogColor::DEBUG);
            debug!("{}", s)
        }
    };
}

#[macro_export]
macro_rules! info_core {
    ($format_string:expr, $($arg:tt)*) => {
        {
            use $crate::sf_log::{LogColor, color_text};
            use log::{info};
            let formatted_message = format!($format_string, $($arg)*);
            let s = color_text(&format!("CORE {}", formatted_message), LogColor::INFO);
            info!("{}", s)
        }
    };
    ($arg:tt) => {
        {
            use $crate::sf_log::{LogColor, color_text};
            use log::{info};
            let s = color_text(&format!("CORE {}", $arg), LogColor::INFO);
            info!("{}", s)
        }
    };
}

#[macro_export]
macro_rules! warn_core {
    ($format_string:expr, $($arg:tt)*) => {
        {
            use $crate::sf_log::{LogColor, color_text};
            use log::{warn};
            let formatted_message = format!($format_string, $($arg)*);
            let s = color_text(&format!("CORE {}", formatted_message), LogColor::WARN);
            warn!("{}", s)
        }
    };
    ($arg:tt) => {
        {
            use $crate::sf_log::{LogColor, color_text};
            use log::{warn};
            let s = color_text(&format!("CORE {}", $arg), LogColor::WARN);
            warn!("{}", s)
        }
    };
}

#[macro_export]
macro_rules! error_core {
    ($format_string:expr, $($arg:tt)*) => {
        {
            use $crate::sf_log::{LogColor, color_text};
            use log::{error};
            let formatted_message = format!($format_string, $($arg)*);
            let s = color_text(&format!("CORE {}", formatted_message), LogColor::ERROR);
            error!("{}", s)
        }
    };
    ($arg:tt) => {
        {
            use $crate::sf_log::{LogColor, color_text};
            use log::{error};
            let s = color_text(&format!("CORE {}", $arg), LogColor::ERROR);
            error!("{}", s)
        }
    };
}

//CLIENT LOG MACROS
#[macro_export]
macro_rules! trace_client {
    ($format_string:expr, $($arg:tt)*) => {
        {
            use $crate::sf_log::{LogColor, color_text};
            use log::{trace};
            let formatted_message = format!($format_string, $($arg)*);
            let s = color_text(&format!("CORE {}", formatted_message), LogColor::TRACE);
            trace!("{}", s)
        }
    };
    ($arg:tt) => {
        {
            use $crate::sf_log::{LogColor, color_text};
            use log::{trace};
            let s = color_text(&format!("CORE {}", $arg), LogColor::TRACE);
            trace!("{}", s)
        }
    };
}

#[macro_export]
macro_rules! debug_client {
    ($format_string:expr, $($arg:tt)*) => {
        {
            use $crate::sf_log::{LogColor, color_text};
            use log::{debug};
            let formatted_message = format!($format_string, $($arg)*);
            let s = color_text(&format!("CORE {}", formatted_message), LogColor::DEBUG);
            debug!("{}", s)
        }
    };
    ($arg:tt) => {
        {
            use $crate::sf_log::{LogColor, color_text};
            use log::{debug};
            let s = color_text(&format!("CORE {}", $arg), LogColor::DEBUG);
            debug!("{}", s)
        }
    };
}

#[macro_export]
macro_rules! info_client {
    ($format_string:expr, $($arg:tt)*) => {
        {
            use $crate::sf_log::{LogColor, color_text};
            use log::{info};
            let formatted_message = format!($format_string, $($arg)*);
            let s = color_text(&format!("CORE {}", formatted_message), LogColor::INFO);
            info!("{}", s)
        }
    };
    ($arg:tt) => {
        {
            use $crate::sf_log::{LogColor, color_text};
            use log::{info};
            let s = color_text(&format!("CORE {}", $arg), LogColor::INFO);
            info!("{}", s)
        }
    };
}

#[macro_export]
macro_rules! warn_client {
    ($format_string:expr, $($arg:tt)*) => {
        {
            use $crate::sf_log::{LogColor, color_text};
            use log::{warn};
            let formatted_message = format!($format_string, $($arg)*);
            let s = color_text(&format!("CORE {}", formatted_message), LogColor::WARN);
            warn!("{}", s)
        }
    };
    ($arg:tt) => {
        {
            use $crate::sf_log::{LogColor, color_text};
            use log::{warn};
            let s = color_text(&format!("CORE {}", $arg), LogColor::WARN);
            warn!("{}", s)
        }
    };
}

#[macro_export]
macro_rules! error_client {
    ($format_string:expr, $($arg:tt)*) => {
        {
            use $crate::sf_log::{LogColor, color_text};
            use log::{error};
            let formatted_message = format!($format_string, $($arg)*);
            let s = color_text(&format!("CORE {}", formatted_message), LogColor::ERROR);
            error!("{}", s)
        }
    };
    ($arg:tt) => {
        {
            use $crate::sf_log::{LogColor, color_text};
            use log::{error};
            let s = color_text(&format!("CORE {}", $arg), LogColor::ERROR);
            error!("{}", s)
        }
    };
}
