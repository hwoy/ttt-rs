#![allow(non_camel_case_types)]

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

pub struct Builder {
    game: ttt_sys::ox_game,
}

impl Builder {
    pub fn new(id: c_uint, seed: c_uint) -> Self {
        let mut s = unsafe { ttt_sys::ox_creatgame(seed) };
        s.id = id;
        Self { game: s }
    }

    pub fn set_win(mut self, win: &[::std::os::raw::c_uint; 8usize]) -> Self {
        self.game.win = *win;
        self
    }
    pub fn set_tri(mut self, tri: &[::std::os::raw::c_uint; 48usize]) -> Self {
        self.game.tri = *tri;
        self
    }
    pub fn set_nwin(mut self, nwin: c_uint) -> Self {
        self.game.nwin = nwin;
        self
    }
    pub fn set_ntri(mut self, tri: c_uint) -> Self {
        self.game.ntri = tri;
        self
    }
    pub fn set_nelement(mut self, nelement: c_uint) -> Self {
        self.game.nelement = nelement;
        self
    }
    pub fn set_ntrielement(mut self, ntrielement: c_uint) -> Self {
        self.game.ntrielement = ntrielement;
        self
    }
    pub fn build(self) -> ttt_sys::ox_game {
        self.game
    }
}

pub trait ox_game_impl {
    fn new(id: c_uint, seed: c_uint) -> Self;
}

impl ox_game_impl for ttt_sys::ox_game {
    fn new(id: c_uint, seed: c_uint) -> Self {
        let mut s = unsafe { ttt_sys::ox_creatgame(seed) };
        s.id = id;
        s
    }
}

/*****************************************************************/
pub fn seed_from_entropy() -> ttt_sys::URND32 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as c_uint
}

pub mod prelude {
    pub extern crate ttt_sys;
    pub use crate::ox_game_impl;
    pub use crate::ox_player_impl;
}
