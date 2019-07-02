use jsonrpc_core::futures::future::{self, Future, FutureResult};
use jsonrpc_core::{Error, IoHandler, Result};
// use jsonrpc_core_client::transports::local;
use jsonrpc_derive::rpc;

use super::MioError;



#[rpc]
pub trait Rpc {
    // #[rpc(name = "protocolVersion")]
    // fn protocol_version(&self) -> Result<String>;

    #[rpc(name = "read_index")]
    fn read_index(&self,node:u32,index:u16, sub: u8) -> Result<String>;
    #[rpc(name = "write_index")]
    fn write_index(&self,node:u32,index:u16, sub: u8,data:Vec<u8>) -> Result<()>;

    #[rpc(name = "get_ain")]
    fn get_ain(&self, num: u8) -> Result<u16>;
    #[rpc(name = "get_aout")]
    fn get_aout(&self, num: u8) -> Result<u16>;
    #[rpc(name = "set_aout")]
    fn set_aout(&self, num: u8, val: u16) -> FutureResult<(),Error>;
    #[rpc(name = "set_aout")]
    fn get_temperatur(&self,num:u8) -> Result<u16>;

    #[rpc(name = "get_din")]
    fn get_din(&self,digit:u8) ->Result<bool>;
    #[rpc(name = "get_dout")]
    fn get_dout(&self,digit:u8) ->Result<bool>;
     #[rpc(name = "set_dout")]
    fn set_dout(&self,digit:u8,value:bool) ->Result<()>;

    #[rpc(name = "setup_uart")]
    fn setup_uart(&self,uart:u8,baut: u16)->Result<()>;
    #[rpc(name = "read_uart")]
    fn read_uart(&self,uart:u8)->FutureResult<Vec<u8>,Error>;
    #[rpc(name = "write_uart")]
    fn write_uart(&self,uart:u8,data: Vec<u8>)->Result<()>;
}




