extern crate cfg_if;
extern crate wasm_bindgen;
extern crate web_sys;
// use std::fmt;
use wasm_bindgen::prelude::*;
// use web_sys::console;

use std::fmt::{Display, Formatter};

use rand::seq::SliceRandom;
use rand::Rng;

use crate::action::ActionType;
use crate::direction::Cardinal;
use crate::element::Physics;
use crate::Cell;
use crate::Element;
use crate::Vector2D;
use crate::{Action, Direction};


#[wasm_bindgen]
pub struct Simulation {
    dimensions: Vector2D,
    world: Vec<Vec<Cell>>,
    data: Vec<u8>,
}

impl Display for Simulation {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        for row in self.world.iter() {
            for cell in row.iter() {
                write!(f, "{}", cell)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[wasm_bindgen]
impl Simulation {
    pub fn width(&self) -> i32 {
        self.dimensions.x
    }

    pub fn height(&self) -> i32 {
        self.dimensions.y
    }

    pub fn new(x: i32, y: i32) -> Self {
        Self {
            dimensions: Vector2D { x, y },
            world: vec![vec![Cell::new(Element::Air); x as usize]; y as usize],
            data: vec![0; ((x * y) * 3) as usize],
        }
    }
    pub fn update(&mut self) {
        let buffer = self.world.clone();
        for (y, row) in buffer.iter().enumerate() {
            let mut shuffle = (0..row.len()).collect::<Vec<usize>>();
            shuffle.shuffle(&mut rand::thread_rng());

            for x in shuffle {
                let action = self.world[y][x].update(Vector2D { x: x as i32, y: y as i32 }, &self);
                for action in action {
                    self.apply_actions(action);
                }
                self.world[y][x].decay();
            }
        }
        for y in 0..self.dimensions.y {
            for x in 0..self.dimensions.x {
                self.data[((y * self.dimensions.x + x) * 3) as usize] = self.world[y as usize][x as usize].element().to_byte();
                self.data[((y * self.dimensions.x + x) * 3 + 1) as usize] = self.world[y as usize][x as usize].variant();
                self.data[((y * self.dimensions.x + x) * 3 + 2) as usize] = self.world[y as usize][x as usize].updated() as u8;
            }
        }
    }

    pub fn reset_update(&mut self) {
        for row in self.world.iter_mut() {
            for cell in row.iter_mut() {
                cell.reset_update();
            }
        }
    }
    pub fn dump(&mut self) -> *const u8 {
        self.data.as_ptr()
    }

    pub fn in_bounds(&self, position: &Vector2D) -> bool {
        position.x < self.dimensions.x && position.y < self.dimensions.y && position.x >= 0 && position.y >= 0
    }

    fn get_cell(&self, position: Vector2D) -> &Cell {
        &self.world[position.y as usize][position.x as usize]
    }

    pub fn check_cell(&self, position: Vector2D) -> Element {
        self.world[position.y as usize][position.x as usize].element()
    }

    pub fn at(&self, from: &Vector2D, direction: Direction) -> Vector2D {
        let factor = direction.factor();
        let destination = Vector2D {
            x: (from.x + direction.distance() as i32 * factor.x),
            y: (from.y + direction.distance() as i32 * factor.y),
        };
        destination
    }

    pub fn apply_actions(&mut self, mut action: Action) {
        match action.t {
            ActionType::Move => {
                for _ in 1..=action.d.distance() {
                    let factor = action.d.factor();
                    let destination = Vector2D {
                        x: (action.v.x + factor.x),
                        y: (action.v.y + factor.y),
                    };
                    self.swap(&action.v, &destination);
                    action.v = destination;
                }
            }
            ActionType::Eat => {
                self.world[action.v.y as usize][action.v.x as usize].set_element(action.e);
            }
            ActionType::Burn => {
                let source = self.world[action.v.y as usize][action.v.x as usize].element();
                let mut rng = rand::thread_rng();
                for neighbour in self.get_neighbours(&action.v) {
                    let ignite = rng.gen_bool(self.get_cell(neighbour).element().flammability() * source.heat());
                    if ignite {
                        if self.get_cell(neighbour).element() == Element::Wood {
                            self.world[neighbour.y as usize][neighbour.x as usize].set_element(Element::Ember);
                        } else if self.get_cell(neighbour).element() == Element::Coal {
                            self.world[neighbour.y as usize][neighbour.x as usize].set_element(Element::Ember);
                        } else {
                            self.world[neighbour.y as usize][neighbour.x as usize] = Cell::new(Element::Fire);
                        }
                        self.world[neighbour.y as usize][neighbour.x as usize].set_update();
                    }
                    if source == Element::Ember
                        && self.get_cell(neighbour).element() == Element::Air
                        && rng.gen_bool(0.005)
                    {
                        if rng.gen_bool(0.5) {
                            self.world[neighbour.y as usize][neighbour.x as usize] = Cell::new(Element::Smoke);
                        } else {
                            self.world[neighbour.y as usize][neighbour.x as usize] = Cell::new(Element::Fire);
                        }
                        self.world[neighbour.y as usize][neighbour.x as usize].set_update();
                    }
                    if source == Element::Ember && self.get_cell(neighbour).element() == Element::Water {
                        self.world[action.v.y as usize][action.v.x as usize] = Cell::new(Element::Coal);
                        self.world[action.v.y as usize][action.v.x as usize].set_update();
                    }
                    if source == Element::Lava {
                        if self.get_cell(neighbour).element() == Element::Water {
                            self.world[action.v.y as usize][action.v.x as usize] = Cell::new(Element::Stone);
                            self.world[action.v.y as usize][action.v.x as usize].set_update();
                            self.world[neighbour.y as usize][neighbour.x as usize] = Cell::new(Element::Stone);
                            self.world[neighbour.y as usize][neighbour.x as usize].set_update();
                        }
                        if self.get_cell(neighbour).element() == Element::Ice {
                            self.world[neighbour.y as usize][neighbour.x as usize] = Cell::new(Element::Water);
                            self.world[neighbour.y as usize][neighbour.x as usize].set_update();
                        }
                        if self.get_cell(neighbour).element() == Element::Air && rng.gen_bool(0.01) {
                            if rng.gen_bool(0.9) {
                                self.world[neighbour.y as usize][neighbour.x as usize] = Cell::new(Element::Smoke);
                            } else {
                                self.world[neighbour.y as usize][neighbour.x as usize] = Cell::new(Element::Fire);
                            }
                            self.world[neighbour.y as usize][neighbour.x as usize].set_update();
                        }
                    }
                    if source == Element::Acid && self.get_cell(neighbour).element() == Element::Air {
                        if rng.gen_bool(0.01) {
                            self.world[neighbour.y as usize][neighbour.x as usize] = Cell::new(Element::Gas);
                        }
                        if rng.gen_bool(0.01) {
                            self.world[neighbour.y as usize][neighbour.x as usize] = Cell::new(Element::Smoke);
                        }
                    }
                    if source == Element::Air && self.get_cell(neighbour).element() == Element::Fire {
                        if rng.gen_bool(0.5) {
                            self.world[action.v.y as usize][action.v.x as usize] = Cell::new(Element::Fire);
                            return;
                        }
                    }
                }
            }
            ActionType::Grow => {
                let mut rng = rand::thread_rng();
                for neighbour in self.get_neighbours(&action.v) {
                    if rng.gen_bool(0.005) {
                        if self.get_cell(neighbour).element() == Element::Water {
                            self.world[neighbour.y as usize][neighbour.x as usize] = Cell::new(Element::Moss);
                            self.world[neighbour.y as usize][neighbour.x as usize].set_update();

                        }
                    }
                }
            }
            ActionType::Disolve => {
                let mut rng = rand::thread_rng();
                for neighbour in self.get_neighbours(&action.v) {
                    if rng.gen_bool(0.005) {
                        if self.get_cell(neighbour).element() == Element::Water {
                            self.world[action.v.y as usize][action.v.x as usize] = Cell::new(Element::Water);
                            self.world[action.v.y as usize][action.v.x as usize].set_update();
                        }
                    }
                }
            }
            ActionType::Liquidize => {
                let mut rng = rand::thread_rng();
                for neighbour in self.get_neighbours(&action.v) {
                    if rng.gen_bool(0.1) {
                        if self.get_cell(neighbour).element() == Element::Fire || self.get_cell(neighbour).element() == Element::Smoke {
                            self.world[action.v.y as usize][action.v.x as usize] = Cell::new(Element::Water);
                            self.world[action.v.y as usize][action.v.x as usize].set_update();
                        }
                    }
                }
            }
            _ => {}
        }
    }

    pub fn swap(&mut self, from: &Vector2D, to: &Vector2D) {
        let temp = self.world[from.y as usize][from.x as usize];
        self.world[from.y as usize][from.x as usize] = self.world[to.y as usize][to.x as usize];
        self.world[to.y as usize][to.x as usize] = temp;

        self.world[from.y as usize][from.x as usize].set_update();
        self.world[to.y as usize][to.x as usize].set_update();
    }

    pub fn get_neighbours(&self, position: &Vector2D) -> Vec<Vector2D> {
        let mut neighbours: Vec<Vector2D> = vec![];
        for orientation in Cardinal::iter() {
            let destintion = self.at(position, Direction::new(orientation, 1));
            if !self.in_bounds(&destintion) {
                continue;
            }
            neighbours.push(destintion);
        }
        neighbours
    }

    pub fn set_cell(&mut self, x: i32, y: i32, element: i32) {
        let element = Element::from_byte(element as u8).unwrap();
        self.world[y as usize][x as usize] = Cell::new(element);
        self.world[y as usize][x as usize].set_update();
    }
}
