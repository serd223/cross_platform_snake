use std::{mem, ptr::slice_from_raw_parts_mut};

use game::{Game, GameColor};

static mut GAME: Game = Game::const_default();

struct ABgrColor {}

impl GameColor for ABgrColor {
    fn from_rgbau32(rgba: u32) -> u32 {
        let r = rgba & 0xff000000;
        let g = rgba & 0x00ff0000;
        let b = rgba & 0x0000ff00;
        let a = rgba & 0x000000ff;
        (a << 24) | (b << 8) | (g >> 8) | (r >> 24)
    }
}

#[no_mangle]
pub extern "C" fn allocate_image(width: usize, height: usize) -> *mut u32 {
    let mut v: Vec<u32> = Vec::with_capacity(width * height);
    let ret = v.as_mut_ptr();
    mem::forget(v);
    ret
}

#[no_mangle]
pub extern "C" fn frame(image_ptr: *mut u32, width: usize, height: usize, delta: f32) {
    let image_data = slice_from_raw_parts_mut(image_ptr, width * height);
    unsafe {
        GAME.frame::<ABgrColor>(&mut (*image_data), width, height, delta);
    }
}
