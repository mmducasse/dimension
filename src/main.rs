//#![forbid(unsafe_code)]
#![allow(dead_code)]
//#![allow(unused_imports)]
//#![allow(non_camel_case_types)]
//#![allow(unused_variables)]

use crate::{graphics::{buffer::buffer_mut, window::set_scale}, consts::SCREEN_SIZE};

mod graphics;
mod consts;
mod common;
mod systems;
mod io;
mod game;
mod level;
mod entities;

#[macroquad::main("dimension")]
async fn main() {
    println!("*** Interstellar Game Jam 2023 ***");
    println!(" \"Another Dimension\" ");

    set_scale(3);
    
    buffer_mut().init(SCREEN_SIZE);

    game::run().await;
}

