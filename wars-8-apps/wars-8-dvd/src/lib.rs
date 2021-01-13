#[macro_use]
extern crate lazy_static;
extern crate wars_8_api;

use wars_8_api::*;
use std::sync::Mutex;

const MSG: &str = "UwU";

lazy_static! {
    static ref COORDS: Mutex<((i32, i32), (i32, i32), bool)> = Mutex::new(((0, 6), (1, 1), false));
}

#[no_mangle]
pub fn _init() {

}

#[no_mangle]
pub fn _update() {
    let mut coords = COORDS.lock().unwrap();
    let ((mut x, mut y),(mut dx, mut dy), mut status) = *coords;

        for _ in 0..4 {
            x += dx;
            y += dy;
            
            if x < 1 || x >= 127 - (MSG.len() as i32 * 4) {
                dx *= -1
            }
            
            if y < 6 || y >= 127 {
                dy *= -1
            }
        }

    status = !status;

    *coords = ((x, y), (dx, dy), status);
}

#[no_mangle]
pub fn _draw() {
    let ((x, y),(_, _), _) = *COORDS.lock().unwrap();
    if input::btn(input::Button::O, input::Player::One) {
        gfx::cls(gfx::ColorPallete::Pink);
        gfx::print(MSG.to_string(), x, y, gfx::ColorPallete::White)
    } else {
        gfx::cls(gfx::ColorPallete::White);
        gfx::print(MSG.to_string(), x, y, gfx::ColorPallete::Pink)
    }

}
