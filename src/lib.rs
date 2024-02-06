extern crate cfg_if;
extern crate wasm_bindgen;
extern crate web_sys;
use wasm_bindgen::prelude::*;
extern crate core;

pub use action::Action;
pub use cell::Cell;
#[allow(dead_code, unused)]
pub use config::open_config;
pub use direction::Direction;
pub use element::Element;
pub use simulation::Simulation;

mod action;
mod cell;
mod config;
mod direction;
mod element;
mod simulation;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct Vector2D {
    pub x: i32,
    pub y: i32,
}
