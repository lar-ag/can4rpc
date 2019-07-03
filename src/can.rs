#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![feature(async_await)]
use std::os::raw::*;




extern {
  fn read_unsigned    (node: c_int, index: c_int, sub: c_uchar)->c_uint;
  fn write_unsigned   (node: c_int, index: c_int, sub: c_uchar, type_: c_uchar, value: c_uint);
  fn analog_get_uart01(node:c_int) -> *mut c_char;
  fn analog_get_uart02(node:c_int) -> *mut c_char;
  fn analog_set_uart02(node:c_int,data: *mut c_char) -> c_int;
  fn analog_set_uart01(node:c_int,data: *mut c_char) -> c_int;

  fn analog_set_baut01(node:c_int,val:c_uint);
  fn analog_set_baut02(node:c_int);
  fn analog_get_in01  (node:c_int) -> c_uint;
  fn analog_get_in02  (node:c_int) -> c_uint;
  fn analog_get_in03  (node:c_int) -> c_uint;
  fn analog_get_in04  (node:c_int) -> c_uint;
  fn analog_get_in05  (node:c_int) -> c_uint;
  fn analog_get_out   (node:c_int) -> c_uint;
  fn analog_set_out   (node:c_int,val:c_uint);
  fn analog_get_temp01(node:c_int);
  fn analog_get_temp02(node:c_int);
  fn analog_get_temp03(node:c_int);
}



pub struct Can;





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
//

//end
