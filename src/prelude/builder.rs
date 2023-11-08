#![allow(non_camel_case_types)]
#![allow(dead_code)]

extern crate ttt_sys;

use std::os::raw::c_uint;

use std::mem::MaybeUninit;

pub struct Builder_Prototype {
    game: MaybeUninit<ttt_sys::ox_game>,
}

pub struct Builder {
    game: MaybeUninit<ttt_sys::ox_game>,
}

impl Builder {
    pub fn new_prototype() -> Builder_Prototype {
        Builder_Prototype {
            game: MaybeUninit::uninit(),
        }
    }

    pub fn build(self) -> ttt_sys::ox_game {
        unsafe { self.game.assume_init() }
    }
}

struct Builder_id {
    game: MaybeUninit<ttt_sys::ox_game>,
}
impl Builder_id {
    pub fn set_id(mut builder: Builder_Prototype, id: c_uint) -> Self {
        let self_game = unsafe { builder.game.assume_init_mut() };

        self_game.id = id;

        Self { game: builder.game }
    }
}

struct Builder_random {
    game: MaybeUninit<ttt_sys::ox_game>,
}
impl Builder_random {
    pub fn set_random(mut builder: Builder_id, seed: c_uint) -> Self {
        let self_game = unsafe { builder.game.assume_init_mut() };

        unsafe {
            ttt_sys::glibcrnginit(self_game.random.as_mut_ptr() as *mut ttt_sys::RND32, seed);
        }

        Self { game: builder.game }
    }
}

struct Builder_win {
    game: MaybeUninit<ttt_sys::ox_game>,
}
impl Builder_win {
    pub fn set_win(mut builder: Builder_random, win: &[::std::os::raw::c_uint; 8usize]) -> Self {
        let self_game = unsafe { builder.game.assume_init_mut() };

        self_game.win = *win;

        Self { game: builder.game }
    }
}

struct Builder_tri {
    game: MaybeUninit<ttt_sys::ox_game>,
}
impl Builder_tri {
    pub fn set_tri(mut builder: Builder_win, tri: &[::std::os::raw::c_uint; 48usize]) -> Self {
        let self_game = unsafe { builder.game.assume_init_mut() };

        self_game.tri = *tri;

        Self { game: builder.game }
    }
}

struct Builder_nwin {
    game: MaybeUninit<ttt_sys::ox_game>,
}
impl Builder_nwin {
    pub fn set_nwin(mut builder: Builder_tri, nwin: c_uint) -> Self {
        let self_game = unsafe { builder.game.assume_init_mut() };

        self_game.nwin = nwin;

        Self { game: builder.game }
    }
}

struct Builder_ntri {
    game: MaybeUninit<ttt_sys::ox_game>,
}
impl Builder_ntri {
    pub fn set_ntri(mut builder: Builder_nwin, ntri: c_uint) -> Self {
        let self_game = unsafe { builder.game.assume_init_mut() };

        self_game.ntri = ntri;

        Self { game: builder.game }
    }
}

struct Builder_nelement {
    game: MaybeUninit<ttt_sys::ox_game>,
}
impl Builder_nelement {
    pub fn set_nelement(mut builder: Builder_ntri, nelement: c_uint) -> Self {
        let self_game = unsafe { builder.game.assume_init_mut() };

        self_game.nelement = nelement;

        Self { game: builder.game }
    }
}

struct Builder_ntrielement {
    game: MaybeUninit<ttt_sys::ox_game>,
}
impl Builder_ntrielement {
    pub fn set_ntrielement(mut builder: Builder_nelement, ntrielement: c_uint) -> Builder {
        let self_game = unsafe { builder.game.assume_init_mut() };

        self_game.ntrielement = ntrielement;

        Builder { game: builder.game }
    }
}
