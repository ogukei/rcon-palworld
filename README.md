# rcon-rs
A simple RCON client written in Rust

```
RCON_ENDPOINT="127.0.0.1:27015" RCON_PASSWORD="passwrd" RCON_COMMAND="/some command" RUST_LOG=trace cargo run --release
```

* RCON spec
  * https://developer.valvesoftware.com/wiki/Source_RCON_Protocol

## Install

```
# install cargo-deb
cargo install cargo-deb
# edit rcon_cli/distribution/config/rcon-cli.ini
cargo deb -p rcon_cli -v
# install .deb
dpkg -i target/debian/rcon-cli_0.1.0-1_amd64.deb
systemctl start rcon_cli
journalctl -u rcon_cli
# remove 
dpkg -r rcon-cli
```
