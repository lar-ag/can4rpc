// #![feature(async_await)]
// use jsonrpc_tcp_server::jsonrpc_core::IoHandler;
// use jsonrpc_tcp_server::ServerBuilder;
// use jsonrpc_tcp_server::jsonrpc_core::{IoHandler};
// use jsonrpc_tcp_server::{AccessControlAllowOrigin, DomainsValidation, RestApi, ServerBuilder};
use jsonrpc_core::futures::Future;
// use jsonrpc_core::{Error, ErrorCode};
// use jsonrpc_derive::rpc;
// use jsonrpc_pubsub::typed;
use jsonrpc_pubsub::{PubSubHandler, Session};

use std::sync::{Arc};
use std::thread;

use can4rpc::*;
use std::net::*;
// use std::io::prelude::*;

// use std::io::prelude::*;
// use structopt;
// With the "paw" feature enabled in structopt

// #[derive(Debug, StructOpt)]
// #[structopt(name = "example", about = "An example of StructOpt usage.")]
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
    let mut io = PubSubHandler::default();
	let subscribe = Subscription::default();
    let active_subscriptions = subscribe.active_subscription();
	thread::spawn(move || loop {
		{
			let subscribers = active_subscriptions.read().unwrap();
			for sink in subscribers.values() {
				let _ = sink.notify(Ok("Hello World!".into())).wait();
			}
		}
		thread::sleep(std::time::Duration::from_secs(1));
	});
	io.extend_with(subscribe.to_delegate());
	io.extend_with(AnalogNode.to_delegate());
	io.extend_with(DMNode.to_delegate());
    io.extend_with(DigitalNode.to_delegate());
    io.extend_with(AOutsNode.to_delegate());
	let server = jsonrpc_tcp_server::ServerBuilder::with_meta_extractor(io, |context: &jsonrpc_tcp_server::RequestContext| {
			Arc::new(Session::new(context.sender.clone()))
		})
		.start(&SocketAddr::new(args.address.parse().unwrap(),args.port))
		.expect("Server must start with no issues");

	server.wait();
    Ok(())
}




//
