#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]

mod acmd;
mod once_per_frame;
mod utils;

#[skyline::main(name = "TurboOverCurry")]
pub fn main() {
    acmd::install();
    once_per_frame::install();
}