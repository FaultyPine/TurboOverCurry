#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![allow(non_snake_case)]

mod once_per_frame;
mod utils;
mod vars;

#[skyline::main(name = "TurboOverCurry")]
pub fn main() {
    once_per_frame::install();
}