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

pub fn build_game_with(id: u32, seed: u32) -> ttt_sys::ox_game {
    use prelude::{builder::Builder, cons};

    Builder::new()
        .set_id(id)
        .set_random(seed)
        .set_nwin(ttt_sys::NWIN)
        .set_ntri(ttt_sys::NTRI)
        .set_nelement(ttt_sys::NELEMENT)
        .set_ntrielement(ttt_sys::NTRIELEMENT)
        .build(&cons::WINLIST, &cons::TRILIST)
}

pub fn build_game() -> ttt_sys::ox_game {
    build_game_with(0, seed_from_entropy())
}

pub fn build_players() -> Vec<ttt_sys::ox_player> {
    vec![ttt_sys::ox_player::new(0), ttt_sys::ox_player::new(1)]
}

pub mod prelude {
    pub extern crate ttt_sys;
    pub mod builder;
    pub mod cons;
    pub use crate::ox_game_impl;
    pub use crate::ox_player_impl;
}
