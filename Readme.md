# Hello World in Rust as a debian package and a systemd service

This is just an example/playground for packaging a Rust app as a .deb package
(using [cargo-deb](https://github.com/mmstick/cargo-deb)) which also contains a (systemd) service that automatically
starts the app once the .deb package is installed.

Requires Rust 1.31+, and optionally `dpkg`, `ldd` and `liblzma-dev`. Compatible with Ubuntu.

## Installation
Once you clone the repo do:

```sh
cargo build && cargo deb
sudo dpkg -i target/debian/hello-world_0.1.0_amd64.deb
```

It will install a `hello` binary under `/usr/bin/` and also a `hello.service` under
`/lib/systemd/system/`. Once installed the service will be enabled and started.


## Usage
You can see the status using:
```sh
sudo systemctl status hello
```

[systemctl](https://manpages.debian.org/stretch/systemd/systemctl.1.en.html) is a wrapper around systemd services so it offers many goodies (stop/restart etc).

To see the logs you can use the [journalctl](https://manpages.debian.org/stretch/systemd/journalctl.1.en.html):
```sh
sudo journalctl -u hello.service
```


## Uninstall
In order to completely remove the binary and the systemd all you need to do is:

```sh
sudo dpkg --purge hello-world
```
