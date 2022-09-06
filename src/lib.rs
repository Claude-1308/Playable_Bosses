#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(non_snake_case)]
#![feature(asm)]

pub static mut FIGHTER_CUTIN_MANAGER_ADDR: usize = 0;
pub static mut FIGHTER_MANAGER: usize = 0;
pub static mut ITEM_MANAGER: usize = 0;

mod common;
mod masterhand;
mod crazyhand;
mod ganonboss;

static mut CONSTANT_OFFSET : usize = 0x3727390; //13.0.1

use skyline::libc::*;
use std::ffi::CStr;

#[skyline::hook(offset = CONSTANT_OFFSET)]
unsafe fn const_allot_hook(unk: *const u8, constant: *const c_char, mut value: u32) {
    if CStr::from_ptr(constant as _).to_str().unwrap().contains("ITEM_INSTANCE_WORK_FLAG_NUM") {
        value = 0x37;
    }
    if CStr::from_ptr(constant as _).to_str().unwrap().contains("ITEM_INSTANCE_WORK_FLAG_TERM") {
        value = 0x20000037;
    }
    if CStr::from_ptr(constant as _).to_str().unwrap().contains("FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_TERM") {
        value = 0x200000eb;
    }
    if CStr::from_ptr(constant as _).to_str().unwrap().contains("FIGHTER_MARIO_INSTANCE_WORK_ID_INT_TERM") {
        value = 0x100000c3;
    }
    original!()(unk,constant,value)
}

#[skyline::main(name = "boss")]
pub fn main() {
    //skyline::install_hook!(const_allot_hook);
    common::install();
    masterhand::install();
    crazyhand::install();
    ganonboss::install();
}