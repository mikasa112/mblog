use tklog::{Format, ASYNC_LOG, LEVEL};

pub struct Logger {}
impl Logger {
    pub async fn init() -> Logger {
        ASYNC_LOG
            //兼容官方日志
            .uselog()
            //控制台打印
            .set_console(true)
            //日志级别
            .set_level(LEVEL::Info)
            .set_format(
                Format::LevelFlag
                    | Format::Date
                    | Format::Time
                    | Format::Microseconds
                    | Format::ShortFileName,
            )
            .set_formatter("{time} {level} {file}: {message}\n")
            .set_cutmode_by_mixed(
                //文件名
                "./logs/mblog.log",
                //滚动字节数，这里是521M
                1 << 29,
                //同时按天滚动
                tklog::MODE::DAY,
                //最多保存30
                30,
                //开启压缩
                true,
            )
            .await;
        Logger {}
    }
}
