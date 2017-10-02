/// Unwrap some `Result<T>`, printing a backtrace and exiting if there was an error.
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

/// Run a subcommand and check the output, logging any errors (returns a `Result<Output>`).
#[macro_export]
macro_rules! cmd {
    ($name:tt, $($arg:expr),*) => {{
        let output = ::std::process::Command::new($name)
            $(
            .arg($arg)

            )*
            .output()?;

        let ret: $crate::errors::Result<::std::process::Output> = if !output.status.success() {
            let command = stringify!($name, $( " ", $arg ),*);

            warn!("{:?} failed with return code {:?}", command, output.status.code());
            if !output.stdout.is_empty() {
                warn!("Stdout: {}", String::from_utf8_lossy(&output.stdout));
            }
            if !output.stdout.is_empty() {
                warn!("Stderr: {}", String::from_utf8_lossy(&output.stderr));
            }
            Err(format!("Command failed, {:?}", output.status.code()).into())
        } else {
            Ok(output)
        };

        ret
    }};
}
