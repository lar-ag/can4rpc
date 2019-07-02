


// use futures::prelude::*;
// use serde_derive::{Deserialize,Serialize};
use lazy_static::lazy_static;
// use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
// use futures::prelude::*;
// use futures_timer::Delay;

lazy_static! {

}
// use super::can0::CAN;
use super::can::{
    Message,
 };

use super::error::CanError;

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct AnalogState {
//     pub node:   u32,
//     /// AnalogIn 6111
//     pub input:  [u16;5],
//     /// AnalogOut 6120:1
//     pub output:  u16,
//     pub uart01: Vec<u8>,
//     pub uart02: Vec<u8>,
//     pub temperatur: [u16;3],
// }

pub type AInput16 = u16;
/// AnalogNode
// #[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalogNode {

    pub node:   u32,
    pub in01:   u16,
    pub in02:   u16,
    pub in03:   u16,
    pub in04:   u16,
    /// AnalogIn 6111:1
    pub in05:   u16,
    /// AnalogOut 6120:1
    pub out01:  u16,
    pub uart01: Vec<u8>,
    pub uart02: Vec<u8>,
    pub temp01: u16,
    pub temp02: u16,
    pub temp03: u16,
}


impl Default for AnalogNode {
    fn default() -> Self{
        Self {
            node: 0x2,
            in01: 3262,
            in02: 3000,
            in03: 1500,
            in04: 1000,
            in05: 0,
            out01: 0,
            uart01: Vec::new(),
            uart02: Vec::new(),
            temp01: 774,
            temp02: 774,
            temp03: 774,
         }
    }
}





impl AnalogNode {
    pub fn new(id:u32,can:) -> Self{
        Self {
            node:   id,
            in01:   3262,
            in02:   3000,
            in03:   1500,
            in04:   1000,
            in05:   0,
            out01:  0,
            uart01: Vec::new(),
            uart02: Vec::new(),
            temp01: 774,
            temp02: 774,
            temp03: 774,
         }
    }
    /// Status
    pub fn state(&mut self) -> AnalogNode {
        self.clone()
    }
    /// Read analog IN01

}



// pub trait AnalogOutput: MsgRx + MsgTx{
//     fn read(&mut self) -> nb::Result<u16,CanError>
//     {
//         let tx = Message::read_message(self.rxmsg())?;
//         let rx =  can0::read_message(tx)?;
//         Ok(rx.to_u16())
//     }

//     fn write(&mut self) -> nb::Result<_,CanError> {
//         let tx = Message::read_message(self.rxmsg())?;
//         let _ =  can0::read_message(tx)?;
//         Ok(())
//     }
// }


// pub trait Temperature: MsgRx {
//     fn celsius(&mut self) -> nb::Result<f32,std::io::Error> {
//         let tx = Message::read_message(self.rxmsg())?;
//         let rx =  can0::read_message(tx)?;
//         (rx.to_u16() as f32) / 10.0
//     }
// }
// pub trait Uart: MsgRx + MsgTx {
//     fn read(&mut self) -> nb::Result<Vec<u8>,std::io::Error>  {
//         let tx = Message::read_message(self.rxmsg())?;
//         let rx =  can0::read_message(tx)?;
//         Ok(rx.get_data())
//     }
//     fn write(&mut self) -> nb::Result<_,std::io::Error> {
//         let tx = Message::read_message(self.rxmsg())?;
//         let _ =  can0::read_message(tx)?;
//         Ok(())
//     }
// }



// impl MsgRx for A1IN1 {
//     fn rxmsg(&self) -> Message {
//         Message::new_message(self.node,0x6101,0x1,Vec::new())
//     }
// }
// impl AnalogInput for A1IN1 { }

// impl MsgRx for A1IN2 {
//      fn rxmsg(&self) -> Message {
//         Message::new_message(NODE,0x6101,0x2,Vec::new())
//     }
// }
// impl AnalogInput for A1IN2 { }

// impl MsgRx for A1IN3 {
//     fn rxmsg(&self) -> Message {
//         Message::new_message(NODE,0x6101,0x3,Vec::new())
//     }
// }
// impl AnalogInput for A1IN3 { }

// impl MsgRx for A1IN4 {
//     fn rxmsg(&self) -> Message {
//         Message::new_message(NODE,0x6101,0x4,Vec::new())
//     }
// }
// impl AnalogInput for A1IN4 { }

// impl MsgRx for A1IN5 {
//     fn rxmsg(&self) -> Message {
//         Message::new_message(NODE,0x6111,0x1,Vec::new())
//     }
// }
// impl AnalogInput for A1IN5 { }


// impl MsgRx for A1OUT {
//     fn rxmsg(&self) -> Message {
//         Message::new_message(NODE,0x6120,0x4,Vec::new())
//     }
// }
// impl MsgTx for A1OUT {
//      fn txmsg(&self) -> Message {
//         Message::new_message(NODE,0x6120,0x4,Vec::new())
//     }
// }


