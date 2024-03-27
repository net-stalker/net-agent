use std::fmt::{Debug, Formatter};

use serde::{Deserialize, Serialize};
use toml::to_string;

use net_config::NetConfig;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct HubConnector {
    pub(crate) addr: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Capture {
    pub(crate) device_name: String,
    pub(crate) number_packages: i32,
    pub(crate) buffer_size: i32,
    pub(crate) capturing_payload: bool,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, NetConfig)]
pub struct Config {
    pub(crate) group_id: String,
    pub(crate) agent_id: String,

    pub(crate) hub_connector: HubConnector,
    pub(crate) capture: Capture,
}

#[cfg(test)]
mod tests {
    use std::env;
    use super::*;

    #[test]
    fn expected_load_config() {
        let config = Config::builder()
            .with_config_dir(".config".to_string())
            .build();

        let expected_config = Config {
            group_id: "7b9475ee-48fd-471e-a3d0-1c663029c9b6".into(),
            agent_id: "5e199f67-4fbd-4ff5-9cda-61bfb95106b6".into(),

            hub_connector: HubConnector {
                addr: "tcp://0.0.0.0:5555".to_string(),
            },

            capture: Capture {
                device_name: "en0".to_string(),
                number_packages: -1,
                buffer_size: 1000,
                capturing_payload: true,
            },
        };

        assert_eq!(config.unwrap(), expected_config);

        env::set_var("NET_GROUP_ID", "ANOTHER_GROUP_UID");
        env::set_var("NET_AGENT_ID", "ANOTHER_AGENT_UID");
        env::set_var("NET_HUB_CONNECTOR.ADDR", "tcp://localhost:5555");
        env::set_var("NET_CAPTURE.DEVICE_NAME", "en1");
        env::set_var("NET_CAPTURE.NUMBER_PACKAGES", "1");
        env::set_var("NET_CAPTURE.buffer_size", "10");
        env::set_var("NET_CAPTURE.capturing_payload", "false");

        let config = Config::builder().build();

        let expected_config = Config {
            group_id: "ANOTHER_GROUP_UID".into(),
            agent_id: "ANOTHER_AGENT_UID".into(),

            hub_connector: HubConnector {
                addr: "tcp://localhost:5555".to_string(),
            },

            capture: Capture {
                device_name: "en1".to_string(),
                number_packages: 1,
                buffer_size: 10,
                capturing_payload: false,
            },
        };

        assert_eq!(config.unwrap(), expected_config);
    }
}