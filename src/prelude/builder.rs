#![allow(non_camel_case_types)]
#![allow(dead_code)]

extern crate ttt_sys;

use std::os::raw::{c_uint, c_void};

use std::mem::MaybeUninit;

pub struct Builder {
    game: MaybeUninit<ttt_sys::ox_game>,
}

impl Builder {
    pub fn new() -> Builder_id {
        Builder_id {
            game: MaybeUninit::uninit(),
        }
    }

    pub fn build(
        mut self,
        winlist: &[[c_uint; 3usize]; 8usize],
        trilist: &[[c_uint; 3usize]; 48usize],
    ) -> ttt_sys::ox_game {
        let self_game = unsafe { self.game.assume_init_mut() };

        unsafe {
            ttt_sys::ox_genpow2a(
                self_game.win.as_mut_ptr(),
                winlist.as_ptr() as *const c_void,
                self_game.nwin,
                self_game.nelement,
            );

            ttt_sys::ox_genpow2a(
                self_game.tri.as_mut_ptr(),
                trilist.as_ptr() as *const c_void,
                self_game.ntri,
                self_game.ntrielement,
            );
        }

        unsafe { self.game.assume_init() }
    }
}

pub struct Builder_id {
    game: MaybeUninit<ttt_sys::ox_game>,
}
impl Builder_id {
    pub fn set_id(mut self, id: c_uint) -> Builder_random {
        let self_game = unsafe { self.game.assume_init_mut() };

        self_game.id = id;

        Builder_random { game: self.game }
    }
}

pub struct Builder_random {
    game: MaybeUninit<ttt_sys::ox_game>,
}
impl Builder_random {
    pub fn set_random(mut self, seed: c_uint) -> Builder_nwin {
        let self_game = unsafe { self.game.assume_init_mut() };

        unsafe {
            ttt_sys::glibcrnginit(self_game.random.as_mut_ptr() as *mut ttt_sys::RND32, seed);
        }

        Builder_nwin { game: self.game }
    }
}

pub struct Builder_nwin {
    game: MaybeUninit<ttt_sys::ox_game>,
}
impl Builder_nwin {
    pub fn set_nwin(mut self, nwin: c_uint) -> Builder_ntri {
        let self_game = unsafe { self.game.assume_init_mut() };

        self_game.nwin = nwin;

        Builder_ntri { game: self.game }
    }
}

pub struct Builder_ntri {
    game: MaybeUninit<ttt_sys::ox_game>,
}
impl Builder_ntri {
    pub fn set_ntri(mut self, ntri: c_uint) -> Builder_nelement {
        let self_game = unsafe { self.game.assume_init_mut() };

        self_game.ntri = ntri;

        Builder_nelement { game: self.game }
    }
}

pub struct Builder_nelement {
    game: MaybeUninit<ttt_sys::ox_game>,
}
impl Builder_nelement {
    pub fn set_nelement(mut self, nelement: c_uint) -> Builder_ntrielement {
        let self_game = unsafe { self.game.assume_init_mut() };

        self_game.nelement = nelement;

        Builder_ntrielement { game: self.game }
    }
}

pub struct Builder_ntrielement {
    game: MaybeUninit<ttt_sys::ox_game>,
}
impl Builder_ntrielement {
    pub fn set_ntrielement(mut self, ntrielement: c_uint) -> Builder {
        let self_game = unsafe { self.game.assume_init_mut() };

        self_game.ntrielement = ntrielement;

        Builder { game: self.game }
    }
}
