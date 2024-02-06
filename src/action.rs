extern crate cfg_if;
extern crate wasm_bindgen;
extern crate web_sys;
use wasm_bindgen::prelude::*;
use crate::{Direction, Element, Vector2D};
use crate::direction::Cardinal;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub enum ActionType {
    Move,
    Burn,
    Eat,
    Die,
    Grow,
    Disolve,
    Liquidize,
}

#[wasm_bindgen]
pub struct Action {
    pub t: ActionType,
    pub v: Vector2D,
    pub d: Direction,
    pub e: Element,
}

impl Action {
    pub fn new_move(v: Vector2D, d: Direction) -> Self {
        Self {
            t: ActionType::Move,
            v,
            d,
            e: Element::Air,
        }
    }
    pub fn new_burn(v: Vector2D) -> Self {
        Self {
            t: ActionType::Burn,
            v,
            d: Direction::new(Cardinal::N, 0),
            e: Element::Fire,
        }
    }
    pub fn new_eat(v: Vector2D, e: Element) -> Self {
        Self {
            t: ActionType::Eat,
            v,
            d: Direction::new(Cardinal::N, 0),
            e,
        }
    }
    pub fn new_die(v: Vector2D, e: Element) -> Self {
        Self {
            t: ActionType::Die,
            v,
            d: Direction::new(Cardinal::N, 0),
            e,
        }
    }
    pub fn new_grow(v: Vector2D) -> Self {
        Self {
            t: ActionType::Grow,
            v,
            d: Direction::new(Cardinal::N, 0),
            e: Element::Air,
        }
    }
    pub fn new_disolve(v: Vector2D) -> Self {
        Self {
            t: ActionType::Disolve,
            v,
            d: Direction::new(Cardinal::N, 0),
            e: Element::Air,
        }
    }
    pub fn new_liquidize(v: Vector2D) -> Self {
        Self {
            t: ActionType::Liquidize,
            v,
            d: Direction::new(Cardinal::N, 0),
            e: Element::Air,
        }
    }
}


// pub enum Action {
//     Move(Vector2D, Direction),
//     Burn(Vector2D),
//     Eat(Vector2D, Element),
//     Die(Vector2D, Element),
//     Grow(Vector2D),
//     Disolve(Vector2D),
//     Liquidize(Vector2D),
// }
