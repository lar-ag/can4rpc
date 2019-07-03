# :pager: `wqm-can`

![pipeline status](https://travis-ci.org/lar-ag/wqm-can.svg?branch=master)


Water quality analyzer can binding.

🚧 _Work In Progress_ 🚧

**TODO:**  Driver?

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
