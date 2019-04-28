use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::io::{Error as IOError, ErrorKind, Result as IOResult};
use toml;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub name: String,
}

impl Config {
    pub fn new(name: String) -> Config {
        Config { name: name }
    }

    pub fn get(path: &str) -> IOResult<Config> {
        let file = File::open(path)?;
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents)?;
        let result: Result<Config, toml::de::Error> = toml::from_str(&contents);

        match result {
            Ok(config) => Ok(config),
            Err(_) => Err(IOError::new(
                ErrorKind::InvalidData,
                "Config file contains invalid Toml data.",
            )),
        }
    }

    pub fn write(&self) -> IOResult<()> {
        let config_text =
            toml::to_string_pretty(&self).expect("It was not possible to serialize configuration.");

        // Todo: get filename from app.config
        let mut file = File::create("todoco.toml")?;

        file.write_all(config_text.as_bytes())?;

        Ok(())
    }
}

// ~~~~~~~~~~~~~~~~~~~~ TESTS ~~~~~~~~~~~~~~~~~~~~ //
#[cfg(test)]
mod tests {
    use super::Config;
    use std::fs::{remove_file, File};
    use std::io::Read;

    #[test]
    fn write_config_test() {
        let config = Config::new(String::from("bla"));
        config.write();
        let mut content = String::new();
        File::open("todoco.toml")
            .unwrap()
            .read_to_string(&mut content);
        let expected = "name = \'bla\'\n";
        assert_eq!(content, expected);
        remove_file("todoco.toml");
    }
}
