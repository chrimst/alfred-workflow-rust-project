use log::LevelFilter;


use log4rs::append::rolling_file::policy::compound::roll::fixed_window::{
    FixedWindowRoller,
};
use log4rs::append::rolling_file::policy::compound::trigger::size::SizeTrigger;
use log4rs::append::rolling_file::policy::compound::{CompoundPolicy};
use log4rs::append::rolling_file::RollingFileAppender;
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::Config;


pub struct Logger {}


impl Logger {
    pub fn init() {
        let appender_name = "app_log_appender";
        let log_path = std::env::var("HOME").unwrap();
        let log_name = format!("{}/.alfred/ali_workflow.log", log_path);
        let archive_name = log_name.as_str().to_owned() + "{}";
        let config = Config::builder()
            .appender(
                Appender::builder().build(
                    appender_name,
                    Box::new(
                        RollingFileAppender::builder()
                            .append(true)
                            .encoder(Box::new(PatternEncoder::new(
                                "{d(%+)(local)} {h({l})} [{f}-> line:{L}]  {m}-{n}",
                            )))
                            .build(
                                log_name,
                                Box::new(CompoundPolicy::new(
                                    Box::new(SizeTrigger::new(1023)),
                                    Box::new(
                                        FixedWindowRoller::builder()
                                            .build(&archive_name, 10)
                                            .unwrap(),
                                    ),
                                )),
                            )
                            .unwrap(),
                    ),
                ),
            )
            .build(
                Root::builder()
                    .appender(appender_name)
                    .build(LevelFilter::Trace),
            )
            .unwrap();

        log4rs::init_config(config).unwrap();
    }
}