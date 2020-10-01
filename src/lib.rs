#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![allow(non_snake_case)]

use skyline::nro::{self, NroInfo};

mod once_per_frame;
mod utils;
mod effect;
mod vars;

fn nro_main(nro: &NroInfo) {
    match nro.name {
        "common" => {
            effect::install();
        }
        _ => (),
    }
}

#[skyline::main(name = "TurboOverCurry")]
pub fn main() {
    nro::add_hook(nro_main).unwrap();
    once_per_frame::install();
}