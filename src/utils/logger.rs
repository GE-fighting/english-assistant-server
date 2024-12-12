use anyhow::Result;
use chrono::Local;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::sync::Once;

static LOGGER_INIT: Once = Once::new();

pub fn init_logger() -> Result<()> {
    // 使用 Once 确保只初始化一次
    LOGGER_INIT.call_once(|| {
        // 确保日志目录存在
        let log_dir = Path::new("logs");
        fs::create_dir_all(log_dir).expect("Failed to create log directory");

        // 生成按日期命名的日志文件
        let log_filename = format!("logs/{}.log", Local::now().format("%Y-%m-%d"));

        // 配置控制台日志
        env_logger::Builder::from_default_env()
            .format(|buf, record| {
                use std::io::Write;
                writeln!(
                    buf,
                    "{} [{}] - {}",
                    Local::now().format("%Y-%m-%d %H:%M:%S"),
                    record.level(),
                    record.args()
                )
            })
            .target(env_logger::Target::Stdout)
            .write_style(env_logger::WriteStyle::Always)
            .parse_filters("info")
            .try_init()
            .unwrap_or_else(|_| eprintln!("Logger already initialized"));

        // 可选：文件日志
        if let Err(e) = file_logger(&log_filename) {
            eprintln!("Failed to set up file logger: {}", e);
        }
    });

    Ok(())
}

fn file_logger(log_path: &str) -> Result<()> {
    use log::{LevelFilter, Metadata, Record};
    use std::sync::Mutex;

    struct FileLogger {
        file: Mutex<fs::File>,
    }

    impl log::Log for FileLogger {
        fn enabled(&self, metadata: &Metadata) -> bool {
            metadata.level() <= log::Level::Info
        }

        fn log(&self, record: &Record) {
            if self.enabled(record.metadata()) {
                let formatted_log = format!(
                    "{} [{}] - {}\n",
                    Local::now().format("%Y-%m-%d %H:%M:%S"),
                    record.level(),
                    record.args()
                );

                let mut file = self.file.lock().unwrap();
                file.write_all(formatted_log.as_bytes())
                    .map_err(|e| {
                        eprintln!("Failed to write log to file: {}", e);
                    })
                    .ok();
            }
        }

        fn flush(&self) {
            if let Ok(mut file) = self.file.lock() {
                file.flush()
                    .map_err(|e| {
                        eprintln!("Failed to flush log file: {}", e);
                    })
                    .ok();
            }
        }
    }

    let file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(log_path)?;

    let logger = FileLogger {
        file: Mutex::new(file),
    };

    log::set_boxed_logger(Box::new(logger))?;
    log::set_max_level(LevelFilter::Info);

    Ok(())
}
