#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![feature(async_await)]
use std::os::raw::*;




extern {
  fn read_unsigned    (node: c_int, index: c_int, sub: c_uchar)->c_uint;
  fn write_unsigned   (node: c_int, index: c_int, sub: c_uchar, type_: c_uchar, value: c_uint);

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
//

//end
