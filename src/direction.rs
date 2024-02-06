extern crate cfg_if;
extern crate wasm_bindgen;
extern crate web_sys;
use wasm_bindgen::prelude::*;
use rand::{thread_rng, Rng};

use crate::Vector2D;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct Direction {
    pub cardinal: Cardinal,
    pub distance: u32,
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub enum Cardinal {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl Cardinal {
    pub fn iter() -> impl Iterator<Item = Cardinal> {
        [
            Cardinal::N,
            Cardinal::NE,
            Cardinal::E,
            Cardinal::SE,
            Cardinal::S,
            Cardinal::SW,
            Cardinal::W,
            Cardinal::NW,
        ]
        .iter()
        .copied()
    }
}

#[wasm_bindgen]
impl Direction {
    pub fn new(cardinal: Cardinal, distance: u32) -> Self {
        Self { cardinal, distance }
    }

    pub fn new_rng(distance: u32) -> Self {
        let mut rng = thread_rng();
        let cardinal = match rng.gen_range(0..8) {
            1 => Cardinal::NW,
            2 => Cardinal::W,
            3 => Cardinal::SW,
            4 => Cardinal::S,
            5 => Cardinal::SE,
            6 => Cardinal::E,
            7 => Cardinal::NE,
            _ => Cardinal::N,
        };
        Self { cardinal, distance }
    }

    pub fn factor(&self) -> Vector2D {
        match self.cardinal {
            Cardinal::N => Vector2D { x: 0, y: -1 },
            Cardinal::NE => Vector2D { x: 1, y: -1 },
            Cardinal::E => Vector2D { x: 1, y: 0 },
            Cardinal::SE => Vector2D { x: 1, y: 1 },
            Cardinal::S => Vector2D { x: 0, y: 1 },
            Cardinal::SW => Vector2D { x: -1, y: 1 },
            Cardinal::W => Vector2D { x: -1, y: 0 },
            Cardinal::NW => Vector2D { x: -1, y: -1 },
        }
    }

    pub fn distance(&self) -> u32 {
        self.distance
    }

    pub fn set_distance(&mut self, distance: u32) {
        self.distance = distance;
    }
}
