//#![forbid(unsafe_code)]
#![allow(dead_code)]
//#![allow(unused_imports)]
//#![allow(non_camel_case_types)]
//#![allow(unused_variables)]

use macroquad::window::request_new_screen_size;
use xf::num::fvec2::FVec2;

use crate::{graphics::buffer::buffer_mut, consts::SCREEN_SIZE};

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

    let scale = 3.0;
    let desired_size = SCREEN_SIZE.as_fvec2() * FVec2::splat(scale);
    request_new_screen_size(desired_size.x, desired_size.y);

    buffer_mut().init(SCREEN_SIZE);

    game::run().await;
}

