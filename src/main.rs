//#![forbid(unsafe_code)]
#![allow(dead_code)]
//#![allow(unused_imports)]
//#![allow(non_camel_case_types)]
//#![allow(unused_variables)]

use crate::{graphics::buffer::buffer_mut, consts::SCREEN_SIZE};

mod graphics;
mod consts;
mod common;
mod game;
mod level;
mod object; // todo: remove Object

#[macroquad::main("dimension")]
async fn main() {
    println!("*** Interstellar Game Jam 2023 ***");
    println!(" \"Another Dimension\" ");

    buffer_mut().init(SCREEN_SIZE);

    game::run().await;
}

