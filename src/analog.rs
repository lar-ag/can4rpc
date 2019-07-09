use std::fmt::Write;
use std::ptr;

// use lazy_static::lazy_static;

// use tide::{error::ResultExt, response, App, Context, EndpointResult};
// use http::status::StatusCode;
// use super::MioError;
use super::can;

// lazy_static! {

// }


pub fn info(node:i32) -> String {
    let mut info = String::new();
    write!(&mut info, "Analog node {}",node).unwrap(); // uses fmt::Write::write_fmt
    info
}

pub fn get_input01(node : i32) -> u16{
    unsafe{
        can::analog_get_in01(node) as u16
    }
}

pub fn get_input02(node:i32) -> u16 {
    unsafe{
        can::analog_get_in02(node) as u16
    }
}

pub fn get_input03(node:i32) -> u16{
    unsafe{
        can::analog_get_in03(node) as u16
    }
}

pub fn get_input04(node :i32) -> u16{
    unsafe{
        can::analog_get_in04(node) as u16
    }
}

pub fn get_input05(node:i32) -> u16{
    unsafe{
        can::analog_get_in05(node) as u16
    }
}
pub fn get_out(node:i32) -> u16{
    unsafe{
        can::analog_get_out(node) as u16
    }
}
pub fn set_out(node:i32,value:u16)  {
    unsafe{
        can::analog_set_out(node,value as u32);
    }
}
pub fn get_temp01(node:i32) -> u16 {
    unsafe{
       can::analog_get_temp01(node)as u16
    }
}
pub fn get_temp02(node:i32) -> u16 {
    unsafe{
      can::analog_get_temp02(node) as u16
    }
}
pub fn get_temp03(node:i32) -> u16 {
    unsafe{
      can::analog_get_temp03(node) as u16
    }
}


pub fn get_uart01(node:i32) -> Vec<u8>{
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
pub fn set_uart01(node:i32,mut value: Vec<u8>)    {
    unsafe{
        can::analog_set_uart01(node,value.as_mut_ptr());
    }
}
pub fn set_bautrate01(node:i32,baut:u32)   {
    unsafe{
      can::analog_set_baut01(node,baut);
    }
}
pub fn get_uart02(node:i32) -> Vec<u8> {
    unsafe{
        let data = can::analog_get_uart01(node);
        let len:usize = data.len as usize;
        let mut vec = data.buf.to_vec();
        vec.truncate(len);
    //    Vec::from_raw_parts(&buf.buf,buf.len as usize,256)
        vec
    }
}
pub fn set_uart02(node:i32,mut value: Vec<u8>)    {
    unsafe{
      can::analog_set_uart02(node,value.as_mut_ptr());
    }
}
pub fn set_bautrate02(node:i32, baut: u32)  {
    unsafe{
     can::analog_set_baut02(node,baut);
    }
}
pub fn get_ext_count()-> u8 {
    unsafe{
        can::analogext_get_count() as u8
    }
}
pub fn get_ext_output(num:u8)-> u16 {
    unsafe{
        can::analogext_get_out(num) as u16
    }
}
pub fn set_ext_output(num:u8,value:u16) {
    unsafe{
        can::analogext_set_out(num,value as u32);
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
