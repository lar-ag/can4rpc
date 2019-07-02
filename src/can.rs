#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![feature(async_await)]


mod error;
mod dbus;
mod rpc;
mod mio;
mod pcan;
pub mod bindings;
// mod analog;
// pub use analog::AIN;


// pub use mio::*;
// pub use pcan::*;
// pub use simulation as io;



mod simulation;
// mod pump;
// pub use sensor::*;
// pub use control::*;

pub use self::error::MioError;
pub use self::mio::*;
pub use self::rpc::*;
pub use self::dbus::*;

extern {
  fn analog_get_in01();
  fn close_can0();
  fn read_unsigned();
  fn write_unsigned(int node,int index,U8 sub, TYPE type, U32 value);
  fn analog_get_uart01();
  fn analog_set_baut01();
  fn analog_set_uart01();
  fn analog_get_uart02();
  fn analog_set_baut02();
  fn analog_set_uart02();
  fn analog_get_in01  ();
  fn analog_get_in02  ();
  fn analog_get_in03  ();
  fn analog_get_in04  ();
  fn analog_get_in05  ();
  fn analog_get_out   ();
  fn analog_set_out   ();
  fn analog_get_temp01();
  fn analog_get_temp02();
  fn analog_get_temp03();
}




#[cfg(test)]
mod tests {
  use super::bindings::*;
    #[test]
    fn canc_test() {
      unsafe{
        let in01 = analog_get_in01(2);
        println!("ANALOG IN01:{}",in01);
        assert_eq!(in01,887);
      }
    }
}
//

//end
