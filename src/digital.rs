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
    #[rpc(name = "digital_info")]
    fn digital_info(&self,node:i32) -> Result<String>;
    #[rpc(name = "digital_get_inputs")]
    fn digital_get_inputs(&self, node:i32) -> Result<u16>;
    #[rpc(name = "digital_get_outputs")]
    fn digital_get_outputs(&self, node:i32) ->Result<u16>;
    #[rpc(name = "digital_set_outputs")]
    fn digital_set_outputs(&self,node:i32,value:u16) -> Result<()>;
    #[rpc(name = "digital_get_in00")]
    fn digital_get_input00(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_in01")]
    fn digital_get_input01(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_in02")]
    fn digital_get_input02(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_in03")]
    fn digital_get_input03(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_in04")]
    fn digital_get_input04(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_in05")]
    fn digital_get_input05(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_in06")]
    fn digital_get_input06(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_in07")]
    fn digital_get_input07(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_in08")]
    fn digital_get_input08(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_in09")]
    fn digital_get_input09(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_in10")]
    fn digital_get_input10(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_in11")]
    fn digital_get_input11(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_in12")]
    fn digital_get_input12(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_in13")]
    fn digital_get_input13(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_in14")]
    fn digital_get_input14(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_in15")]
    fn digital_get_input15(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_out00")]
    fn digital_get_output00(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_out01")]
    fn digital_get_output01(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_out02")]
    fn digital_get_output02(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_out03")]
    fn digital_get_output03(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_out04")]
    fn digital_get_output04(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_out05")]
    fn digital_get_output05(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_out06")]
    fn digital_get_output06(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_out07")]
    fn digital_get_output07(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_out08")]
    fn digital_get_output08(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_out09")]
    fn digital_get_output09(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_out10")]
    fn digital_get_output10(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_out11")]
    fn digital_get_output11(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_out12")]
    fn digital_get_output12(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_out13")]
    fn digital_get_output13(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_out14")]
    fn digital_get_output14(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_out15")]
    fn digital_get_output15(&self, node:i32) ->Result<bool>;

    #[rpc(name = "digital_set_out00")]
    fn digital_set_output00(&self, node:i32,value:bool) -> Result<()>;
    #[rpc(name = "digital_set_out01")]
    fn digital_set_output01(&self, node:i32,value:bool) -> Result<()>;
    #[rpc(name = "digital_set_out02")]
    fn digital_set_output02(&self, node:i32,value:bool) -> Result<()>;
    #[rpc(name = "digital_set_out03")]
    fn digital_set_output03(&self, node:i32,value:bool) -> Result<()>;
    #[rpc(name = "digital_set_out04")]
    fn digital_set_output04(&self, node:i32,value:bool) -> Result<()>;
    #[rpc(name = "digital_set_out05")]
    fn digital_set_output05(&self, node:i32,value:bool) -> Result<()>;
    #[rpc(name = "digital_set_out06")]
    fn digital_set_output06(&self, node:i32,value:bool) -> Result<()>;
    #[rpc(name = "digital_set_out07")]
    fn digital_set_output07(&self, node:i32,value:bool) -> Result<()>;
    #[rpc(name = "digital_set_out08")]
    fn digital_set_output08(&self, node:i32,value:bool) -> Result<()>;
    #[rpc(name = "digital_set_out09")]
    fn digital_set_output09(&self, node:i32,value:bool) -> Result<()>;
    #[rpc(name = "digital_set_out10")]
    fn digital_set_output10(&self, node:i32,value:bool) -> Result<()>;
    #[rpc(name = "digital_set_out11")]
    fn digital_set_output11(&self, node:i32,value:bool) -> Result<()>;
    #[rpc(name = "digital_set_out12")]
    fn digital_set_output12(&self, node:i32,value:bool) -> Result<()>;
    #[rpc(name = "digital_set_out13")]
    fn digital_set_output13(&self, node:i32,value:bool) -> Result<()>;
    #[rpc(name = "digital_set_out14")]
    fn digital_set_output14(&self, node:i32,value:bool) -> Result<()>;
    #[rpc(name = "digital_set_out15")]
    fn digital_set_output15(&self, node:i32,value:bool) -> Result<()>;
}

// use log::info;

///Digital1 #r`{"jsonrpc": "2.0", "method": "digital_info","params":[24],"id": 1}`
///Digital1 #r`{"jsonrpc": "2.0", "method": "digital_get_in00","params":[24],"id": 1}`
///Digital2 #r`{"jsonrpc": "2.0", "method": "digital_get_in00","params":[25],"id": 1}`
///
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
        println!("Digital::digital_get_input00");
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
