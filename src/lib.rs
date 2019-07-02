#![feature(async_await)]

//! This crate is an placeholder for a future development.
mod error;
mod dbus;
mod rpc;
mod mio;
mod pcan;
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



// #[cfg(test)]
// mod tests {

    // #[test]
    // fn airflow_value() {
    // }
// }
//

//end
