use std::sync::{Once, ONCE_INIT};
use env_logger;
use website::errors::*;
use rand::{self, Rng};

pub fn init_logging() {
    static THING: Once = ONCE_INIT;
    THING.call_once(|| {
        env_logger::init().ok();
    });
}

pub struct Docker {
    image_hash: String,
    port: u16,
}

impl Docker {
    pub fn new() -> Result<Docker> {
        init_logging();

        let mut rng = rand::thread_rng();
        let port: u16 = rng.gen_range(10_000, u16::max_value());

        let output = cmd!(
            "docker",
            "run",
            "--detach",
            "-p",
            format!("{}:27017", port),
            "mongo"
        )?;

        let image_hash = String::from_utf8_lossy(&output.stdout).trim().to_string();

        Ok(Docker { image_hash, port })
    }

    pub fn close(&mut self) -> Result<()> {
        cmd!("docker", "kill", &self.image_hash)?;
        cmd!("docker", "rm", &self.image_hash)?;

        Ok(())
    }

    pub fn database_url(&self) -> String {
        format!("mongodb://localhost:{}", self.port)
    }
}

impl Drop for Docker {
    fn drop(&mut self) {
        self.close().unwrap();
    }
}
