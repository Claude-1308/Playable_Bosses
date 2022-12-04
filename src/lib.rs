#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_variables)]
#![allow(unused_must_use)]

pub static mut FIGHTER_CUTIN_MANAGER_ADDR: usize = 0;
pub static mut FIGHTER_MANAGER: usize = 0;
pub static mut ITEM_MANAGER: usize = 0;

mod common;
mod masterhand;
mod crazyhand;
mod ganonboss;
mod lioleusboss;
mod galleom;
mod marx;
mod koopag;
mod dracula;
mod dracula2;
mod darz;
mod kiila;

#[skyline::main(name = "boss")]
pub fn main() {
    common::install();
    masterhand::install();
    crazyhand::install();
    ganonboss::install();
    lioleusboss::install();
    galleom::install();
    marx::install();
    koopag::install();
    dracula::install();
    dracula2::install();
    darz::install();
    kiila::install();
}
