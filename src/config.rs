use std::fmt::{Debug, Formatter};

use serde::{Deserialize, Serialize};
use toml::to_string;

use net_config::NetConfig;
#[allow(unused_imports)]
use std::env;


#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Capture {
    pub(crate) device_name: String,
    pub(crate) number_packages: i32,
    pub(crate) buffer_size: i32,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, NetConfig)]
pub struct Config {
    pub(crate) capture: Capture,
}
