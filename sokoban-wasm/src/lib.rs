extern crate csv;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::ffi::{CStr, CString};
use std::mem;
use std::os::raw::{c_char, c_void};
use std::str;
use String;

use wasm_bindgen::prelude::*;
mod sokoban;

#[wasm_bindgen]
pub fn win_state(board: &str) -> bool
{
    sokoban::win_state(board)
}

#[wasm_bindgen]
pub fn move_player(board: &str, direction: &str) -> String {
    sokoban::move_player(board, direction)
}

#[wasm_bindgen]
pub fn get_level(level: i8, board: i8) -> String {
    sokoban::read_level(level, board).unwrap_or("".to_string())
}

#[no_mangle]
pub extern fn c_move(board: *mut c_char, direction: *mut c_char) -> *mut c_char {
    let org_board: Vec<u8> = unsafe { CStr::from_ptr(board).to_bytes().to_vec() };
    let move_dir: Vec<u8> = unsafe { CStr::from_ptr(direction).to_bytes().to_vec() };

    let mut new_board: String = move_player(
        str::from_utf8(&org_board).unwrap(),
        str::from_utf8(&move_dir).unwrap());

    unsafe {
        let output: Vec<u8> = new_board.as_mut_vec().iter().map(|c| *c as u8).collect::<Vec<_>>();
        CString::from_vec_unchecked(output)
    }.into_raw()
}

#[no_mangle]
pub extern fn c_win_state(board: *mut c_char) -> bool {
    let org_board: Vec<u8> = unsafe { CStr::from_ptr(board).to_bytes().to_vec() };
    return win_state(str::from_utf8(&org_board).unwrap());
}

#[no_mangle]
pub extern fn c_get_level(level: i8, board: i8) -> *mut c_char {
    unsafe {
        let output: Vec<u8> = sokoban::read_level(level, board).unwrap().as_mut_vec().iter().map(|c| *c as u8).collect::<Vec<_>>();
        CString::from_vec_unchecked(output)
    }.into_raw()
}

#[no_mangle]
pub extern fn allocate(size: usize) -> *mut c_void {
    let mut buffer = Vec::with_capacity(size);
    let pointer = buffer.as_mut_ptr();
    mem::forget(buffer);

    pointer as *mut c_void
}

#[no_mangle]
pub extern fn deallocate(pointer: *mut c_void, capacity: usize) {
    unsafe {
        let _ = Vec::from_raw_parts(pointer, 0, capacity);
    }
}
