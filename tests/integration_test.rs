use std::{process::Command, path::Path, error::Error, env, fs::File, io::Write};

use assert_cmd::prelude::{CommandCargoExt, OutputAssertExt};
use predicates::prelude::predicate;
use uuid::Uuid;

const APP: &str = "example-cli";

#[test]
fn error_when_arg_config_doesnt_exist() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin(APP)?;
    cmd.arg("--config").arg("__should_not_exist__.json");
    cmd.assert().failure().stderr(predicate::str::contains("not found"));
    Ok(())
}

#[test]
fn var_matches_env_var() -> Result<(), Box<dyn Error>> {
    temp_env::with_var("EXAMPLE_CLI_verbose", Some("trace"), || {
        env::set_var("EXAMPLE_CLI_verbose", "trace");
        let mut cmd = Command::cargo_bin(APP).unwrap();
        cmd.arg("config");
        cmd.assert().success().stdout(predicate::str::contains("verbose = \"trace\""));
    });
    Ok(())
}

#[test]
fn var_matches_config_file() -> Result<(), Box<dyn Error>> {
    // unset the env var to ensure it does not affect the test.
    // env::remove_var("EXAMPLE_CLI_verbose");
    temp_env::with_var("EXAMPLE_CLI_verbose", None::<&str>, || -> Result<(), Box<dyn Error>> {
        let dir = tempfile::tempdir()?;
        let filename = Uuid::new_v4().to_string();
        let config_path = dir.path().join(filename);
        let mut config = File::create(&config_path)?;
        write!(config, "verbose = debug").unwrap();
        let mut cmd = Command::cargo_bin(APP)?;
        cmd.arg("--config").arg(config_path.as_os_str()).arg("config");
        cmd.assert().success().stdout(predicate::str::contains("verbose = \"debug\""));
        drop(config);
        dir.close()?;
        Ok(())
    }).unwrap();
    Ok(())
}

mod config {
    use std::fs;

    use super::*;
    
    #[test]
    fn arg_output_file_created() -> Result<(), Box<dyn Error>> {
        // Generate a random config filename (assuming it won't already exist)
        let config = Uuid::new_v4().to_string();
        let config = config.as_str();
        assert!(!Path::new(config).exists());
        let mut cmd = Command::cargo_bin(APP)?;
        cmd.arg("config").arg("--output").arg(config);
        cmd.assert().success();
        assert!(Path::new(config).exists());
        fs::remove_file(config).unwrap();
        Ok(())
    }
}
