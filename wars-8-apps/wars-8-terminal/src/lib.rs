#[macro_use]
extern crate lazy_static;
extern crate wars_8_api;

use std::sync::{Mutex, MutexGuard};
use gfx::{ColorPallete, print, printh, rectfill};
use wars_8_api::*;


lazy_static! {
    static ref LINES: Mutex<Vec<(ColorPallete, String)>> = Mutex::new(Vec::new());
    static ref CRS_OFFSET: Mutex<i32> = Mutex::new(0);
}


#[no_mangle]
pub fn _init() {
    gfx::printh("[WARS-8-Terminal] Starting!".to_string());
    LINES.lock().unwrap().push((ColorPallete::Pink, String::new())) ;
}

pub fn process_command(mutex: &mut MutexGuard<Vec<(ColorPallete, String)>>) {
    let (_, cmd_str) = mutex.last().unwrap();
    let cmd_str = cmd_str.to_lowercase();
    let split_cmd = cmd_str.split(' ').collect::<Vec<&str>>();
    match split_cmd[0] {
        "echo" => {
            match cmd_str.find(' ') {
                Some(idx) => {
                    mutex.push((ColorPallete::OffWhite, cmd_str[idx..].to_string()));
                },
                None => { }
            }
        },
        "exit" => {
            misc::exit();
        },
        "help" => {
            mutex.push((ColorPallete::SkyBlue, "help echo exit".to_string()));
        },
        _ => {
            mutex.push((ColorPallete::Red, "INVALID COMMAND".to_string()));
        }
    }
}

#[no_mangle]
pub fn _update() {
    let mut lines_mutex = LINES.lock().unwrap();
    let mut key = input::key();
    let mut crs_offset = CRS_OFFSET.lock().unwrap();
    while key != input::Scancode::None {
        let mut last = (lines_mutex.last().unwrap()).clone();

        if key == input::Scancode::BACKSPACE {
            last.1.pop();
            *(lines_mutex.last_mut().unwrap()) = (last.0, last.1);
            *crs_offset -= 1;
        } else if *crs_offset != 32 {
            *(lines_mutex.last_mut().unwrap()) = (last.0, last.1 + char::from(key).to_string().as_str());
            *crs_offset += 1;
        }

        if key == input::Scancode::RETURN {
            *crs_offset = 0;
            process_command(&mut lines_mutex);
            lines_mutex.push((ColorPallete::Pink, String::new()));
        }
        key = input::key();
    }
    let lines_len = lines_mutex.len();
    if lines_len > 32 {
        for _ in 0..(lines_len - 32) {
            lines_mutex.remove(0);
        }
    }
}

#[no_mangle]
pub fn _draw() {
    rectfill(0, 0, 255, 255, ColorPallete::Black);
    let lines_mutex = LINES.lock().unwrap();
    let lines_len = lines_mutex.len();

    for row in 0..lines_len {
        let y = 256 - (8 * (row + 1));
        let row_content = lines_mutex[(lines_len - 1) - row].clone();
        print(row_content.1, 0, y as i32, row_content.0);
    }
}