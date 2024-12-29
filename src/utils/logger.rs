use anyhow::Result;
use flexi_logger::{Cleanup, Criterion, FileSpec, Logger, Naming};
use std::env;
use std::fs;
use std::path::Path;
use std::sync::Once;

static LOGGER_INIT: Once = Once::new();

pub fn init_logger() -> Result<()> {
    // 使用 Once 确保只初始化一次
    LOGGER_INIT.call_once(|| {
        // 读取日志目录路径配置，默认为 "logs"
        let log_dir = env::var("LOG_DIR").unwrap_or_else(|_| "logs".to_string());
        let log_dir_path = Path::new(&log_dir);

        // 确保日志目录存在
        fs::create_dir_all(log_dir_path).expect("Failed to create log directory");

        // 读取日志级别配置，默认为 "info"
        let log_level = env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());

        // 配置日志文件
        let log_file_spec = FileSpec::default()
            .directory(&log_dir)
            .basename("app")
            .suffix("log")
            .suppress_timestamp();

        // 配置日志滚动策略
        let criterion = Criterion::Size(10 * 1024 * 1024); // 10 MB
        let naming = Naming::Timestamps;
        let cleanup = Cleanup::KeepLogFiles(7); // 保留最近7天的日志文件

        // 初始化日志记录器
        Logger::try_with_str(&log_level)
            .expect("Invalid log level")
            .log_to_file(log_file_spec)
            .rotate(criterion, naming, cleanup)
            .format(|writer, datetime, record| {
                write!(
                    writer,
                    "{} [{}] - {}",
                    datetime.format("%Y-%m-%d %H:%M:%S"),
                    record.level(),
                    record.args()
                )
            })
            .start()
            .expect("Failed to initialize logger");
    });

    Ok(())
}
