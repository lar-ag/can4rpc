use std::os::raw::*;
use serde_derive::{Deserialize,Serialize};
// use tide::{error::ResultExt, response, App, Context, EndpointResult};
use super::bindings as can;
use lazy_static::lazy_static;

lazy_static! {

}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DigitalNode {
  pub node: u32,
  pub input: u32,
  pub output: u32,
}


use std::fmt::Write;




pub fn info(node : i32) -> String {
    let mut info = String::new();
    write!(&mut info, "Digital:{}",node).unwrap(); // uses fmt::Write::write_fmt
    info
}
pub fn get_input(node:i32) -> u16 {
    unsafe{
        can::digital_get_input(node) as u16
    }
}
pub fn get_output(node:i32) -> u16 {
    unsafe{
        can::digital_get_output(node) as u16
    }
}
pub fn set_output(node:i32, value: u16) {
    unsafe{
        can::digital_set_output(node,value as u32);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn digital_node_test(node:i32) {
       unsafe{
          let din = get_input(node);
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
