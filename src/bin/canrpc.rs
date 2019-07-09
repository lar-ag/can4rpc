// #![feature(async_await)]
use jsonrpc_http_server::jsonrpc_core::*;
use jsonrpc_http_server::{AccessControlAllowOrigin, DomainsValidation, RestApi, ServerBuilder};
use wqm_can::*;

fn main() {
    let mut io = IoHandler::new();
	io.extend_with(RpcServer.to_delegate());

	let server = ServerBuilder::new(io)
		.threads(3)
		.rest_api(RestApi::Unsecure)
		.cors(DomainsValidation::AllowOnly(vec![AccessControlAllowOrigin::Any]))
		.start_http(&"127.0.0.1:3030".parse().unwrap())
		.expect("Unable to start RPC server");

	server.wait();
}
