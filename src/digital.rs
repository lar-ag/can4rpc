#![feature(async_await)]
use std::os::raw::*;
use serde_derive::{Deserialize,Serialize};
use tide::{error::ResultExt, response, App, Context, EndpointResult};
use http::status::StatusCode;

use lazy_static::lazy_static;

lazy_static! {

}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DigitalNode {
  pub node: u32,
  pub input: u32,
  pub output: u32,
}


extern {
  fn digital_read_unsigned  ( node: c_int, index: c_int, sub: c_uchar)->c_uint;
  fn digital_write_unsigned ( node: c_int, index: c_int, sub: c_uchar, type_: c_uchar, value: c_uint);
  fn digital_get_input      ( node:c_int) -> c_uint;
  fn digital_get_output     ( node:c_int) -> c_uint;
  fn digital_set_output     ( node:c_int,value: c_uint);
}



pub struct Can;



use std::fmt::Write;




pub async fn info(cx: Context<()>) -> EndpointResult<String> {
    let node:i32 = cx.param("id").client_err()?;
    let mut info = String::new();
    write!(&mut info, "Digital:{}",node).unwrap(); // uses fmt::Write::write_fmt
    Ok(info)
}
pub async fn get_input(cx: Context<()>) -> EndpointResult {
    let node:i32 = cx.param("id").client_err()?;
    unsafe{
         Ok(response::json(digital_get_input(node)))
    }
}
pub async fn get_output(cx: Context<()>) -> EndpointResult {
    let node:i32 = cx.param("id").client_err()?;
    unsafe{
       Ok(response::json(digital_get_output(node)))
    }
}
pub async fn set_output(mut cx: Context<()>) -> EndpointResult<()>  {
    let node:i32 = cx.param("id").client_err()?;
    let value:u32 = cx.body_json().await.client_err()?;
    unsafe{
        digital_set_output(node,value);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn digital_node_test(node:i32) {
       unsafe{
          let din:u32 = digital_get_input(node);
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
