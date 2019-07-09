use jsonrpc_core::futures::future::{Future};
use jsonrpc_core::{Result};
use jsonrpc_core_client::transports::local;
use jsonrpc_derive::rpc;

use bitvec::prelude::*;

use super::analog;
use super::digital;
use super::doppelmotor;

pub struct RpcServer;

#[rpc]
pub trait Rpc {
    #[rpc(name = "protocolVersion")]
    fn protocol_version(&self) -> Result<String>;

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
    fn analog_set_out(&self,value: u16) -> Result<()>;
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
    #[rpc(name = "analog_set_uart01")]
    fn analog_set_uart01(&self,data:Vec<u8>) -> Result<()>;
    #[rpc(name = "analog_set_uart02")]
    fn analog_set_uart02(&self,data:Vec<u8>) -> Result<()>;

    #[rpc(name = "digital_get_din")]
    fn digital_get_din(&self,node:i32) ->Result<u16>;
    #[rpc(name = "digital_get_dout")]
    fn digital_get_dout(&self,node:i32) ->Result<u16>;
    #[rpc(name = "digital_set_dout")]
    fn digital_set_dout(&self,node:i32,value:u16) -> Result<()>;

    #[rpc(name = "digital_get_din00")]
    fn digital_get_din00(&self,node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_din01")]
    fn digital_get_din01(&self,node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_din02")]
    fn digital_get_din02(&self,node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_din03")]
    fn digital_get_din03(&self,node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_din04")]
    fn digital_get_din04(&self,node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_din05")]
    fn digital_get_din05(&self,node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_din06")]
    fn digital_get_din06(&self,node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_din07")]
    fn digital_get_din07(&self,node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_din08")]
    fn digital_get_din08(&self,node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_din09")]
    fn digital_get_din09(&self,node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_din10")]
    fn digital_get_din10(&self,node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_din11")]
    fn digital_get_din11(&self,node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_din12")]
    fn digital_get_din12(&self,node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_din13")]
    fn digital_get_din13(&self,node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_din14")]
    fn digital_get_din14(&self,node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_din15")]
    fn digital_get_din15(&self,node:i32) ->Result<bool>;


    #[rpc(name = "digital_get_dout00")]
    fn digital_get_dout00(&self,node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_dout01")]
    fn digital_get_dout01(&self,node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_dout02")]
    fn digital_get_dout02(&self,node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_dout03")]
    fn digital_get_dout03(&self,node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_dout04")]
    fn digital_get_dout04(&self,node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_dout05")]
    fn digital_get_dout05(&self,node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_dout06")]
    fn digital_get_dout06(&self,node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_dout07")]
    fn digital_get_dout07(&self,node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_dout08")]
    fn digital_get_dout08(&self,node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_dout09")]
    fn digital_get_dout09(&self,node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_dout10")]
    fn digital_get_dout10(&self,node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_dout11")]
    fn digital_get_dout11(&self,node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_dout12")]
    fn digital_get_dout12(&self,node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_dout13")]
    fn digital_get_dout13(&self,node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_dout14")]
    fn digital_get_dout14(&self,node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_dout15")]
    fn digital_get_dout15(&self,node:i32) ->Result<bool>;

    #[rpc(name = "digital_set_dout00")]
    fn digital_set_dout00(&self,node:i32,value:bool) -> Result<()>;
    #[rpc(name = "digital_set_dout01")]
    fn digital_set_dout01(&self,node:i32,value:bool) -> Result<()>;
    #[rpc(name = "digital_set_dout02")]
    fn digital_set_dout02(&self,node:i32,value:bool) -> Result<()>;
    #[rpc(name = "digital_set_dout03")]
    fn digital_set_dout03(&self,node:i32,value:bool) -> Result<()>;
    #[rpc(name = "digital_set_dout04")]
    fn digital_set_dout04(&self,node:i32,value:bool) -> Result<()>;
    #[rpc(name = "digital_set_dout05")]
    fn digital_set_dout05(&self,node:i32,value:bool) -> Result<()>;
    #[rpc(name = "digital_set_dout06")]
    fn digital_set_dout06(&self,node:i32,value:bool) -> Result<()>;
    #[rpc(name = "digital_set_dout07")]
    fn digital_set_dout07(&self,node:i32,value:bool) -> Result<()>;
    #[rpc(name = "digital_set_dout08")]
    fn digital_set_dout08(&self,node:i32,value:bool) -> Result<()>;
    #[rpc(name = "digital_set_dout09")]
    fn digital_set_dout09(&self,node:i32,value:bool) -> Result<()>;
    #[rpc(name = "digital_set_dout10")]
    fn digital_set_dout10(&self,node:i32,value:bool) -> Result<()>;
    #[rpc(name = "digital_set_dout11")]
    fn digital_set_dout11(&self,node:i32,value:bool) -> Result<()>;
    #[rpc(name = "digital_set_dout12")]
    fn digital_set_dout12(&self,node:i32,value:bool) -> Result<()>;
    #[rpc(name = "digital_set_dout13")]
    fn digital_set_dout13(&self,node:i32,value:bool) -> Result<()>;
    #[rpc(name = "digital_set_dout14")]
    fn digital_set_dout14(&self,node:i32,value:bool) -> Result<()>;
    #[rpc(name = "digital_set_dout15")]
    fn digital_set_dout15(&self,node:i32,value:bool) -> Result<()>;

    #[rpc(name = "motor_get_uart01")]
    fn motor_get_uart01(&self,node:i32)-> Result<Vec<u8>>;
    #[rpc(name = "motor_get_uart01")]
    fn motor_get_uart02(&self,node:i32)-> Result<Vec<u8>>;
    #[rpc(name = "motor_set_uart01")]
    fn motor_set_uart01(&self,node:i32,data:Vec<u8>)-> Result<()>;
    #[rpc(name = "motor_set_uart02")]
    fn motor_set_uart02(&self,node:i32,data:Vec<u8>)-> Result<()>;
    #[rpc(name = "motor_set_baut01")]
    fn motor_set_baut01(&self,node:i32,bautrate:u32)-> Result<()>;
    #[rpc(name = "motor_set_baut02")]
    fn motor_set_baut02(&self,node:i32,bautrate:u32)-> Result<()>;

    #[rpc(name = "analogext_count")]
    fn analogext_count(&self) -> Result<u8>;
    #[rpc(name = "analogext_get_out")]
    fn analogext_get_out(&self, num: u8) -> Result<u16>;
    #[rpc(name = "analogext_set_out")]
    fn analogext_set_out(&self, num: u8, val: u16) -> Result<()>;
}









impl Rpc for RpcServer {
    fn protocol_version(&self) -> Result<String> {
		Ok("version1".into())
	}
    fn analog_get_in01(&self) -> Result<u16> {
        Ok(analog::get_input01(0x2))
    }
    fn analog_get_in02(&self) -> Result<u16> {
        Ok(analog::get_input02(0x2))
    }
    fn analog_get_in03(&self) -> Result<u16> {
        Ok(analog::get_input03(0x2))
    }
    fn analog_get_in04(&self) -> Result<u16> {
        Ok(analog::get_input04(0x2))
    }
    fn analog_get_in05(&self) -> Result<u16> {
        Ok(analog::get_input05(0x2))
    }
    fn analog_get_out(&self) -> Result<u16>{
        Ok(analog::get_out(0x2))
    }
    fn analog_set_out(&self,value: u16) -> Result<()>{
        analog::set_out(0x2,value);
        Ok(())
    }
    fn analog_get_temp01(&self) -> Result<u16>{
        Ok(analog::get_temp01(2))
    }
    fn analog_get_temp02(&self) -> Result<u16>{
        Ok(analog::get_temp02(2))
    }
    fn analog_get_temp03(&self) -> Result<u16>{
        Ok(analog::get_temp03(2))
    }
    fn analog_get_uart01(&self) -> Result<Vec<u8>>{
        Ok(analog::get_uart01(2))
    }
    fn analog_get_uart02(&self) -> Result<Vec<u8>>{
        Ok(analog::get_uart02(2))
    }
    fn analog_set_uart01(&self, data: Vec<u8>) -> Result<()>{
        Ok(analog::set_uart01(0x2,data))
    }
    fn analog_set_uart02(&self, data: Vec<u8>) -> Result<()>{
        Ok(analog::set_uart02(0x2,data))
    }
    fn digital_get_din(&self,node:i32) ->Result<u16>{
        Ok(digital::get_input(node))
    }
    fn digital_get_dout(&self,node:i32) ->Result<u16> {
        Ok(digital::get_output(node))
    }
    fn digital_set_dout(&self,node:i32,value:u16) -> Result<()> {
        Ok(digital::set_output(node, value))
    }
    fn digital_get_din00(&self,node:i32) ->Result<bool> {
        let din = self.digital_get_din(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(din);
        Ok(bv.get(0).unwrap())
    }
    fn digital_get_din01(&self,node:i32) ->Result<bool> {
        Ok(false)
    }
    fn digital_get_din02(&self,node:i32) ->Result<bool> {
        Ok(false)
    }
    fn digital_get_din03(&self,node:i32) ->Result<bool> {
        Ok(false)
    }
    fn digital_get_din04(&self,node:i32) ->Result<bool> {
        Ok(false)
    }
    fn digital_get_din05(&self,node:i32) ->Result<bool> {
        Ok(false)
    }
    fn digital_get_din06(&self,node:i32) ->Result<bool> {
        Ok(false)
    }
    fn digital_get_din07(&self,node:i32) ->Result<bool> {
        Ok(false)
    }
    fn digital_get_din08(&self,node:i32) ->Result<bool> {
        Ok(false)
    }
    fn digital_get_din09(&self,node:i32) ->Result<bool> {
        Ok(false)
    }
    fn digital_get_din10(&self,node:i32) ->Result<bool> {
        Ok(false)
    }
    fn digital_get_din11(&self,node:i32) ->Result<bool> {
        Ok(false)
    }
    fn digital_get_din12(&self,node:i32) ->Result<bool> {
        Ok(false)
    }
    fn digital_get_din13(&self,node:i32) ->Result<bool> {
        Ok(false)
    }
    fn digital_get_din14(&self,node:i32) ->Result<bool> {
        Ok(false)
    }
    fn digital_get_din15(&self,node:i32) ->Result<bool> {
        Ok(false)
    }
    fn digital_get_dout00(&self,node:i32) ->Result<bool> {
        Ok(false)
    }
    fn digital_get_dout01(&self,node:i32) ->Result<bool> {
        Ok(false)
    }
    fn digital_get_dout02(&self,node:i32) ->Result<bool> {
        Ok(false)
    }
    fn digital_get_dout03(&self,node:i32) ->Result<bool> {
        Ok(false)
    }
    fn digital_get_dout04(&self,node:i32) ->Result<bool> {
        Ok(false)
    }
    fn digital_get_dout05(&self,node:i32) ->Result<bool> {
        Ok(false)
    }
    fn digital_get_dout06(&self,node:i32) ->Result<bool> {
        Ok(false)
    }
    fn digital_get_dout07(&self,node:i32) ->Result<bool> {
        Ok(false)
    }
    fn digital_get_dout08(&self,node:i32) ->Result<bool> {
        Ok(false)
    }
    fn digital_get_dout09(&self,node:i32) ->Result<bool> {
        Ok(false)
    }
    fn digital_get_dout10(&self,node:i32) ->Result<bool> {
        Ok(false)
    }
    fn digital_get_dout11(&self,node:i32) ->Result<bool> {
        Ok(false)
    }
    fn digital_get_dout12(&self,node:i32) ->Result<bool> {
        Ok(false)
    }
    fn digital_get_dout13(&self,node:i32) ->Result<bool> {
        Ok(false)
    }
    fn digital_get_dout14(&self,node:i32) ->Result<bool> {
        Ok(false)
    }
    fn digital_get_dout15(&self,node:i32) ->Result<bool> {
        Ok(false)
    }

    fn digital_set_dout00(&self,node:i32,value:bool) -> Result<()>{
        Ok(())
    }
    fn digital_set_dout01(&self,node:i32,value:bool) -> Result<()>{
        Ok(())
    }
    fn digital_set_dout02(&self,node:i32,value:bool) -> Result<()>{
        Ok(())
    }
    fn digital_set_dout03(&self,node:i32,value:bool) -> Result<()>{
        Ok(())
    }
    fn digital_set_dout04(&self,node:i32,value:bool) -> Result<()>{
        Ok(())
    }
    fn digital_set_dout05(&self,node:i32,value:bool) -> Result<()>{
        Ok(())
    }
    fn digital_set_dout06(&self,node:i32,value:bool) -> Result<()>{
        Ok(())
    }
    fn digital_set_dout07(&self,node:i32,value:bool) -> Result<()>{
        Ok(())
    }
    fn digital_set_dout08(&self,node:i32,value:bool) -> Result<()>{
        Ok(())
    }
    fn digital_set_dout09(&self,node:i32,value:bool) -> Result<()>{
        Ok(())
    }
    fn digital_set_dout10(&self,node:i32,value:bool) -> Result<()>{
        Ok(())
    }
    fn digital_set_dout11(&self,node:i32,value:bool) -> Result<()>{
        Ok(())
    }
    fn digital_set_dout12(&self,node:i32,value:bool) -> Result<()>{
        Ok(())
    }
    fn digital_set_dout13(&self,node:i32,value:bool) -> Result<()>{
        Ok(())
    }
    fn digital_set_dout14(&self,node:i32,value:bool) -> Result<()>{
        Ok(())
    }
    fn digital_set_dout15(&self,node:i32,value:bool) -> Result<()>{
        Ok(())
    }

    fn motor_get_uart01(&self,node:i32)-> Result<Vec<u8>> {
        Ok(vec![0 as u8])
    }
    fn motor_get_uart02(&self,node:i32)-> Result<Vec<u8>> {
       Ok(vec![0 as u8])
    }
    fn motor_set_uart01(&self,node:i32,data:Vec<u8>)-> Result<()> {
        Ok(())
    }
    fn motor_set_uart02(&self,node:i32,data:Vec<u8>)-> Result<()> {
       Ok(())
    }
    fn motor_set_baut01(&self,node:i32,bautrate:u32)-> Result<()> {
        Ok(())
    }
    fn motor_set_baut02(&self,node:i32,bautrate:u32)-> Result<()> {
        Ok(())
    }
    fn analogext_count(&self) -> Result<u8> {
        Ok(analog::get_ext_count())
    }
    fn analogext_get_out(&self, num: u8) -> Result<u16> {
        Ok(analog::get_ext_output(num))
    }
    fn analogext_set_out(&self, num: u8, val: u16) -> Result<()> {
        analog::set_ext_output(num, val);
        Ok(())
    }
}

