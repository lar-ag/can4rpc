use std::os::raw::*;
use std::fmt::Write;

use lazy_static::lazy_static;

use tide::{error::ResultExt, response, App, Context, EndpointResult};
use http::status::StatusCode;

lazy_static! {

}
// use super::can0::CAN;
// use super::can::{
    // Message,
//  };






extern {
  // fn read_unsigned    (node: c_int, index: c_int, sub: c_uchar)->c_uint;
  // fn write_unsigned   (node: c_int, index: c_int, sub: c_uchar, type_: c_uchar, value: c_uint);
  fn analog_get_uart01(node:c_int) -> *mut c_uchar;
  fn analog_get_uart02(node:c_int) -> *mut c_uchar;
  fn analog_set_uart02(node:c_int,data: *mut c_uchar) -> c_int;
  fn analog_set_uart01(node:c_int,data: *mut c_uchar) -> c_int;

  fn analog_set_baut01(node:c_int,val:c_uint)->c_int;
  fn analog_set_baut02(node:c_int,val:c_uint)->c_uint;
  fn analog_get_in01  (node:c_int) -> c_uint;
  fn analog_get_in02  (node:c_int) -> c_uint;
  fn analog_get_in03  (node:c_int) -> c_uint;
  fn analog_get_in04  (node:c_int) -> c_uint;
  fn analog_get_in05  (node:c_int) -> c_uint;
  fn analog_get_out   (node:c_int) -> c_uint;
  fn analog_set_out   (node:c_int,val:c_uint)->c_int;
  fn analog_get_temp01(node:c_int) -> c_uint;
  fn analog_get_temp02(node:c_int) -> c_uint;
  fn analog_get_temp03(node:c_int) -> c_uint;
}







pub async fn info(cx: Context<()>) -> EndpointResult<String> {
     let node:i32 = cx.param("id").client_err()?;
    let mut info = String::new();
    write!(&mut info, "Digital:{}",node).unwrap(); // uses fmt::Write::write_fmt
    Ok(info)

}
// GET /analog/:node/in01
pub async fn get_input01(cx: Context<()>) -> EndpointResult{
    let node:i32 = cx.param("id").client_err()?;
    unsafe{
      Ok(response::json(analog_get_in01(node)))
    }
}
pub async fn get_input02(cx: Context<()>) -> EndpointResult{
    let node:i32 = cx.param("id").client_err()?;
    unsafe{
      Ok(response::json(analog_get_in02(node)))
    }
}
pub async fn get_input03(cx: Context<()>) -> EndpointResult{
    let node:i32 = cx.param("id").client_err()?;
    unsafe{
      Ok(response::json(analog_get_in03(node)))
    }
}
pub async fn get_input04(cx: Context<()>) -> EndpointResult{
    let node:i32 = cx.param("id").client_err()?;
    unsafe{
        Ok(response::json(analog_get_in04(node)))
    }
}
pub async fn get_input05(cx: Context<()>) -> EndpointResult{
    let node:i32 = cx.param("id").client_err()?;
    unsafe{
      Ok(response::json(analog_get_in05(node)))
    }
}
pub async fn get_out(cx: Context<()>) -> EndpointResult{
    let node:i32 = cx.param("id").client_err()?;
    unsafe{
        Ok(response::json(analog_get_out(node)))
    }
}
pub async fn set_out(mut cx: Context<()>) -> EndpointResult<()> {
    let node:i32 =  cx.param("id").client_err()?;
    let value:u32 = cx.body_json().await.client_err()?;
    unsafe{
        analog_set_out(node,value);
    }
    Ok(())
}
pub async fn get_temp01(cx: Context<()>) -> EndpointResult {
    let node:i32 = cx.param("id").client_err()?;
    unsafe{
       Ok(response::json(analog_get_temp01(node)))
    }
}
pub async fn get_temp02(cx: Context<()>) -> EndpointResult {
    let node:i32 = cx.param("id").client_err()?;
    unsafe{
      Ok(response::json(analog_get_temp02(node)))
    }
}
pub async fn get_temp03(cx: Context<()>) -> EndpointResult {
    let node:i32 = cx.param("id").client_err()?;
    unsafe{
      Ok(response::json(analog_get_temp03(node)))
    }
}
pub async fn get_uart01(cx: Context<()>) -> EndpointResult {
    let node:i32 = cx.param("id").client_err()?;
    let uart:Vec<u8> = Vec::new();
    unsafe{
      let t = analog_get_uart01(node);
    }
    // uart.from(analog_get_uart01(node));
    Ok(response::json(uart.clone()))
}
pub async fn set_uart01(mut cx: Context<()>)  -> EndpointResult<()>  {
    let node:i32 = cx.param("id").client_err()?;
    let mut value:Vec<u8> = cx.body_json().await.client_err()?;
    unsafe{
        analog_set_uart01(node,value.as_mut_ptr());
    }
    Ok(())
}
pub async fn setup_uart01(mut cx: Context<()>) -> EndpointResult<()> {
    let node:i32 = cx.param("id").client_err()?;
    let baut:u32 = cx.body_json().await.client_err()?;
    unsafe{
      analog_set_baut01(node,baut);
    }
    Ok(())
}
pub async fn get_uart02(cx: Context<()>) -> EndpointResult {
    let node:i32 = cx.param("id").client_err()?;
    let uart:Vec<u8> = Vec::new();
    unsafe{
      let t = analog_get_uart02(node);
    }
    Ok(response::json(uart.clone()))
}
pub async fn set_uart02(mut cx: Context<()>) -> EndpointResult<()> {
    let node:i32 = cx.param("id").client_err()?;
    let mut value:Vec<u8> = cx.body_json().await.client_err()?;
    unsafe{
      analog_set_uart02(node,value.as_mut_ptr());
    }
    Ok(())
}
pub async fn setup_uart02(mut cx: Context<()>) -> EndpointResult<()>  {
    let node:i32 = cx.param("id").client_err()?;
    let baut:u32 = cx.body_json().await.client_err()?;
    unsafe{
      analog_set_baut02(node,baut);
    }
    Ok(())
}





#[cfg(test)]
mod tests {
    use super::*;
    use std::os::raw::*;
    #[test]
    fn canc_test() {
      unsafe{
        let in01:u32 = analog_get_in01(2);
        println!("ANALOG IN01:{:}",in01);
        assert_eq!(in01,887);
      }
    }
}
