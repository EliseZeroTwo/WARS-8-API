#[macro_use]
extern crate lazy_static;
extern crate wars_8_api;

use wars_8_api::gfx::*;
use wars_8_api::input::*;
use std::sync::Mutex;

pub enum Pride {
    Lgbt,
    Trans,
    Bi,
}

lazy_static! {
    static ref STATE: Mutex<Pride> = Mutex::new(Pride::Lgbt);
}

#[no_mangle]
pub fn _init() {
    printh("[WARS-8-Pride] Starting!".to_string());
}

#[no_mangle]
pub fn _update() {
    if btnp(Button::O, Player::One) {
        let mut state_mutex = STATE.lock().unwrap();
        *state_mutex = match *state_mutex {
            Pride::Lgbt => Pride::Bi,
            Pride::Trans => Pride::Lgbt,
            Pride::Bi => Pride::Trans,
        }

    } else if btnp(Button::X, Player::One) {
        let mut state_mutex = STATE.lock().unwrap();
        *state_mutex = match *state_mutex {
            Pride::Lgbt => Pride::Trans,
            Pride::Trans => Pride::Bi,
            Pride::Bi => Pride::Lgbt,
        }
    }
}

pub fn draw_lgbt() {
    let mut y = 0;
    rectfill(0, y, 127, y + 21, ColorPallete::Red);
    y += 21;
    rectfill(0, y, 127, y + 21, ColorPallete::Orange);
    y += 21;
    rectfill(0, y, 127, y + 21, ColorPallete::Yellow);
    y += 21;
    rectfill(0, y, 127, y + 21, ColorPallete::Green);
    y += 21;
    rectfill(0, y, 127, y + 21, ColorPallete::Blue);
    y += 21;
    rectfill(0, y, 127, 127, ColorPallete::DarkPurple);
}

pub fn draw_trans() {
    let mut y = 0;
    rectfill(0, y, 127, y + 25, ColorPallete::Blue);
    y += 25;
    rectfill(0, y, 127, y + 25, ColorPallete::Pink);
    y += 25;
    rectfill(0, y, 127, y + 27, ColorPallete::White);
    y += 27;
    rectfill(0, y, 127, y + 21, ColorPallete::Pink);
    y += 21;
    rectfill(0, y, 127, 127, ColorPallete::Blue);
}

pub fn draw_bi() {
    let mut y = 0;
    rectfill(0, y, 127, y + 47, ColorPallete::DarkPurple);
    y += 47;
    rectfill(0, y, 127, y + 32, ColorPallete::Indigo);
    y += 32;
    rectfill(0, y, 127, 127, ColorPallete::DarkBlue);
}

#[no_mangle]
pub fn _draw() {
    match *STATE.lock().unwrap() {
        Pride::Lgbt => draw_lgbt(),
        Pride::Trans => draw_trans(),
        Pride::Bi => draw_bi(),
    };
}