mod model;
mod render;

use crate::{model::PetviewModel, render::PetviewRender};
use rust_pixel::game::Game;
// use log::info;

#[cfg(target_arch = "wasm32")]
use rust_pixel::render::adapter::web::{input_events_from_web, WebAdapter};
#[cfg(target_arch = "wasm32")]
use rust_pixel::render::adapter::RenderCell;
use wasm_bindgen::prelude::*;

use pixel_macro::pixel_game;

pixel_game!(Petview);
