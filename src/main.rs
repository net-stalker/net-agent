use clap::Parser;
use log4rs::config::{load_config_file, Deserializers};
use net_agent::{agent::Agent, config::Config};
use net_agent::args::Cli;

fn main() {
    if cfg!(debug_assertions) {
        init_log();
    }
    log::info!("info log for test");
    log::debug!("debug log for test");
    let cli = Cli::parse();

    let config = if cli.config_file.is_some() {
        Config::new(cli.config_file.as_ref().unwrap()).build().unwrap()
    } else {
        Config::cli_builder()
            .with_device_name(cli.device_name.unwrap())
            .with_number_packages(cli.number_packages.unwrap())
            .with_buffer_size(cli.buffer_size.unwrap())
            .with_output_directory(cli.output_directory.unwrap())
            .build()
    };

    let agent = Agent::new(config);
    agent.run();
}

fn init_log() {
    let config_str = include_str!("log4rs.yml");
    // let config = serde_yaml::from_str(config_str).unwrap();
    let config = load_config_file("log4rs.yml", Deserializers::default()).unwrap();
    let a = 10;
}
