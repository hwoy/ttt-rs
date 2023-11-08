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

use std::mem::MaybeUninit;
pub struct Builder {
    game: MaybeUninit<ttt_sys::ox_game>,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            game: MaybeUninit::uninit(),
        }
    }
	
	fn as_game(&self)->&ttt_sys::ox_game
	{
		unsafe { self.game.assume_init_ref() }
	}
	
	fn as_game_mut(&mut self)->&mut ttt_sys::ox_game
	{
		unsafe { self.game.assume_init_mut() }
	}

    pub fn set_id(mut self, id: c_uint) -> Self {
        let self_game = self.as_game_mut();
        self_game.id = id;
        self
    }

    pub fn set_seed(mut self, seed: c_uint) -> Self {
        let self_game = self.as_game_mut();
        unsafe {
            ttt_sys::glibcrnginit(self_game.random.as_mut_ptr() as *mut ttt_sys::RND32, seed);
        }
        self
    }

    pub fn set_win(mut self, win: &[::std::os::raw::c_uint; 8usize]) -> Self {
        let self_game = self.as_game_mut();
        self_game.win = *win;
        self
    }
    pub fn set_tri(mut self, tri: &[::std::os::raw::c_uint; 48usize]) -> Self {
        let self_game = self.as_game_mut();
        self_game.tri = *tri;
        self
    }
    pub fn set_nwin(mut self, nwin: c_uint) -> Self {
        let self_game = self.as_game_mut();
        self_game.nwin = nwin;
        self
    }
    pub fn set_ntri(mut self, tri: c_uint) -> Self {
        let self_game = self.as_game_mut();
        self_game.ntri = tri;
        self
    }
    pub fn set_nelement(mut self, nelement: c_uint) -> Self {
        let self_game = self.as_game_mut();
        self_game.nelement = nelement;
        self
    }
    pub fn set_ntrielement(mut self, ntrielement: c_uint) -> Self {
        let self_game = self.as_game_mut();
        self_game.ntrielement = ntrielement;
        self
    }
    pub fn build(self) -> ttt_sys::ox_game {
        unsafe { self.game.assume_init() }
    }
}

pub trait ox_game_impl {
}

impl ox_game_impl for ttt_sys::ox_game {

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
