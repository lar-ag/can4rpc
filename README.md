# :pager: `wqm-can`

![pipeline status](https://travis-ci.org/lar-ag/wqm-can.svg?branch=master)


Water quality analyzer can binding.

🚧 _Work In Progress_ 🚧

**TODO:**  Driver?

##  Compiling

Requires Rust nightly. To compile using [`rustup`](https://rustup.rs/):

```ShellSession
$ rustup toolchain install nightly
$ rustup default nightly
$ cargo build
```

Be sure to switch back to `stable` with `rustup default stable` if that's your preferred toolchain.


To cross-compile for the Raspberry Pi you will need an
`gcc-multilib-i686-linux-gnu` GCC toolchain and Rust component installed. Add the Rust target
with `rustup target add i686-unknown-linux-gnu`. Then you can
cross-compile with `cargo`:

```ShellSession
    cargo build --release --target i686-unknown-linux-gnu
```

`target/i686-unknown-linux-gnu/release/wqm-uv` 


[ui]: https://user-images.githubusercontent.com/383250/59148363-53188c80-8a08-11e9-9b29-9cac56809ee2.png "Automaat UI Example"


## Communication uber `jsonrpc`

```shell
cargo add jsonrpc-core
cargo add jsonrpc-delive
```

## 🚀 Deployment

## Testing

Integrating the test into a CI system is non-trivial as it relies on a `vcan0` virtual can device existing. Adding one to most linux systems is pretty easy with root access but attaching a vcan device to a container for CI seems difficult to find support for.

To run the tests locally, though, setup should be simple:

```sh
sudo modprobe vcan
sudo ip link add vcan0 type vcan
sudo ip link set vcan0 up
cargo test
```

[package.metadata.deb]
maintainer = "Harry Gill <tech@gill.net.in>"
copyright = "2019, Harry Gill"
depends = "$auto, systemd"
conf-files = ["/usr/local/etc/tide-config.ini", "/etc/systemd/system/tide-server.service"]
extended-description = """\
web-server written in rust.\
"""
section = "admin"
priority = "optional"
assets = [
    ["target/release/tide-server", "/usr/local/bin/", "755"],
    ["assets/tide-config.ini", "/usr/local/etc/", "644"],
    ["assets/tide-server.service", "/etc/systemd/system/", "644"],
]
