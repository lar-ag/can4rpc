use jsonrpc_core::futures::future::{Future};
use jsonrpc_core::{Result};
// use jsonrpc_core_client::transports::local;
use jsonrpc_derive::rpc;

use std::fmt::Write;

use super::can;


fn info(node:i32) -> String {
    let mut info = String::new();
    write!(&mut info, "Analog node {}",node).unwrap(); // uses fmt::Write::write_fmt
    info
}

fn node_unsigned(node : i32,index: u16,sub: u8) -> <u32,CanError>{
    unsafe{
        can::analog_get_in01(node) as u16
    }
}
fn set_(node : i32,index: u16,sub: u8,value: u32) -> u32{
    unsafe{
        can::analog_get_in01(node) as u16
    }
}
fn set_u32(node : i32,index: u16,sub: u8,value: u32) -> u32{
    unsafe{
        can::analog_get_in01(node) as u16
    }
}
fn set_u16(node : i32,index: u16,sub: u8, value: u16) -> u32{
    unsafe{
        can::analog_get_in01(node) as u16
    }
}


fn get_input02(node:i32) -> u16 {
    unsafe{
        can::analog_get_in02(node) as u16
    }
}

fn get_input03(node:i32) -> u16{
    unsafe{
        can::analog_get_in03(node) as u16
    }
}

fn get_input04(node :i32) -> u16{
    unsafe{
        can::analog_get_in04(node) as u16
    }
}

fn get_input05(node:i32) -> u16{
    unsafe{
        can::analog_get_in05(node) as u16
    }
}
fn get_out(node:i32) -> u16{
    unsafe{
        can::analog_get_out(node) as u16
    }
}
fn set_out(node:i32,value:u16)  {
    unsafe{
        can::analog_set_out(node,value as u32);
    }
}
fn get_temp01(node:i32) -> u16 {
    unsafe{
       can::analog_get_temp01(node)as u16
    }
}
fn get_temp02(node:i32) -> u16 {
    unsafe{
      can::analog_get_temp02(node) as u16
    }
}
fn get_temp03(node:i32) -> u16 {
    unsafe{
      can::analog_get_temp03(node) as u16
    }
}


fn get_uart01(node:i32) -> Vec<u8>{
    // let data:
    unsafe{
        let data = can::analog_get_uart01(node);
        let len:usize = data.len as usize;
        let mut vec = data.buf.to_vec();
        vec.truncate(len);
    //    Vec::from_raw_parts(&buf.buf,buf.len as usize,256)
        vec

    }
    // uart.from(analog_get_uart01(node));
}
fn set_uart01(node:i32,mut value: Vec<u8>)    {
    unsafe{
        can::analog_set_uart01(node,value.as_mut_ptr());
    }
}
fn get_uart02(node:i32) -> Vec<u8> {
    unsafe{
        let data = can::analog_get_uart01(node);
        let len:usize = data.len as usize;
        let mut vec = data.buf.to_vec();
        vec.truncate(len);
    //    Vec::from_raw_parts(&buf.buf,buf.len as usize,256)
        vec
    }
}
fn set_uart02(node:i32,mut value: Vec<u8>)    {
    unsafe{
      can::analog_set_uart02(node,value.as_mut_ptr());
    }
}


#[rpc]
pub trait Analog {
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
}


pub struct AnalogNode;

impl Analog for AnalogNode {
      fn analog_get_in01(&self) -> Result<u16> {
        Ok(get_input01(0x2))
    }
    fn analog_get_in02(&self) -> Result<u16> {
        Ok(get_input02(0x2))
    }
    fn analog_get_in03(&self) -> Result<u16> {
        Ok(get_input03(0x2))
    }
    fn analog_get_in04(&self) -> Result<u16> {
        Ok(get_input04(0x2))
    }
    fn analog_get_in05(&self) -> Result<u16> {
        Ok(get_input05(0x2))
    }
    fn analog_get_out(&self) -> Result<u16>{
        Ok(get_out(0x2))
    }
    fn analog_set_out(&self,value: u16) -> Result<()>{
        set_out(0x2,value);
        Ok(())
    }
    fn analog_get_temp01(&self) -> Result<u16>{
        Ok(get_temp01(2))
    }
    fn analog_get_temp02(&self) -> Result<u16>{
        Ok(get_temp02(2))
    }
    fn analog_get_temp03(&self) -> Result<u16>{
        Ok(get_temp03(2))
    }
    fn analog_get_uart01(&self) -> Result<Vec<u8>>{
        Ok(get_uart01(2))
    }
    fn analog_get_uart02(&self) -> Result<Vec<u8>>{
        Ok(get_uart02(2))
    }
    fn analog_set_uart01(&self, data: Vec<u8>) -> Result<()>{
        Ok(set_uart01(0x2,data))
    }
    fn analog_set_uart02(&self, data: Vec<u8>) -> Result<()>{
        Ok(set_uart02(0x2,data))
    }
}

#[cfg(test)]
mod tests {
    #![feature(async_await)]
    use super::*;
    #[test]
    fn analogasync_test() {

        // println!("ANALOG IN01:{:}",in01);
        // assert_eq!(in01,887);
    }
}
