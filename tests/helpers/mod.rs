use std::process::Command;
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

        let output = Command::new("docker")
            .arg("run")
            .arg("--detach")
            .arg("-p")
            .arg(format!("{}:27017", port))
            .arg("mongo")
            .output()?;

        if !output.status.success() {
            bail!("Starting docker image failed");
        }

        let image_hash = String::from_utf8_lossy(&output.stdout).trim().to_string();

        Ok(Docker { image_hash, port })
    }

    pub fn close(&mut self) -> Result<()> {
        let output = Command::new("docker")
            .arg("rm")
            .arg("--force")
            .arg(&self.image_hash)
            .output()?;


        if !output.status.success() {
            bail!(
                "Trying to kill the docker container gave an erroneous return code, {:?}",
                output.status.code()
            );
        }

        Ok(())
    }

    pub fn database_url(&self) -> String {
        format!("mongodb://localhost:{}", self.port)
    }
}

impl Drop for Docker {
    fn drop(&mut self) {
        if let Err(e) = self.close() {
            eprintln!("Error stopping database: {}", e);
        }
    }
}
