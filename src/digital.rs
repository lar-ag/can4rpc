use jsonrpc_core::futures::future::{Future};
use jsonrpc_core::{Result};
use jsonrpc_derive::rpc;

use serde_derive::{Deserialize,Serialize};
// use tide::{error::ResultExt, response, App, Context, EndpointResult};
use bitvec::prelude::*;
use super::bindings as can;
// use lazy_static::lazy_static;
// lazy_static! {
// }


#[derive(Debug,Default, Clone, Serialize, Deserialize)]
pub struct DigitalNode;

use std::fmt::Write;



fn info(node : i32) -> String {
    let mut info = String::new();
    write!(&mut info, "digital:{}",node).unwrap(); // uses fmt::Write::write_fmt
    info
}
fn get_inputs(node:i32) -> u16 {
    unsafe{
        can::digital_get_input(node) as u16
    }
}
fn get_outputs(node:i32) -> u16 {
    unsafe{
        can::digital_get_output(node) as u16
    }
}
fn set_outputs(node:i32, value: u16) {
    unsafe{
        can::digital_set_output(node,value as u32);
    }
}

#[rpc]
pub trait Digital {
    #[rpc(name = "digitalInfo")]
    fn digital_info(&self,node:i32) -> Result<String>;
    #[rpc(name = "digitalGetInputs")]
    fn digital_get_inputs(&self, node:i32) -> Result<u16>;
    #[rpc(name = "digitalGetOutputs")]
    fn digital_get_outputs(&self, node:i32) ->Result<u16>;
    #[rpc(name = "digitalSetOutputs")]
    fn digital_set_outputs(&self,node:i32,value:u16) -> Result<()>;
    #[rpc(name = "digitalGetIn00")]
    fn digital_get_input00(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digitalGetIn01")]
    fn digital_get_input01(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digitalGetIn02")]
    fn digital_get_input02(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digitalGetIn03")]
    fn digital_get_input03(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digitalGetIn04")]
    fn digital_get_input04(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digitalGetIn05")]
    fn digital_get_input05(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digitalGetIn06")]
    fn digital_get_input06(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digitalGetIn07")]
    fn digital_get_input07(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digitalGetIn08")]
    fn digital_get_input08(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digitalGetIn09")]
    fn digital_get_input09(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digitalGetIn10")]
    fn digital_get_input10(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digitalGetIn11")]
    fn digital_get_input11(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digitalGetIn12")]
    fn digital_get_input12(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digitalGetIn13")]
    fn digital_get_input13(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digitalGetIn14")]
    fn digital_get_input14(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digitalGetIn15")]
    fn digital_get_input15(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digitalGetOut00")]
    fn digital_get_output00(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digitalGetOut01")]
    fn digital_get_output01(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digitalGetOut02")]
    fn digital_get_output02(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digitalGetOut03")]
    fn digital_get_output03(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digitalGetOut04")]
    fn digital_get_output04(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digitalGetOut05")]
    fn digital_get_output05(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digitalGetOut06")]
    fn digital_get_output06(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digitalGetOut07")]
    fn digital_get_output07(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digitalGetOut08")]
    fn digital_get_output08(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digitalGetOut09")]
    fn digital_get_output09(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digitalGetOut10")]
    fn digital_get_output10(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digitalGetOut11")]
    fn digital_get_output11(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digitalGetOut12")]
    fn digital_get_output12(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digitalGetOut13")]
    fn digital_get_output13(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digitalGetOut14")]
    fn digital_get_output14(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digitalGetOut15")]
    fn digital_get_output15(&self, node:i32) ->Result<bool>;

    #[rpc(name = "digitalSetOut00")]
    fn digital_set_output00(&self, node:i32,value:bool) -> Result<()>;
    #[rpc(name = "digitalSetOut01")]
    fn digital_set_output01(&self, node:i32,value:bool) -> Result<()>;
    #[rpc(name = "digitalSetOut02")]
    fn digital_set_output02(&self, node:i32,value:bool) -> Result<()>;
    #[rpc(name = "digitalSetOut03")]
    fn digital_set_output03(&self, node:i32,value:bool) -> Result<()>;
    #[rpc(name = "digitalSetOut04")]
    fn digital_set_output04(&self, node:i32,value:bool) -> Result<()>;
    #[rpc(name = "digitalSetOut05")]
    fn digital_set_output05(&self, node:i32,value:bool) -> Result<()>;
    #[rpc(name = "digitalSetOut06")]
    fn digital_set_output06(&self, node:i32,value:bool) -> Result<()>;
    #[rpc(name = "digitalSetOut07")]
    fn digital_set_output07(&self, node:i32,value:bool) -> Result<()>;
    #[rpc(name = "digitalSetOut08")]
    fn digital_set_output08(&self, node:i32,value:bool) -> Result<()>;
    #[rpc(name = "digitalSetOut09")]
    fn digital_set_output09(&self, node:i32,value:bool) -> Result<()>;
    #[rpc(name = "digitalSetOut10")]
    fn digital_set_output10(&self, node:i32,value:bool) -> Result<()>;
    #[rpc(name = "digitalSetOut11")]
    fn digital_set_output11(&self, node:i32,value:bool) -> Result<()>;
    #[rpc(name = "digitalSetOut12")]
    fn digital_set_output12(&self, node:i32,value:bool) -> Result<()>;
    #[rpc(name = "digitalSetOut13")]
    fn digital_set_output13(&self, node:i32,value:bool) -> Result<()>;
    #[rpc(name = "digitalSetOut14")]
    fn digital_set_output14(&self, node:i32,value:bool) -> Result<()>;
    #[rpc(name = "digitalSetOut15")]
    fn digital_set_output15(&self, node:i32,value:bool) -> Result<()>;
}

// use log::info;

/// #r`{"jsonrpc": "2.0", "method": "protocolVersion","id": 1}`
/// #r`{"jsonrpc": "2.0", "method": "digitalGetIn00","id": 1}`
// pub trait DigitalNode {

// }






impl Digital for DigitalNode {

    fn digital_info(&self,node:i32) -> Result<String> {
        Ok(info(node))
    }
    fn digital_get_inputs(&self, node:i32) -> Result<u16> {
        Ok(get_inputs(node))
    }
    fn digital_get_outputs(&self, node:i32) ->Result<u16> {
        Ok(get_outputs(node))
    }
    fn digital_set_outputs(&self, node:i32,value:u16) -> Result<()> {
        Ok(set_outputs(node, value))
    }
    fn digital_get_input00(&self, node:i32) ->Result<bool> {
        println!("server digital::digital_get_input00");
        let din = self.digital_get_inputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(din);
        Ok(bv.get(0).unwrap())
    }
    fn digital_get_input01(&self, node:i32) ->Result<bool> {
        let din = self.digital_get_inputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(din);
        Ok(bv.get(1).unwrap())
    }
    fn digital_get_input02(&self, node:i32) ->Result<bool> {
        let din = self.digital_get_inputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(din);
        Ok(bv.get(2).unwrap())
    }
    fn digital_get_input03(&self, node:i32) ->Result<bool> {
        let din = self.digital_get_inputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(din);
        Ok(bv.get(3).unwrap())
    }
    fn digital_get_input04(&self, node:i32) ->Result<bool> {
        let din = self.digital_get_inputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(din);
        Ok(bv.get(4).unwrap())
    }
    fn digital_get_input05(&self, node:i32) ->Result<bool> {
         let din = self.digital_get_inputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(din);
        Ok(bv.get(5).unwrap())
    }
    fn digital_get_input06(&self, node:i32) ->Result<bool> {
         let din = self.digital_get_inputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(din);
        Ok(bv.get(6).unwrap())
    }
    fn digital_get_input07(&self, node:i32) ->Result<bool> {
        let din = self.digital_get_inputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(din);
        Ok(bv.get(7).unwrap())
    }
    fn digital_get_input08(&self, node:i32) ->Result<bool> {
        let din = self.digital_get_inputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(din);
        Ok(bv.get(8).unwrap())
    }
    fn digital_get_input09(&self, node:i32) ->Result<bool> {
        let din = self.digital_get_inputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(din);
        Ok(bv.get(9).unwrap())
    }
    fn digital_get_input10(&self, node:i32) ->Result<bool> {
        let din = self.digital_get_inputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(din);
        Ok(bv.get(10).unwrap())
    }
    fn digital_get_input11(&self, node:i32) ->Result<bool> {
        let din = self.digital_get_inputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(din);
        Ok(bv.get(11).unwrap())
    }
    fn digital_get_input12(&self, node:i32) ->Result<bool> {
        let din = self.digital_get_inputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(din);
        Ok(bv.get(12).unwrap())
    }
    fn digital_get_input13(&self, node:i32) ->Result<bool> {
        let din = self.digital_get_inputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(din);
        Ok(bv.get(13).unwrap())
    }
    fn digital_get_input14(&self, node:i32) ->Result<bool> {
        let din = self.digital_get_inputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(din);
        Ok(bv.get(14).unwrap())
    }
    fn digital_get_input15(&self, node:i32) ->Result<bool> {
        let din = self.digital_get_inputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(din);
        Ok(bv.get(15).unwrap())
    }
    fn digital_get_output00(&self, node:i32) ->Result<bool> {
        let dout = self.digital_get_outputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(dout);
        Ok(bv.get(0).unwrap())
    }
    fn digital_get_output01(&self, node:i32) ->Result<bool> {
        let dout = self.digital_get_outputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(dout);
        Ok(bv.get(1).unwrap())
    }
    fn digital_get_output02(&self, node:i32) ->Result<bool> {
        let dout = self.digital_get_outputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(dout);
        Ok(bv.get(2).unwrap())
    }
    fn digital_get_output03(&self, node:i32) ->Result<bool> {
        let dout = self.digital_get_outputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(dout);
        Ok(bv.get(3).unwrap())
    }
    fn digital_get_output04(&self, node:i32) ->Result<bool> {
        let dout = self.digital_get_outputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(dout);
        Ok(bv.get(4).unwrap())
    }
    fn digital_get_output05(&self, node:i32) ->Result<bool> {
        let dout = self.digital_get_outputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(dout);
        Ok(bv.get(5).unwrap())
    }
    fn digital_get_output06(&self, node:i32) ->Result<bool> {
        let dout = self.digital_get_outputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(dout);
        Ok(bv.get(6).unwrap())
    }
    fn digital_get_output07(&self, node:i32) ->Result<bool> {
        let dout = self.digital_get_outputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(dout);
        Ok(bv.get(7).unwrap())
    }
    fn digital_get_output08(&self, node:i32) ->Result<bool> {
        let dout = self.digital_get_outputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(dout);
        Ok(bv.get(8).unwrap())
    }
    fn digital_get_output09(&self, node:i32) ->Result<bool> {
        let dout = self.digital_get_outputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(dout);
        Ok(bv.get(9).unwrap())
    }
    fn digital_get_output10(&self, node:i32) ->Result<bool> {
        let dout = self.digital_get_outputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(dout);
        Ok(bv.get(10).unwrap())
    }
    fn digital_get_output11(&self, node:i32) ->Result<bool> {
        let dout = self.digital_get_outputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(dout);
        Ok(bv.get(11).unwrap())
    }
    fn digital_get_output12(&self, node:i32) ->Result<bool> {
        let dout = self.digital_get_outputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(dout);
        Ok(bv.get(12).unwrap())
    }
    fn digital_get_output13(&self, node:i32) ->Result<bool> {
        let dout = self.digital_get_outputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(dout);
        Ok(bv.get(13).unwrap())
    }
    fn digital_get_output14(&self, node:i32) ->Result<bool> {
        let dout = self.digital_get_outputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(dout);
        Ok(bv.get(14).unwrap())
    }
    fn digital_get_output15(&self, node:i32) ->Result<bool> {
        let dout = self.digital_get_outputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(dout);
        Ok(bv.get(15).unwrap())
    }

    fn digital_set_output00(&self, node:i32,value:bool) -> Result<()>{
        let mut dout:u16 = self.digital_get_outputs(node)?;
        let bv:&mut BitSlice<BigEndian,u16> = dout.as_mut_bitslice();
        bv.set(0,value);
        self.digital_set_outputs(node,dout)
    }
    fn digital_set_output01(&self, node:i32,value:bool) -> Result<()>{
        let mut dout:u16 = self.digital_get_outputs(node)?;
        let bv:&mut BitSlice<BigEndian,u16> = dout.as_mut_bitslice();
        bv.set(1,value);
        self.digital_set_outputs(node,dout)
    }
    fn digital_set_output02(&self, node:i32,value:bool) -> Result<()>{
        let mut dout:u16 = self.digital_get_outputs(node)?;
        let bv:&mut BitSlice<BigEndian,u16> = dout.as_mut_bitslice();
        bv.set(2,value);
        self.digital_set_outputs(node,dout)
    }
    fn digital_set_output03(&self, node:i32,value:bool) -> Result<()>{
        let mut dout:u16 = self.digital_get_outputs(node)?;
        let bv:&mut BitSlice<BigEndian,u16> = dout.as_mut_bitslice();
        bv.set(3,value);
        self.digital_set_outputs(node,dout)
    }
    fn digital_set_output04(&self, node:i32,value:bool) -> Result<()>{
        let mut dout:u16 = self.digital_get_outputs(node)?;
        let bv:&mut BitSlice<BigEndian,u16> = dout.as_mut_bitslice();
        bv.set(4,value);
        self.digital_set_outputs(node,dout)
    }
    fn digital_set_output05(&self, node:i32,value:bool) -> Result<()>{
        let mut dout:u16 = self.digital_get_outputs(node)?;
        let bv:&mut BitSlice<BigEndian,u16> = dout.as_mut_bitslice();
        bv.set(5,value);
        self.digital_set_outputs(node,dout)
    }
    fn digital_set_output06(&self, node:i32,value:bool) -> Result<()>{
        let mut dout:u16 = self.digital_get_outputs(node)?;
        let bv:&mut BitSlice<BigEndian,u16> = dout.as_mut_bitslice();
        bv.set(6,value);
        self.digital_set_outputs(node,dout)
    }
    fn digital_set_output07(&self, node:i32,value:bool) -> Result<()>{
        let mut dout:u16 = self.digital_get_outputs(node)?;
        let bv:&mut BitSlice<BigEndian,u16> = dout.as_mut_bitslice();
        bv.set(7,value);
        self.digital_set_outputs(node,dout)
    }
    fn digital_set_output08(&self, node:i32,value:bool) -> Result<()>{
        let mut dout:u16 = self.digital_get_outputs(node)?;
        let bv:&mut BitSlice<BigEndian,u16> = dout.as_mut_bitslice();
        bv.set(8,value);
        self.digital_set_outputs(node,dout)
    }
    fn digital_set_output09(&self, node:i32,value:bool) -> Result<()>{
        let mut dout:u16 = self.digital_get_outputs(node)?;
        let bv:&mut BitSlice<BigEndian,u16> = dout.as_mut_bitslice();
        bv.set(9,value);
        self.digital_set_outputs(node,dout)
    }
    fn digital_set_output10(&self, node:i32,value:bool) -> Result<()>{
        let mut dout:u16 = self.digital_get_outputs(node)?;
        let bv:&mut BitSlice<BigEndian,u16> = dout.as_mut_bitslice();
        bv.set(10,value);
        self.digital_set_outputs(node,dout)
    }
    fn digital_set_output11(&self, node:i32,value:bool) -> Result<()>{
        let mut dout:u16 = self.digital_get_outputs(node)?;
        let bv:&mut BitSlice<BigEndian,u16> = dout.as_mut_bitslice();
        bv.set(11,value);
        self.digital_set_outputs(node,dout)
    }
    fn digital_set_output12(&self, node:i32,value:bool) -> Result<()>{
        let mut dout:u16 = self.digital_get_outputs(node)?;
        let bv:&mut BitSlice<BigEndian,u16> = dout.as_mut_bitslice();
        bv.set(12,value);
        self.digital_set_outputs(node,dout)
    }
    fn digital_set_output13(&self, node:i32,value:bool) -> Result<()>{
        let mut dout:u16 = self.digital_get_outputs(node)?;
        let bv:&mut BitSlice<BigEndian,u16> = dout.as_mut_bitslice();
        bv.set(13,value);
        self.digital_set_outputs(node,dout)
    }
    fn digital_set_output14(&self, node:i32,value:bool) -> Result<()>{
        let mut dout:u16 = self.digital_get_outputs(node)?;
        let bv:&mut BitSlice<BigEndian,u16> = dout.as_mut_bitslice();
        bv.set(14,value);
        self.digital_set_outputs(node,dout)
    }
    fn digital_set_output15(&self, node:i32,value:bool) -> Result<()>{
        let mut dout:u16 = self.digital_get_outputs(node)?;
        let bv:&mut BitSlice<BigEndian,u16> = dout.as_mut_bitslice();
        bv.set(15,value);
        self.digital_set_outputs(node,dout)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn digital_node_test(node:i32) {
        unsafe{
            let din = get_inputs(node);
            println!("DIGITAL-IN:{:}",din);
            assert_eq!(din,0);
        }
    }
    #[test]
    fn digital_test() {
        digital_node_test(0x18);
        // digital_node_test(0x19);
        // digital_node_test(0x1a);
    }
}
