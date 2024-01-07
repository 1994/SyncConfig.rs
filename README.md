# SyncConfig
This tool is a command-line utility for synchronizing a file from a specified URL to a local directory. It automatically updates the local file if it differs from the remote file.

## Installing 

This tool is written in Rust, so you'll need a Rust environment to build it.

1. Clone the repository:
```bash
git clone https://github.com/1994/SyncConfig.rs.git
cd SyncConfig.rs
```

2. Build with Cargo:

3. The executable will be located in target/release/.

## Configuration

Before you start, create a configuration file (e.g., config.toml) with the following parameters:

- url: The URL of the file to synchronize.
- target_path: The local file path where the file will be stored.
- update_interval_secs: The synchronization interval in seconds.

Example: 
```toml
url = "http://example.com/file.txt"
target_path = "local_file.txt"
update_interval_secs = 3600
```

## Usage

```bash
./target/release/sync_config --config /path/to/config.toml
```






