// #![feature(async_await)]
// use jsonrpc_tcp_server::jsonrpc_core::IoHandler;
// use jsonrpc_tcp_server::ServerBuilder;
// use jsonrpc_tcp_server::jsonrpc_core::{IoHandler};
// use jsonrpc_tcp_server::{AccessControlAllowOrigin, DomainsValidation, RestApi, ServerBuilder};

use jsonrpc_core;
use jsonrpc_tcp_server;
use jsonrpc_core::IoHandler;
use jsonrpc_tcp_server::{ServerBuilder};

use can4rpc::*;
use std::net::*;
// use std::io::prelude::*;

#[derive(structopt::StructOpt)]
struct Args {
    /// Port to listen on.
    #[structopt(short = "p", long = "port", env = "PORT", default_value = "3030")]
    port: u16,
    /// Address to listen on.
    #[structopt(short = "a", long = "address", default_value = "127.0.0.1")]
    address: String,
}

#[paw::main]
fn main(args:Args) -> Result<(), std::io::Error> {
    let mut io  = IoHandler::new();
    io.extend_with(AnalogNode.to_delegate());
	io.extend_with(DMNode.to_delegate());
    io.extend_with(DigitalNode.to_delegate());
    io.extend_with(AOutsNode.to_delegate());

    let server = ServerBuilder::new(io)
		.start(&SocketAddr::new(args.address.parse().unwrap(),args.port))
		.expect("Unable to start RPC server");
	let _ = server.wait();
    Ok(())
}



//
