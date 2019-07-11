# :pager: `can4rpc`

![pipeline status](https://travis-ci.org/lar-ag/can4rpc.svg?branch=master)


Water quality monitor can binding `jsonrpc`

🚧 _Work In Progress_ 🚧


## API
**protocol:**`jsonrpc`
[Auf GitHub](https://github.com/paritytech/jsonrpc/)

![Build Status][travis-image]][travis-url]
[![Build Status][appveyor-image]][appveyor-url]

[Documentation](http://paritytech.github.io/jsonrpc/)

[travis-image]: https://travis-ci.org/paritytech/jsonrpc.svg?branch=master
[travis-url]: https://travis-ci.org/paritytech/jsonrpc
[appveyor-image]: https://ci.appveyor.com/api/projects/status/github/paritytech/jsonrpc?svg=true
[appveyor-url]: https://ci.appveyor.com/project/paritytech/jsonrpc/branch/master

```shell
cargo add jsonrpc-core
cargo add jsonrpc-delive
```

**Testing:**
Read analog1 in01
```
curl -X POST -H "Content-Type: application/json" -d '{"jsonrpc": "2.0", "method": "analog_get_in01", "id":123 }' 127.0.0.1:3030
```


##  Compiling

Requires Rust nightly. To compile using [`rustup`](https://rustup.rs/):

```ShellSession
$ curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain nightly
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


## Quick Links

* Work List: https://
* API docs: https://lar-ag/github.io/canrpc/ra_ide_api/index.html
* CI: https://travis-ci.org/rust-analyzer/rust-analyzer

## 🚀 PCan uber `socketcan`

[socketcan] crate.

### PCan soketcan setup vcan0

Integrating the test into a CI system is non-trivial as it relies on a `vcan0` virtual can device existing. Adding one to most linux systems is pretty easy with root access but attaching a vcan device to a container for CI seems difficult to find support for.

To run the tests locally, though, setup should be simple:

```sh
sudo modprobe vcan
sudo ip link add vcan0 type vcan
sudo ip link set vcan0 up
cargo test
```
