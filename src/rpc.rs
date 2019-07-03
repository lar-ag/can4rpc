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

    #[rpc(name = "analog_get_in01")]
    fn analog_get_in01(&self) -> Result<u16>;
    #[rpc(name = "analog_get_in02")]
    fn analog_get_in02(&self) -> Result<u16>;
    #[rpc(name = "analog_get_in03")]
    fn analog_get_in03(&self) -> Result<u16>;
    #[rpc(name = "analog_get_in04")]
    fn analog_get_in04(&self) -> Result<u16>;
    #[rpc(name = "analog_get_in05")]
    fn analog_get_in05(&self) -> Result<u16>;
    #[rpc(name = "analog_get_out")]
    fn analog_get_out(&self) -> Result<u16>;
    #[rpc(name = "analog_set_out")]
    fn analog_set_out(&self,value: u16) -> Result<u16>;
     #[rpc(name = "analog_get_temp01")]
    fn analog_get_temp01(&self) -> Result<u16>;
     #[rpc(name = "analog_get_temp02")]
    fn analog_get_temp02(&self) -> Result<u16>;
     #[rpc(name = "analog_get_temp03")]
    fn analog_get_temp03(&self) -> Result<u16>;

    #[rpc(name = "analog_get_uart01")]
    fn analog_get_uart01(&self) -> Result<Vec<u8>>;
    #[rpc(name = "analog_get_uart02")]
    fn analog_get_uart02(&self) -> Result<Vec<u8>>;
    #[rpc(name = "analog_get_uart03")]
    fn analog_get_uart03(&self) -> Result<Vec<u8>>;

    #[rpc(name = "analog_set_uart01")]
    fn analog_set_uart01(&self,data:Vec<u8>) -> Result<()>;
    #[rpc(name = "analog_set_uart02")]
    fn analog_set_uart02(&self,data:Vec<u8>) -> Result<()>;
    #[rpc(name = "analog_set_uart03")]
    fn analog_set_uart03(&self,data:Vec<u8>) -> Result<()>;


    #[rpc(name = "digital_get_din")]
    fn digital_get_din(&self,node:u32) ->Result<u16>;
    #[rpc(name = "digital_get_dout")]
    fn digital_get_dout(&self,node:u32) ->Result<u16>;
    #[rpc(name = "digital_set_dout")]
    fn digital_set_dout(&self,node:u32,value:u16) -> Result<()>;

    #[rpc(name = "digital_get_din00")]
    fn digital_get_din00(&self,node:u32) ->Result<bool>;
    #[rpc(name = "digital_get_din01")]
    fn digital_get_din01(&self,node:u32) ->Result<bool>;
    #[rpc(name = "digital_get_din02")]
    fn digital_get_din02(&self,node:u32) ->Result<bool>;
    #[rpc(name = "digital_get_din03")]
    fn digital_get_din03(&self,node:u32) ->Result<bool>;
    #[rpc(name = "digital_get_din04")]
    fn digital_get_din04(&self,node:u32) ->Result<bool>;
    #[rpc(name = "digital_get_din05")]
    fn digital_get_din05(&self,node:u32) ->Result<bool>;
    #[rpc(name = "digital_get_din06")]
    fn digital_get_din06(&self,node:u32) ->Result<bool>;
    #[rpc(name = "digital_get_din07")]
    fn digital_get_din07(&self,node:u32) ->Result<bool>;
    #[rpc(name = "digital_get_din08")]
    fn digital_get_din08(&self,node:u32) ->Result<bool>;
    #[rpc(name = "digital_get_din09")]
    fn digital_get_din09(&self,node:u32) ->Result<bool>;
    #[rpc(name = "digital_get_din10")]
    fn digital_get_din10(&self,node:u32) ->Result<bool>;
    #[rpc(name = "digital_get_din11")]
    fn digital_get_din11(&self,node:u32) ->Result<bool>;
    #[rpc(name = "digital_get_din12")]
    fn digital_get_din12(&self,node:u32) ->Result<bool>;
    #[rpc(name = "digital_get_din13")]
    fn digital_get_din13(&self,node:u32) ->Result<bool>;
    #[rpc(name = "digital_get_din14")]
    fn digital_get_din14(&self,node:u32) ->Result<bool>;
    #[rpc(name = "digital_get_din15")]
    fn digital_get_din15(&self,node:u32) ->Result<bool>;


    #[rpc(name = "digital_get_dout00")]
    fn digital_get_dout00(&self,node:u32) ->Result<bool>;
    #[rpc(name = "digital_get_dout01")]
    fn digital_get_dout01(&self,node:u32) ->Result<bool>;
    #[rpc(name = "digital_get_dout02")]
    fn digital_get_dout02(&self,node:u32) ->Result<bool>;
    #[rpc(name = "digital_get_dout03")]
    fn digital_get_dout03(&self,node:u32) ->Result<bool>;
    #[rpc(name = "digital_get_dout04")]
    fn digital_get_dout04(&self,node:u32) ->Result<bool>;
    #[rpc(name = "digital_get_dout05")]
    fn digital_get_dout05(&self,node:u32) ->Result<bool>;
    #[rpc(name = "digital_get_dout06")]
    fn digital_get_dout06(&self,node:u32) ->Result<bool>;
    #[rpc(name = "digital_get_dout07")]
    fn digital_get_dout07(&self,node:u32) ->Result<bool>;
    #[rpc(name = "digital_get_dout08")]
    fn digital_get_dout08(&self,node:u32) ->Result<bool>;
    #[rpc(name = "digital_get_dout09")]
    fn digital_get_dout09(&self,node:u32) ->Result<bool>;
    #[rpc(name = "digital_get_dout10")]
    fn digital_get_dout10(&self,node:u32) ->Result<bool>;
    #[rpc(name = "digital_get_dout11")]
    fn digital_get_dout11(&self,node:u32) ->Result<bool>;
    #[rpc(name = "digital_get_dout12")]
    fn digital_get_dout12(&self,node:u32) ->Result<bool>;
    #[rpc(name = "digital_get_dout13")]
    fn digital_get_dout13(&self,node:u32) ->Result<bool>;
    #[rpc(name = "digital_get_dout14")]
    fn digital_get_dout14(&self,node:u32) ->Result<bool>;
    #[rpc(name = "digital_get_dout15")]
    fn digital_get_dout15(&self,node:u32) ->Result<bool>;

    #[rpc(name = "digital_set_dout00")]
    fn digital_set_dout00(&self,node:u32,value:bool) -> Result<()>;
    #[rpc(name = "digital_set_dout01")]
    fn digital_set_dout01(&self,node:u32,value:bool) -> Result<()>;
    #[rpc(name = "digital_set_dout02")]
    fn digital_set_dout02(&self,node:u32,value:bool) -> Result<()>;
    #[rpc(name = "digital_set_dout03")]
    fn digital_set_dout03(&self,node:u32,value:bool) -> Result<()>;
    #[rpc(name = "digital_set_dout04")]
    fn digital_set_dout04(&self,node:u32,value:bool) -> Result<()>;
    #[rpc(name = "digital_set_dout05")]
    fn digital_set_dout05(&self,node:u32,value:bool) -> Result<()>;
    #[rpc(name = "digital_set_dout06")]
    fn digital_set_dout06(&self,node:u32,value:bool) -> Result<()>;
    #[rpc(name = "digital_set_dout07")]
    fn digital_set_dout07(&self,node:u32,value:bool) -> Result<()>;
    #[rpc(name = "digital_set_dout08")]
    fn digital_set_dout08(&self,node:u32,value:bool) -> Result<()>;
    #[rpc(name = "digital_set_dout09")]
    fn digital_set_dout09(&self,node:u32,value:bool) -> Result<()>;
    #[rpc(name = "digital_set_dout10")]
    fn digital_set_dout10(&self,node:u32,value:bool) -> Result<()>;
    #[rpc(name = "digital_set_dout11")]
    fn digital_set_dout11(&self,node:u32,value:bool) -> Result<()>;
    #[rpc(name = "digital_set_dout12")]
    fn digital_set_dout12(&self,node:u32,value:bool) -> Result<()>;
    #[rpc(name = "digital_set_dout13")]
    fn digital_set_dout13(&self,node:u32,value:bool) -> Result<()>;
    #[rpc(name = "digital_set_dout14")]
    fn digital_set_dout14(&self,node:u32,value:bool) -> Result<()>;
    #[rpc(name = "digital_set_dout15")]
    fn digital_set_dout15(&self,node:u32,value:bool) -> Result<()>;

    #[rpc(name = "motor_get_uart01")]
    fn motor_get_uart01(&self,node:u32)-> Result<Vec<u8>>;
    #[rpc(name = "motor_get_uart01")]
    fn motor_get_uart02(&self,node:u32)-> Result<Vec<u8>>;
    #[rpc(name = "motor_get_uart01")]
    fn motor_get_uart03(&self,node:u32)-> Result<Vec<u8>>;
    #[rpc(name = "motor_get_uart01")]
    fn motor_get_uart04(&self,node:u32)-> Result<Vec<u8>>;
    #[rpc(name = "motor_set_uart01")]
    fn motor_set_uart01(&self,data:Vec<u8>)-> Result<()>;
    #[rpc(name = "motor_set_uart02")]
    fn motor_set_uart02(&self,data:Vec<u8>)-> Result<()>;
    #[rpc(name = "motor_set_uart03")]
    fn motor_set_uart03(&self,data:Vec<u8>)-> Result<()>;
    #[rpc(name = "motor_set_uart01")]
    fn motor_set_uart04(&self,data:Vec<u8>)-> Result<()>;

    #[rpc(name = "motor_set_baut01")]
    fn motor_set_baut01(&self,bautrate:u8)-> Result<()>;
    #[rpc(name = "motor_set_baut02")]
    fn motor_set_baut02(&self,bautrate:u8)-> Result<()>;
    #[rpc(name = "motor_set_baut03")]
    fn motor_set_baut03(&self,bautrate:u8)-> Result<()>;
    #[rpc(name = "motor_set_baut04")]
    fn motor_set_baut04(&self,bautrate:u8)-> Result<()>;

    #[rpc(name = "analogext_get_aout")]
    fn analogext_get_aout(&self, num: u8) -> Result<u16>;
    #[rpc(name = "analogext_set_aout")]
    fn analogext_set_aout(&self, num: u8, val: u16) -> FutureResult<(),Error>;
}




