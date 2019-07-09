#![allow(unused_variables)]

use failure::{Fail};
use std::io;
// use std::string::FromUtf8Error;

// use socketcan::{CANSocketOpenError,ConstructionError};
// use dbus::Error as DBusError;
use jsonrpc_core::Error as RpcError;
// use mut_guard::*;

#[derive(Fail, Debug)]
pub enum MioError {
    #[fail(display = "io error - {}",err)]
    IOError {err: io::Error },

    #[fail(display = "rpc error - {}",err)]
    RpcError {err: RpcError },
//    #[fail(display = "socketcan open error {}", err)]
    // CanOpenError { err: CANSocketOpenError },

//    #[fail(display = "socketcan frame construction error {}", err)]
    // FrameConstructError{err:ConstructionError},

    #[fail(display = "socket can error - {}", msg)]
    Canbus { msg: String },


}


impl From<RpcError> for MioError {
    fn from(kind: RpcError) -> MioError {
        MioError::RpcError{err:kind}
    }
}
// impl From<CANSocketOpenError> for MioError {
    // fn from(kind: CANSocketOpenError) -> MioError {
        // MioError::CanOpenError{err:kind}
    // }
// }
impl From<io::Error> for MioError {
    fn from(kind:io::Error) -> MioError {
        MioError::IOError{err: kind}
    }
}

// impl From<DBusError> for MioError {
    // fn from(kind:DBusError) -> MioError {
        // MioError::DBusError{err:kind}
    // }
// }


//
