use net_agent::{agent::Agent, config::Config};

fn main() {
    if cfg!(debug_assertions) {
        init_log();
    }

    let config = Config::new("../net-agent").build().unwrap();
    let agent = Agent::new(config);
    agent.run();
}

fn init_log() {
    let config_str = include_str!("log4rs.yml");
    let config = serde_yaml::from_str(config_str).unwrap();
    log4rs::init_raw_config(config).unwrap();
}
