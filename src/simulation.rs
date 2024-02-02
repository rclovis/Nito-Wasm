extern crate cfg_if;
extern crate wasm_bindgen;
extern crate web_sys;
use std::fmt::{Display, Formatter};

use rand::seq::SliceRandom;
use rand::Rng;

use crate::direction::Cardinal;
use crate::element::Physics;
use crate::Cell;
use crate::Element;
use crate::Vector2D;
use crate::{Action, Direction};

pub struct Simulation {
    dimensions: Vector2D<usize>,
    pub world: Vec<Vec<Cell>>,
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

impl Simulation {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            dimensions: Vector2D { x, y },
            world: vec![vec![Cell::new(Element::Air); x]; y],
        }
    }
    pub fn update(&mut self) {
        let buffer = self.world.clone();
        for (y, row) in buffer.iter().enumerate() {
            let mut shuffle = (0..row.len()).collect::<Vec<usize>>();
            shuffle.shuffle(&mut rand::thread_rng());

            for x in shuffle {
                let action = row[x].update(Vector2D { x, y }, &self);
                for action in action {
                    self.apply_actions(action);
                }
                self.world[y][x].decay();
            }
        }
    }
    pub fn dump(&mut self, first: bool) -> Vec<u8> {
        let mut data: Vec<u8> = vec![];
        let mut body: Vec<u8> = vec![];
        data.extend((self.dimensions.x as u16).to_le_bytes());
        data.extend((self.dimensions.y as u16).to_le_bytes());
        let buffer = self.world.clone();
        for (y, row) in buffer.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if cell.updated() || (first && cell.element() != Element::Air) {
                    body.extend((x as u16).to_le_bytes());
                    body.extend((y as u16).to_le_bytes());
                    body.push(cell.element().to_byte());
                    body.extend((cell.variant() as u8).to_le_bytes());
                    self.world[y][x].reset_update();
                }
            }
        }
        data.extend(((body.len() / 6) as u32).to_le_bytes());
        data.extend(body);
        data
    }

    fn in_bounds(&self, position: &Vector2D<usize>) -> bool {
        position.x < self.dimensions.x && position.y < self.dimensions.y
    }

    pub fn at(
        &self,
        from: &Vector2D<usize>,
        direction: Direction,
    ) -> Option<(Cell, Vector2D<usize>)> {
        let factor = direction.factor();
        let destination = Vector2D {
            x: (from.x as isize + direction.distance() as isize * factor.x as isize) as usize,
            y: (from.y as isize + direction.distance() as isize * factor.y as isize) as usize,
        };
        if !self.in_bounds(&from) || !self.in_bounds(&destination) {
            return None;
        }
        Some((self.world[destination.y][destination.x], destination))
    }

    pub fn apply_actions(&mut self, action: Action) {
        match action {
            Action::Move(mut from, to) => {
                for _ in 1..=to.distance() {
                    let factor = to.factor();
                    let destination = Vector2D {
                        x: (from.x as isize + factor.x as isize) as usize,
                        y: (from.y as isize + factor.y as isize) as usize,
                    };
                    self.swap(&from, &destination);
                    from = destination;
                }
            }
            Action::Eat(position, replacement) => {
                self.world[position.y][position.x].element = replacement;
            }
            Action::Burn(position) => {
                let source = self.world[position.y][position.x].element();
                let mut rng = rand::thread_rng();
                for neighbour in self.get_neighbours(&position) {
                    let ignite = rng.gen_bool(neighbour.0.element().flammability() * source.heat());
                    if ignite {
                        if neighbour.0.element() == Element::Wood {
                            self.world[neighbour.1.y][neighbour.1.x].element = Element::Ember;
                        } else if neighbour.0.element() == Element::Coal {
                            self.world[neighbour.1.y][neighbour.1.x].element = Element::Ember;
                        } else {
                            self.world[neighbour.1.y][neighbour.1.x] = Cell::new(Element::Fire);
                        }
                        self.world[neighbour.1.y][neighbour.1.x].set_update();
                    }
                    if source == Element::Ember
                        && neighbour.0.element() == Element::Air
                        && rng.gen_bool(0.005)
                    {
                        if rng.gen_bool(0.5) {
                            self.world[neighbour.1.y][neighbour.1.x] = Cell::new(Element::Smoke);
                        } else {
                            self.world[neighbour.1.y][neighbour.1.x] = Cell::new(Element::Fire);
                        }
                        self.world[neighbour.1.y][neighbour.1.x].set_update();
                    }
                    if source == Element::Ember && neighbour.0.element() == Element::Water {
                        self.world[position.y][position.x] = Cell::new(Element::Coal);
                        self.world[position.y][position.x].set_update();
                    }
                    if source == Element::Lava {
                        if neighbour.0.element() == Element::Water {
                            self.world[position.y][position.x] = Cell::new(Element::Stone);
                            self.world[position.y][position.x].set_update();
                            self.world[neighbour.1.y][neighbour.1.x] = Cell::new(Element::Stone);
                            self.world[neighbour.1.y][neighbour.1.x].set_update();
                        }
                        if neighbour.0.element() == Element::Ice {
                            self.world[neighbour.1.y][neighbour.1.x] = Cell::new(Element::Water);
                            self.world[neighbour.1.y][neighbour.1.x].set_update();
                        }
                        if neighbour.0.element() == Element::Air && rng.gen_bool(0.01) {
                            if rng.gen_bool(0.9) {
                                self.world[neighbour.1.y][neighbour.1.x] = Cell::new(Element::Smoke);
                            } else {
                                self.world[neighbour.1.y][neighbour.1.x] = Cell::new(Element::Fire);
                            }
                            self.world[neighbour.1.y][neighbour.1.x].set_update();
                        }
                    }
                    if source == Element::Acid && neighbour.0.element() == Element::Air {
                        if rng.gen_bool(0.01) {
                            self.world[neighbour.1.y][neighbour.1.x] = Cell::new(Element::Gas);
                        }
                        if rng.gen_bool(0.01) {
                            self.world[neighbour.1.y][neighbour.1.x] = Cell::new(Element::Smoke);
                        }
                    }
                    if source == Element::Air && neighbour.0.element() == Element::Fire {
                        if rng.gen_bool(0.5) {
                            self.world[position.y][position.x] = Cell::new(Element::Fire);
                            return;
                        }
                    }
                }
            }
            Action::Grow(position) => {
                let mut rng = rand::thread_rng();
                for neighbour in self.get_neighbours(&position) {
                    if rng.gen_bool(0.005) {
                        if neighbour.0.element() == Element::Water {
                            self.world[neighbour.1.y][neighbour.1.x] = Cell::new(Element::Moss);
                            self.world[neighbour.1.y][neighbour.1.x].set_update();

                        }
                    }
                }
            }
            Action::Disolve(position) => {
                let mut rng = rand::thread_rng();
                for neighbour in self.get_neighbours(&position) {
                    if rng.gen_bool(0.005) {
                        if neighbour.0.element() == Element::Water {
                            self.world[position.y][position.x] = Cell::new(Element::Water);
                            self.world[position.y][position.x].set_update();
                        }
                    }
                }
            }
            Action::Liquidize(position) => {
                let mut rng = rand::thread_rng();
                for neighbour in self.get_neighbours(&position) {
                    if rng.gen_bool(0.1) {
                        if neighbour.0.element() == Element::Fire || neighbour.0.element() == Element::Ember || neighbour.0.element() == Element::Smoke {
                            self.world[position.y][position.x] = Cell::new(Element::Water);
                            self.world[position.y][position.x].set_update();
                        }
                    }
                }
            }
            _ => {}
        }
    }

    pub fn swap(&mut self, from: &Vector2D<usize>, to: &Vector2D<usize>) {
        let temp = self.world[from.y][from.x];
        self.world[from.y][from.x] = self.world[to.y][to.x];
        self.world[to.y][to.x] = temp;

        self.world[from.y][from.x].set_update();
        self.world[to.y][to.x].set_update();
    }

    pub fn get_neighbours(&self, position: &Vector2D<usize>) -> Vec<(Cell, Vector2D<usize>)> {
        let mut neighbours: Vec<(Cell, Vector2D<usize>)> = vec![];
        for orientation in Cardinal::iter() {
            if let Some((cell, destination)) = self.at(position, Direction::new(orientation, 1)) {
                neighbours.push((cell, destination));
            }
        }
        neighbours
    }
}
