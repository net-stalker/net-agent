use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short = 'D', long, conflicts_with_all = ["config_file"])]
    pub device_name: Option<String>,
    #[arg(short = 'N', long, conflicts_with_all = ["config_file"])]
    pub number_packages: Option<u64>,
    #[arg(short = 'B', long, conflicts_with_all = ["config_file"])]
    pub buffer_size: Option<i32>,
    #[arg(short = 'C', long, conflicts_with_all = ["device_name", "number_packages", "buffer_size"])]
    pub config_file: Option<String>,
    #[arg(short = 'O', long, default_value = "output", conflicts_with_all = ["config_file"])]
    pub output_directory: Option<String>,
}

impl Cli {
    pub fn is_valid(&self) -> bool {
        self.config_file.is_none() &&
        (
            self.buffer_size.is_none() ||
            self.device_name.is_none() ||
            self.number_packages.is_none()
        )
    }

    pub fn missing_fields_message(&self) -> String {
        let mut missing_fields = Vec::new();

        if self.buffer_size.is_none() {
            missing_fields.push("buffer_size");
        }
        if self.device_name.is_none() {
            missing_fields.push("device_name");
        }
        if self.number_packages.is_none() {
            missing_fields.push("number_packages");
        }

        format!("Missing fields: {}. Provide a path to a valid config file or command line arguments for net-agent.\nYou can use config_file to set all these arguments.\nSee --help",
                missing_fields.join(", "))
    }
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
        assert_eq!(cli.output_directory, Some("output".to_string()));
    }

    #[test]
    fn test_cli_custom_values_long() {
        let cli = Cli::parse_from(&["test", "--device-name", "eth0", "--number-packages", "100", "--buffer-size", "1024", "--output-directory", "my-output"]);

        assert_eq!(cli.device_name, Some("eth0".to_string()));
        assert_eq!(cli.number_packages, Some(100));
        assert_eq!(cli.buffer_size, Some(1024));
        assert_eq!(cli.config_file, None);
        assert_eq!(cli.output_directory, Some("my-output".to_string()));
    }

    #[test]
    fn test_cli_custom_values_short() {
        let cli = Cli::parse_from(&["test", "-D", "eth0", "-N", "100", "-B", "1024", "-O", "my-output"]);

        assert_eq!(cli.device_name, Some("eth0".to_string()));
        assert_eq!(cli.number_packages, Some(100));
        assert_eq!(cli.buffer_size, Some(1024));
        assert_eq!(cli.config_file, None);
        assert_eq!(cli.output_directory, Some("my-output".to_string()));
    }

    #[test]
    fn test_panic_lower_case_short() {
        assert!(Cli::try_parse_from(&["test", "-d", "eth0", "-N", "100", "-B", "1024"]).is_err());
    }

    #[test]
    fn test_cli_with_config_and_all_the_other_args() { 
        assert!(Cli::try_parse_from(&["test", "-D", "eth0", "-N", "100", "-B", "1024", "--config-file", "config.toml"]).is_err());
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
        // Actually all the other args must be set from the config file
        assert_eq!(cli.output_directory, Some("output".to_string()));
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
        assert_eq!(cli.output_directory, Some("output".to_string()));
    }
}
