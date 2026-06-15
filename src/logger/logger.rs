use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::append::rolling_file::policy::compound::roll::fixed_window::FixedWindowRoller;
use log4rs::append::rolling_file::policy::compound::trigger::size::SizeTrigger;
use log4rs::append::rolling_file::policy::compound::CompoundPolicy;
use log4rs::append::rolling_file::RollingFileAppender;
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::Config;
use tracing_appender::rolling::{self};
use tracing_subscriber::{
    filter::LevelFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt, Layer,
};

use crate::configure::LogLevel;

pub fn init_log() {
    let window_size = 3; // log0, log1, log2
    let fixed_window_roller = FixedWindowRoller::builder()
        .build("logs/oss_pipe_{}.log", window_size)
        .unwrap();
    let size_limit = 100 * 1024 * 1024; // 100M as max log file size to roll
    let size_trigger = SizeTrigger::new(size_limit);
    let compound_policy =
        CompoundPolicy::new(Box::new(size_trigger), Box::new(fixed_window_roller));
    let rolling_file = RollingFileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{l} {d} {t} {f} {L} - {m}{n}",
        )))
        .build("logs/oss_pipe.log", Box::new(compound_policy))
        .unwrap();

    let file_out = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{l} {d} {t} {f} {L} - {m}{n}",
        )))
        .build("logs/oss_pipe.log")
        .unwrap();

    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{l} {d} {t} {f} {L} - {m}{n}",
        )))
        .build();

    let config = Config::builder()
        .appender(Appender::builder().build("rolling_file", Box::new(rolling_file)))
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("file_out", Box::new(file_out)))
        .build(
            Root::builder()
                .appender("stdout")
                .appender("rolling_file")
                .build(log::LevelFilter::Info),
        )
        .unwrap();

    let _ = log4rs::init_config(config).unwrap();
}

pub fn tracing_init(log_level: &LogLevel) {
    let level = match log_level {
        LogLevel::TRACE => LevelFilter::TRACE,
        LogLevel::DEBUG => LevelFilter::DEBUG,
        LogLevel::INFO => LevelFilter::INFO,
        LogLevel::ERROR => LevelFilter::ERROR,
        LogLevel::WARN => LevelFilter::WARN,
        LogLevel::OFF => LevelFilter::OFF,
    };

    // 格式化输出层，并且输出到终端。
    let formatting_layer = fmt::layer()
        .with_file(true)
        .with_line_number(true)
        .with_writer(std::io::stdout)
        // .json()
        .pretty()
        .with_filter(level)
        .boxed();

    // 文件输出层
    let file_appender = rolling::daily("logs/", "oss_pipe.log");
    let file_layer = fmt::layer()
        .with_ansi(false)
        .with_file(true)
        .with_line_number(true)
        .with_writer(file_appender)
        .with_filter(level)
        .boxed();

    let registry = tracing_subscriber::registry()
        .with(file_layer)
        .with(formatting_layer);

    registry.init()
}
