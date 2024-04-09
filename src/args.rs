use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short = 'D', long, conflicts_with_all = ["config_file"])]
    pub device_name: Option<String>,
    #[arg(short = 'N', long, conflicts_with_all = ["config_file"])]
    pub number_packages: Option<i32>,
    #[arg(short = 'B', long, conflicts_with_all = ["config_file"])]
    pub buffer_size: Option<i32>,
    #[arg(short = 'C', long, conflicts_with_all = ["device_name", "number_packages", "buffer_size"])]
    pub config_file: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_default_values() {
        let cli = Cli::parse_from(&["test"]);

        assert_eq!(cli.device_name, None);
        assert_eq!(cli.number_packages, None);
        assert_eq!(cli.buffer_size, None);
        assert_eq!(cli.config_file, None);
    }

    #[test]
    fn test_cli_custom_values_long() {
        let cli = Cli::parse_from(&["test", "--device-name", "eth0", "--number-packages", "100", "--buffer-size", "1024"]);

        assert_eq!(cli.device_name, Some("eth0".to_string()));
        assert_eq!(cli.number_packages, Some(100));
        assert_eq!(cli.buffer_size, Some(1024));
        assert_eq!(cli.config_file, None);
    }

    #[test]
    fn test_cli_custom_values_short() {
        let cli = Cli::parse_from(&["test", "-D", "eth0", "-N", "100", "-B", "1024"]);

        assert_eq!(cli.device_name, Some("eth0".to_string()));
        assert_eq!(cli.number_packages, Some(100));
        assert_eq!(cli.buffer_size, Some(1024));
        assert_eq!(cli.config_file, None);
    }

    #[test]
    fn test_panic_lower_case_short() {
        assert!(Cli::try_parse_from(&["test", "-d", "eth0", "-N", "100", "-B", "1024"]).is_err());
    }

    #[test]
    fn test_cli_with_config_and_all_the_other_args() { 
        assert!(Cli::try_parse_from(&["test", "-d", "eth0", "-n", "100", "-b", "1024", "--config-file", "config.toml"]).is_err());
    }

    #[test]
    fn test_cli_with_long_config() {
        let cli = Cli::try_parse_from(&["test", "--config-file", "config.toml"]);
        assert!(cli.is_ok());
        let cli = cli.unwrap();
        assert_eq!(cli.device_name, None);
        assert_eq!(cli.number_packages, None);
        assert_eq!(cli.buffer_size, None);
        assert_eq!(cli.config_file, Some("config.toml".to_string()));
    }

    #[test]
    fn test_cli_with_short_config() {
        let cli = Cli::try_parse_from(&["test", "-C", "config.toml"]);
        assert!(cli.is_ok());
        let cli = cli.unwrap();
        assert_eq!(cli.device_name, None);
        assert_eq!(cli.number_packages, None);
        assert_eq!(cli.buffer_size, None);
        assert_eq!(cli.config_file, Some("config.toml".to_string()));
    }
}
