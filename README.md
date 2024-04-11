# Net Agent

[![Crates.io](https://img.shields.io/crates/v/net-agent.svg)](https://crates.io/crates/net-agent)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

Net-agent is a stand-alone binary crate within the Net Stalker project that allows you to capture network traffic, which can then be uploaded to [my.netstalker.io](https://my.netstalker.io/) for further analysis.

## Installation

To install `net-agent`, use Cargo:

```shell
cargo install net-agent
```

## Requirements

Before running `net-agent`, make sure you have the necessary dependencies installed for your operating system:

### Linux

- Ubuntu/Debian:

    ```shell
    sudo apt-get install libpcap-dev
    ```

- Fedora:

    ```shell
    sudo dnf install libpcap-devel
    ```
    
- Arch:

    ```shell
    sudo pacman -S libpcap
    ```

### macOS

- `libpcap` should be installed on Mac OS X by default.

### Windows

1. Install [Npcap](https://npcap.com/#download).

2. Download the [Npcap SDK](https://npcap.com/#download).

3. Add the SDK's `/Lib` or `/Lib/x64` folder to your `LIB` environment variable.

## Usage

You can run `net-agent` with the following command-line options:

```shell
net-agent [OPTIONS]
```

The available options are:

- `-D, --device-name <DEVICE_NAME>`: Specify the network device to capture traffic from.

- `-N, --number-packages <NUMBER_PACKAGES>`: Specify the number of packages to capture.

- `-B, --buffer-size <BUFFER_SIZE>`: Specify the buffer size for capturing packages.

- `-O, --output-directory <OUTPUT_DIRECTORY>`: Specify the output directory for captured traffic (default: "output").

Note: The `-C`, `--config-file` option conflicts with all other options. If specified, the configuration will be loaded from the specified file.

## Configuration

To use the configuration file, pass the `-C`, `--config-file` option with the path to the file.

- template

    ```toml
    device_name = <DEVICE_NAME>
    number_packages = <NUMBER_PACKAGES>
    buffer_size = <BUFFER_SIZE>
    output_directory = <OUTPUT_DIRECTORY>
    ```
- example

    ```toml
    device_name = "eth0"
    number_packages = 1000
    buffer_size = 2048
    output_directory = "captured_traffic"
    ```

## Troubleshooting

On linux to run the application with `sudo` it usually needs to run

```shell
sudo cp ~/.cargo/bin/net-agent /usr/local/bin/
```

## Contributing

All the contributions to `net-agent` are welcome! If you find any issues or have suggestions for improvements, please open an issue on the GitHub repository.
If you want to contribute code to the project, please follow these steps:

1. Clone the repository and create a new branch for your feature or bug fix.
2. Make your changes and ensure that the code passes all tests and linting checks.
3. Submit a pull request describing your changes and why they should be merged.

Please note that all contributions are subject to review and approval by the project maintainers.

## License
`net-agent` is licensed under the [Apache License](LICENSE), Version 2.0. See LICENSE for more information.