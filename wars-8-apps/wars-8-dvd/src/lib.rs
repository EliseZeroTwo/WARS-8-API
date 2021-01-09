#[macro_use]
extern crate lazy_static;
extern crate wars_8_api;

use wars_8_api::*;
use std::sync::Mutex;

const MSG: &str = "UwU";

lazy_static! {
    static ref COORDS: Mutex<((i32, i32), (i32, i32))> = Mutex::new(((80, 128), (1, 1)));
}

#[no_mangle]
pub fn _init() {

}

#[no_mangle]
pub fn _update() {
    let mut coords = COORDS.lock().unwrap();
    let ((mut x, mut y),(mut dx, mut dy)) = *coords;

    for _ in 0..2 {
        x += dx;
        y += dy;
        
        if x < 1 || x > 255 - (MSG.len() as i32 * 16) {
            dx *= -1
        }  else if y < 1 || y > 255 - 16 {
            dy *= -1
        }
    }

    *coords = ((x, y), (dx, dy));
}

#[no_mangle]
pub fn _draw() {
    let ((x, y),(_, _)) = *COORDS.lock().unwrap();
    if input::btn(input::Button::O, input::Player::One) {
        gfx::cls(gfx::ColorPallete::Pink);
        gfx::print(MSG.to_string(), x, y, gfx::ColorPallete::White)
    } else {
        gfx::cls(gfx::ColorPallete::White);
        gfx::print(MSG.to_string(), x, y, gfx::ColorPallete::Pink)
    }

}
