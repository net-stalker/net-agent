use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;
use std::sync::Arc;

use clap::Parser;
use log4rs::config::{load_config_file, Deserializers};
use net_agent::{agent::Agent, config::Config};
use net_agent::args::Cli;
use tabled::Table;
use temp_dir::TempDir;
use threadpool::ThreadPool;

fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    let pool = ThreadPool::new(1);

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl+C handler");

    read_log4rs_from_temp_dir();
    let cli = Cli::parse();

    let config = if cli.config_file.is_some() {
        Config::new(cli.config_file.as_ref().unwrap()).build().unwrap()
    } else {
        let config = Config::cli_builder()
            .with_device_name(cli.device_name.unwrap())
            .with_buffer_size(cli.buffer_size.unwrap())
            .with_output_directory(cli.output_directory.unwrap());
        let config = if cli.number_packages.is_some() {
            config.with_number_packages(cli.number_packages.unwrap())
        } else {
            config
        };
        config.build()
    };
    log::warn!("Loaded config");
    log::warn!("{}", Table::new(vec![config.clone()]).to_string());
    let agent = Agent::new(config, running.clone());
    let captured_packets_counter = Arc::new(AtomicU64::new(0));
    let cnt = captured_packets_counter.clone();

    pool.execute(move|| captured_packets_counter.store(agent.run(), Ordering::SeqCst));
    
    while running.load(Ordering::SeqCst) { /* do nothing club */ }
    
    pool.join();

    log::warn!("Finished capturing!");
    log::warn!("Captured {} files", cnt.load(Ordering::SeqCst));
}

fn read_log4rs_from_temp_dir() {
    const LOG4RS_FILE_NAME: &str = "log4rs.yml";
    let d = TempDir::new().unwrap();
    let raw_log_config = d.child(LOG4RS_FILE_NAME);
    let bytes = include_bytes!("../log4rs.yml");
    std::fs::write(raw_log_config, bytes)
        .expect("valid log config is expected. Error while writing log config to a temporary file");
    let mut config = load_config_file(
        format!("{}/{}", d.path().display(), LOG4RS_FILE_NAME),
        Deserializers::default()
    ).expect("excepted to build a config for log4rs");

    if !cfg!(debug_assertion) {
        config.root_mut().set_level(log::LevelFilter::Info);
    }
    log4rs::config::init_config(config).unwrap();
}
