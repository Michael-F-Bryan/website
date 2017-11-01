use std::env;
use env_logger::{LogBuilder, LogTarget};
use chrono::Local;
use clap::{App, Arg};


pub fn app() -> App<'static, 'static> {
    app_from_crate!().arg(
        Arg::with_name("verbosity")
            .short("v")
            .long("verbose")
            .multiple(true)
            .help("Enable verbose output (repeat for more verbosity)"),
    )
}

pub fn initialize_logging(verbosity: u64) {
    let mut lb = LogBuilder::new();
    lb.format(|record| {
        let loc = record.location();

        format!(
            "{} [{:5}] ({}#{}): {}",
            Local::now().format("%Y-%m-%d %H:%M:%S"),
            record.level(),
            loc.module_path(),
            loc.line(),
            record.args()
        )
    });

    if let Ok(var) = env::var("RUST_LOG") {
        lb.parse(&var);
    }

    lb.target(LogTarget::Stderr).init().ok();
}
