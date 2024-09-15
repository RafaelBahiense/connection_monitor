# Connection Monitor

## Overview

`connection_monitor` is a Rust application designed to monitor the TCP connections of a specific process and execute a predefined action (such as running a script) when no established connections are found for a specified timeout period. This is particularly useful for gracefully handling unused processes or services, such as shutting down a server when it's no longer in use.

## Releases

This project uses a GitHub Actions workflow to automatically create and release a `.deb` package each time a new version is tagged. This makes it easy to install and update the application on Debian-based systems.

### Using the `.deb` Release

1. Navigate to the [Releases](https://github.com/RafaelBahiense/connection_monitor/releases) section of the GitHub repository.
2. Download the latest `.deb` file.
3. Install the package using your preferred Debian package manager, for example, by running `sudo dpkg -i filename.deb`. (Replace `filename.deb` with the actual file name of the downloaded release.)

## Prerequisites - Use

To use `connection_monitor`, you need:

- A Debian-based system. For non-Debian systems and macOS, contributions are welcome to add support. As for Windows support—well, let's just say it's on the horizon! 🪟😉
- Appropriate permissions to monitor the target process. If the process is owned by another user or requires elevated permissions, you may need to run `connection_monitor` with `sudo`.

## Configuration

`connection_monitor` does not require a configuration file; it is configured via command-line arguments.

### Command-Line Arguments

- `--pid <PID>`: The process ID to monitor (required).
- `--port <PORT>`: The TCP port to monitor for established connections (required).
- `--timeout <SECONDS>`: Timeout duration in seconds before executing the action (default: 60).
- `--interval <SECONDS>`: Polling interval in seconds (default: 5).
- `--script <SCRIPT>`: The script or command to execute when the timeout is reached (required).
- `--log-level <LEVEL>`: Set the log level (`error`, `warn`, `info`, `debug`, `trace`), default is `info`.

### Example Usage

```sh
connection_monitor --pid 1234 --port 8080 --script ./shutdown.sh --timeout 300 --interval 10
```

## Usage

Run the application by executing the installed binary with the required arguments. The application will:

1. Monitor the specified process (by PID) for established TCP connections on the given port.
2. If no established connections are found, it starts a timer.
3. If the timer reaches the specified timeout without any new connections, it executes the provided script.
4. The timer resets if a connection is established before the timeout is reached.

### Example Script

The script you provide can perform any action you desire. For example, to shut down the monitored process:

```sh
#!/bin/bash
echo "No connections found. Shutting down process."
kill -SIGTERM <PID>
```

Make sure your script is executable:

```sh
chmod +x shutdown.sh
```

## Prerequisites - Development

To build or modify `connection_monitor`, you need:

- A Rust development environment (Rust 1.56 or later).
- `cargo-deb` for building the `.deb` package (install with `cargo install cargo-deb`).

## Building

1. Clone the repository to your local machine:

   ```sh
   git clone https://github.com/RafaelBahiense/connection_monitor.git
   ```

2. Navigate to the project directory:

   ```sh
   cd connection_monitor
   ```

3. Build the project using Cargo:

   ```sh
   cargo build --release
   ```

4. The executable will be available in the `target/release` directory.

### Building the `.deb` Package

To build the `.deb` package locally:

```sh
cargo install cargo-deb
cargo deb --no-build
```

The `.deb` file will be generated in the `target/debian` directory.

## Dependencies

- `clap`: For command-line argument parsing.
- `log`: Logging facade.
- `simple_logger`: Simple logger implementation.
- `procfs`: For accessing process information via the `/proc` filesystem.

## FAQ

- **Why not just use a bash script with `netstat`?**

  Bash is cursed

- **Does this application support monitoring UDP connections?**

  Currently, `connection_monitor` monitors TCP connections. Support for UDP connections could be added in future versions or via community contributions.

- **Can I monitor multiple processes or ports simultaneously?**

  You can run multiple instances of `connection_monitor` to monitor multiple processes or ports with GNU Parallel.

## Contributing

Contributions to the project are welcome! Please follow the standard Git workflow:

1. Fork the repository.
2. Create a new branch for your feature or bugfix.
3. Make your changes.
4. Submit a pull request with a clear description of your changes.

## License

`connection_monitor` is licensed under the GLWT Public License. See [LICENSE](LICENSE) for details.
