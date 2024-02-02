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

#[derive(Clone, Copy, Debug)]
pub struct Vector2D<T> {
    pub x: T,
    pub y: T,
}
