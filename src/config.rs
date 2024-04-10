use std::fmt::Debug;
use std::fmt::Formatter;

use serde::Deserialize;
use serde::Serialize;
use toml::to_string;

use net_config::NetConfig;
#[allow(unused_imports)]
use std::env;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, NetConfig)]
pub struct Config {
    device_name: String,
    number_packages: i32,
    buffer_size: i32,
    output_directory: String,
}

impl Config {
    pub fn cli_builder() -> ConfigBuilder {
        ConfigBuilder::default()
    }

    pub fn get_device_name(&self) -> &str {
        &self.device_name
    }

    pub fn get_number_packages(&self) -> i32 {
        self.number_packages
    }

    pub fn get_buffer_size(&self) -> i32 {
        self.buffer_size
    }

    pub fn get_output_directory(&self) -> &str {
        &self.output_directory
    }
}

#[derive(Debug, Default)]
pub struct ConfigBuilder {
    device_name: Option<String>,
    number_packages: Option<i32>,
    buffer_size: Option<i32>,
    output_directory: Option<String>,
}

impl ConfigBuilder {
    pub fn with_device_name(mut self, device_name: String) -> Self {
        self.device_name = Some(device_name);
        self
    }

    pub fn with_number_packages(mut self, number_packages: i32) -> Self {
        self.number_packages = Some(number_packages);
        self
    }

    pub fn with_buffer_size(mut self, buffer_size: i32) -> Self {
        self.buffer_size = Some(buffer_size);
        self
    }

    pub fn with_output_directory(mut self, output_directory: String) -> Self {
        self.output_directory = Some(output_directory);
        self
    }

    pub fn build(self) -> Config {
        Config {
            device_name: self.device_name.unwrap(),
            number_packages: self.number_packages.unwrap(),
            buffer_size: self.buffer_size.unwrap(),
            output_directory: self.output_directory.unwrap(),
        }
    }
}
