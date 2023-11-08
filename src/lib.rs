#![allow(non_camel_case_types)]
#![allow(dead_code)]

extern crate ttt_sys;

use std::os::raw::c_uint;

pub trait ox_player_impl {
    fn new(id: c_uint) -> Self;
}

impl ox_player_impl for ttt_sys::ox_player {
    fn new(id: c_uint) -> Self {
        Self { id: id, val: 0 }
    }
}

pub trait ox_game_impl {}

impl ox_game_impl for ttt_sys::ox_game {}

/*****************************************************************/
pub fn seed_from_entropy() -> c_uint {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as c_uint
}

pub mod prelude {
    pub extern crate ttt_sys;
    pub mod builder;
    pub use crate::ox_game_impl;
    pub use crate::ox_player_impl;
}
