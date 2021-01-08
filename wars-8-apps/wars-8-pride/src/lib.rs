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
    rectfill(0, y, 255, y + 42, ColorPallete::Red);
    y += 42;
    rectfill(0, y, 255, y + 42, ColorPallete::Gold);
    y += 42;
    rectfill(0, y, 255, y + 42, ColorPallete::Yellow);
    y += 42;
    rectfill(0, y, 255, y + 42, ColorPallete::Green);
    y += 42;
    rectfill(0, y, 255, y + 42, ColorPallete::SkyBlue);
    y += 42;
    rectfill(0, y, 255, 255, ColorPallete::Magenta);
}

pub fn draw_trans() {
    let mut y = 0;
    rectfill(0, y, 255, y + 51, ColorPallete::SkyBlue);
    y += 51;
    rectfill(0, y, 255, y + 51, ColorPallete::Pink);
    y += 51;
    rectfill(0, y, 255, y + 52, ColorPallete::OffWhite);
    y += 52;
    rectfill(0, y, 255, y + 51, ColorPallete::Pink);
    y += 42;
    rectfill(0, y, 255, 255, ColorPallete::SkyBlue);
}

pub fn draw_bi() {
    let mut y = 0;
    rectfill(0, y, 255, y + 95, ColorPallete::Magenta);
    y += 95;
    rectfill(0, y, 255, y + 65, ColorPallete::PalePurple);
    y += 65;
    rectfill(0, y, 255, y + 95, ColorPallete::DarkBlue);
}

#[no_mangle]
pub fn _draw() {
    match *STATE.lock().unwrap() {
        Pride::Lgbt => draw_lgbt(),
        Pride::Trans => draw_trans(),
        Pride::Bi => draw_bi(),
    };
}