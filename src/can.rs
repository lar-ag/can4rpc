#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
// #![feature(async_await)]
use std::os::raw::*;
use




// extern {
    // pub fn read_unsigned    (node: c_int, index: c_int, sub: c_uchar)->c_uint;
    // pub fn write_unsigned   (node: c_int, index: c_int, sub: c_uchar, type_: c_uchar, value: c_uint);
    // pub fn read_uart        (node: c_int, index: c_int, sub: c_uchar) -> *mut c_uchar;
    // pub fn write_uart       (node: c_int, index: c_int, sub: c_uchar, data: *mut c_uchar) -> c_int;
    // // fn read_unsigned    (node: c_int, index: c_int, sub: c_uchar)->c_uint;
    // // fn write_unsigned   (node: c_int, index: c_int, sub: c_uchar, type_: c_uchar, value: c_uint);
    // pub fn analog_get_uart01(node:c_int) -> *mut c_uchar;
    // pub fn analog_get_uart02(node:c_int) -> *mut c_uchar;
    // pub fn analog_set_uart02(node:c_int,data: *mut c_uchar) -> c_int;
    // pub fn analog_set_uart01(node:c_int,data: *mut c_uchar) -> c_int;

    // pub fn analog_set_baut01(node:c_int,val:c_uint)->c_int;
    // pub fn analog_set_baut02(node:c_int,val:c_uint)->c_uint;
    // pub fn analog_get_in01  (node:c_int) -> c_uint;
    // pub fn analog_get_in02  (node:c_int) -> c_uint;
    // pub fn analog_get_in03  (node:c_int) -> c_uint;
    // pub fn analog_get_in04  (node:c_int) -> c_uint;
    // pub fn analog_get_in05  (node:c_int) -> c_uint;
    // pub fn analog_get_out   (node:c_int) -> c_uint;
    // pub fn analog_set_out   (node:c_int,val:c_uint)->c_int;
    // pub fn analog_get_temp01(node:c_int) -> c_uint;
    // pub fn analog_get_temp02(node:c_int) -> c_uint;
    // pub fn analog_get_temp03(node:c_int) -> c_uint;

    // pub fn digital_read_unsigned  ( node: c_int, index: c_int, sub: c_uchar)->c_uint;
    // pub fn digital_write_unsigned ( node: c_int, index: c_int, sub: c_uchar, type_: c_uchar, value: c_uint);
    // pub fn digital_get_input      ( node:c_int) -> c_uint;
    // pub fn digital_get_output     ( node:c_int) -> c_uint;
    // pub fn digital_set_output     ( node:c_int,value: c_uint);


    // pub fn doppelmotor_get_uart01(node:c_int) -> *mut c_uchar;
    // pub fn doppelmotor_set_baut01(node:c_int,val:c_uint)->c_int;
    // pub fn doppelmotor_set_uart01(node:c_int,data: *mut c_uchar) -> c_int;
    // pub fn doppelmotor_get_uart02(node:c_int) -> *mut c_uchar;
    // pub fn doppelmotor_set_uart02(node:c_int,data: *mut c_uchar) -> c_int;
    // pub fn doppelmotor_set_baut02(node:c_int,val:c_uint)->c_uint;

    // pub fn analogext_get_count() -> c_uchar;
    // pub fn analogext_get_out(out: c_uchar) -> c_uint;
    // pub fn analogext_set_out(out: c_uchar, value: c_uint) -> c_int;
// }








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
