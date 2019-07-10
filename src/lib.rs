// #![feature(async_await)]


mod error;
mod subscription;
mod simulation;
// pub mod can;
mod analog;
mod motor;
mod digital;
mod aouts;
pub mod bindings;

// pub mod api;
// pub mod bindings;
pub use self::error::MioError;
pub use self::bindings as can;
pub use self::subscription::*;
pub use self::analog::*;
pub use self::digital::*;
pub use self::motor::*;
pub use self::aouts::*;

// pub trait CanNode {
    // const ID: i32;
    //
    // fn id() -> i32 {
    // Self::ID
    // }
    // }
    //
    //
    //
    //end
