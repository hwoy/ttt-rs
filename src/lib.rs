#![allow(non_camel_case_types)]
#![allow(dead_code)]

extern crate ttt_sys;

use std::os::raw::{c_int, c_uint};

pub trait ox_player_impl {
    fn new(id: c_uint) -> Self;
}

impl ox_player_impl for ttt_sys::ox_player {
    fn new(id: c_uint) -> Self {
        Self { id: id, val: 0 }
    }
}

pub struct Ai {}
impl Ai {
    pub fn ai(
        game: &mut ttt_sys::ox_game,
        p1: &ttt_sys::ox_player,
        p2: &ttt_sys::ox_player,
    ) -> c_int {
        unsafe { ttt_sys::ox_ai(game, p1, p2) }
    }
}

pub trait ox_game_impl {
    fn gameplay(
        &self,
        p1: &ttt_sys::ox_player,
        p2: &mut ttt_sys::ox_player,
        val: c_uint,
    ) -> ttt_sys::ox_gameid;

    fn iswin(&self, player: &ttt_sys::ox_player) -> c_int;
}

impl ox_game_impl for ttt_sys::ox_game {
    fn gameplay(
        &self,
        p1: &ttt_sys::ox_player,
        p2: &mut ttt_sys::ox_player,
        val: c_uint,
    ) -> ttt_sys::ox_gameid {
        unsafe { ttt_sys::ox_gameplay(self, p1, p2, val) }
    }

    fn iswin(&self, player: &ttt_sys::ox_player) -> c_int {
        unsafe { ttt_sys::ox_iswin(self, player) }
    }
}

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
        .set_win(&cons::WINLIST)
        .set_tri(&cons::TRILIST)
        .build()
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
