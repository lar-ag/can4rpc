use jsonrpc_core::futures::future::{Future};
use jsonrpc_core::{Result};
// use jsonrpc_core_client::transports::local;
use jsonrpc_derive::rpc;

use std::fmt::Write;

use super::can;

// lazy_static! {

// }


fn info(node:i32) -> String {
    let mut info = String::new();
    write!(&mut info, "AnalogOut node {}",node).unwrap(); // uses fmt::Write::write_fmt
    info
}


fn analog_outs_count()-> u8 {
    unsafe{
        can::analogext_get_count() as u8
    }
}
fn analog_outs_get(num:u8)-> u16 {
    unsafe{
        can::analogext_get_out(num) as u16
    }
}
fn analog_outs_set(num:u8,value:u16) {
    unsafe{
        can::analogext_set_out(num,value as u32);
    }
}

#[rpc]
pub trait AOuts {
    #[rpc(name = "analogOutsCount")]
    fn analog_outs_count(&self) -> Result<u8>;
    #[rpc(name = "analogOutsGet")]
    fn analog_outs_get(&self, num: u8) -> Result<u16>;
    #[rpc(name = "analogOutsSet")]
    fn analog_outs_set(&self, num: u8, val: u16) -> Result<()>;
}


pub struct AOutsNode;

impl AOuts for AOutsNode {
    fn analog_outs_count(&self) -> Result<u8> {
        Ok(analog_outs_count())
    }
    fn analog_outs_get(&self, num: u8) -> Result<u16> {
        Ok(analog_outs_get(num))
    }
    fn analog_outs_set(&self, num: u8, val: u16) -> Result<()> {
        analog_outs_set(num, val);
        Ok(())
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
