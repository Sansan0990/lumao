use log::LevelFilter;
use log4rs::{
    append::{
        console::ConsoleAppender,
        rolling_file::{
            policy::compound::{
                roll::fixed_window::FixedWindowRoller, trigger::size::SizeTrigger, CompoundPolicy,
            },
            RollingFileAppender,
        },
    },
    encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter,
};
use std::{cmp::max, fs, path::Path};

const LOG_FORMAT_ENCODE: &str = "[{d(%Y-%m-%dT%H:%M:%S.%3fZ)(utc)}] [{l}] - {m}{n}";

// 初始化文件输出和标准输出的日志系统, 若指定目录不存在, 就创建此目录
pub fn init_log_file_stdout(
    path: &Path,
    level_stdout: LevelFilter,
    level_file: LevelFilter,
) -> Result<(), String> {
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(LOG_FORMAT_ENCODE)))
        .build();

    if let Err(e) = fs::create_dir_all(path) {
        return Err(e.to_string());
    }
    let path_str = path
        .to_str()
        .ok_or(format!("path is non-UTF-8 strings, {:?}", path))?;
    let path_string = format!("{}/log.{{}}.txt", path_str);

    let file = RollingFileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(LOG_FORMAT_ENCODE)))
        .build(
            path.join("log.txt"),
            Box::new(CompoundPolicy::new(
                Box::new(SizeTrigger::new(10 * 1024 * 1024)),
                Box::new(
                    FixedWindowRoller::builder()
                        .build(path_string.as_str(), 8)
                        .unwrap(),
                ),
            )),
        );
    let file = match file {
        Ok(f) => f,
        Err(e) => {
            return Err(format!("log init fault, {}", e));
        }
    };

    let config = log4rs::config::Config::builder()
        .appender(
            log4rs::config::Appender::builder()
                .filter(Box::new(ThresholdFilter::new(level_stdout)))
                .build("stdout", Box::new(stdout)),
        )
        .appender(
            log4rs::config::Appender::builder()
                .filter(Box::new(ThresholdFilter::new(level_file)))
                .build("file", Box::new(file)),
        )
        .build(
            log4rs::config::Root::builder()
                .appender("stdout")
                .appender("file")
                .build(max(level_file, level_stdout)),
        )
        .unwrap();

    if let Err(e) = log4rs::init_config(config) {
        Err(format!("{}", e))
    } else {
        Ok(())
    }
}

// 初始化标准输出的日志系统
pub fn init_log_stdout(level: LevelFilter) -> Result<(), String> {
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(LOG_FORMAT_ENCODE)))
        .build();

    let config = log4rs::config::Config::builder()
        .appender(
            log4rs::config::Appender::builder()
                .filter(Box::new(ThresholdFilter::new(level)))
                .build("stdout", Box::new(stdout)),
        )
        .build(
            log4rs::config::Root::builder()
                .appender("stdout")
                .build(level),
        )
        .unwrap();

    if let Err(e) = log4rs::init_config(config) {
        Err(format!("{}", e))
    } else {
        Ok(())
    }
}
