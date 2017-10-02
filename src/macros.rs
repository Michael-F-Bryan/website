#[macro_export]
macro_rules! backtrace {
    ($result:expr) => {
        match $result {
            Err(e) => {
                eprintln!("Error: {}", e);

                for cause in e.iter().skip(1) {
                    eprintln!("\tCaused by: {}", cause);
                }

                ::std::process::exit(1)
            }
            Ok(v) => v,
        }
    };
}
