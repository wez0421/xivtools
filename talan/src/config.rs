use failure::Error;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

// This module handles all configuration management for Talan.

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct Delays {
    pub gcd_buff: f32,
    pub gcd_action: f32,
}

impl Default for Delays {
    fn default() -> Delays {
        Delays {
            gcd_buff: 2.0,
            gcd_action: 2.5,
        }
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct GearSets {
    pub crp: i32,
    pub bsm: i32,
    pub arm: i32,
    pub gsm: i32,
    pub ltw: i32,
    pub wvr: i32,
    pub alc: i32,
    pub cul: i32,
}

impl Default for GearSets {
    fn default() -> GearSets {
        GearSets {
            crp: 0,
            bsm: 0,
            arm: 0,
            gsm: 0,
            ltw: 0,
            wvr: 0,
            alc: 0,
            cul: 0,
        }
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub gear: GearSets,
    pub delays: Delays,
}

const CONFIG_PATH: &str = "config.json";

// Reads the config from disk. Returns a default config
// if one cannot be found.
pub fn read_config() -> Config {
    read_config_internal(Path::new(CONFIG_PATH))
}

// Writes |cfg| to disk.
pub fn write_config(cfg: &Config) -> Result<(), Error> {
    write_config_internal(cfg, Path::new(CONFIG_PATH))
}

fn read_config_internal(path: &Path) -> Config {
    match read_config_from_file(path) {
        Ok(c) => c,
        Err(_) => {
            println!("Config not found, creating a new one!");
            Config::default()
        }
    }
}

fn read_config_from_file(path: &Path) -> Result<Config, Error> {
    let mut contents = String::new();
    let mut f = File::open(path)?;
    f.read_to_string(&mut contents)?;
    Ok(serde_json::from_str(&contents)?)
}

fn write_config_internal(cfg: &Config, path: &Path) -> Result<(), Error> {
    let mut f = File::create(path)?;
    f.write_all(serde_json::to_string(&cfg)?.as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::remove_file;
    use tempfile::NamedTempFile;

    #[test]
    fn test_write_and_read() -> Result<(), Error> {
        let mut file = NamedTempFile::new()?;
        let mut c1 = Config::default();
        c1.gear.crp = 1;
        c1.gear.bsm = 2;
        c1.gear.arm = 3;
        c1.gear.gsm = 4;
        c1.gear.ltw = 5;
        c1.gear.wvr = 6;
        c1.gear.alc = 7;
        c1.gear.cul = 8;
        assert!(write_config_internal(&c1, file.path()).is_ok());
        let c2 = read_config_internal(file.path());
        assert_eq!(c1, c2);
        Ok(())
    }

    #[test]
    fn test_default_config() -> Result<(), Error> {
        let mut file = NamedTempFile::new()?;
        assert_eq!(read_config_internal(file.path()), Config::default());
        Ok(())
    }
}
