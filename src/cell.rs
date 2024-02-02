use std::fmt::{Display, Formatter};

use rand::{thread_rng, Rng};

use crate::direction::Cardinal;
use crate::element::Physics;
use crate::{simulation, Action, Simulation, Vector2D};
use crate::{Direction, Element};

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    pub element: Element,
    pub velocity: Vector2D<i8>,
    pub life: Option<u32>,
    variant: u8,
    updated: bool,
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.element)
    }
}

impl Cell {
    pub fn new(element: Element) -> Self {
        let mut rng = rand::thread_rng();
        let variant = rng.gen_range(0..=255);
        let velocity = Vector2D { x: 0, y: 0 };
        let mut life = None;
        match element {
            Element::Wood => life = Some(rng.gen_range(150..=300)),
            Element::Coal => life = Some(rng.gen_range(500..=650)),
            Element::Fire => life = Some(5),
            Element::Smoke => life = Some(20),
            Element::Acid => life = Some(rng.gen_range(200..=2000)),
            _ => {}
        }
        Self {
            element,
            velocity,
            life,
            variant,
            updated: false,
        }
    }
    pub fn update(&self, origin: Vector2D<usize>, sim: &Simulation) -> Vec<Action> {
        type Dir = Direction;
        type Car = Cardinal;
        let mut actions: Vec<Action> = vec![];
        let mut rng = rand::thread_rng();
        match self.element {
            Element::Air => {}
            Element::Water => {
                let first = rng.gen_bool(0.5);
                if let Some(action) = self.move_to(origin, Dir::new(Car::S, 3), sim) {
                    actions.push(action);
                } else if let Some(action) = self.move_to(
                    origin,
                    Dir::new(if first { Car::SE } else { Car::SW }, 3),
                    sim,
                ) {
                    actions.push(action);
                } else if let Some(action) = self.move_to(
                    origin,
                    Dir::new(if first { Car::SW } else { Car::SE }, 3),
                    sim,
                ) {
                    actions.push(action);
                } else if let Some(action) = self.move_to(
                    origin,
                    Dir::new(if first { Car::E } else { Car::W }, 3),
                    sim,
                ) {
                    actions.push(action);
                } else if let Some(action) = self.move_to(
                    origin,
                    Dir::new(if first { Car::W } else { Car::E }, 3),
                    sim,
                ) {
                    actions.push(action);
                }
            }
            Element::Acid => {
                let first = rng.gen_bool(0.5);
                if let Some(action) = self.move_to(origin, Dir::new(Car::S, 1), sim) {
                    actions.push(action);
                } else if let Some(action) = self.move_to(
                    origin,
                    Dir::new(if first { Car::SE } else { Car::SW }, 1),
                    sim,
                ) {
                    actions.push(action);
                } else if let Some(action) = self.move_to(
                    origin,
                    Dir::new(if first { Car::SW } else { Car::SE }, 1),
                    sim,
                ) {
                    actions.push(action);
                } else if let Some(action) = self.move_to(
                    origin,
                    Dir::new(if first { Car::E } else { Car::W }, 1),
                    sim,
                ) {
                    actions.push(action);
                } else if let Some(action) = self.move_to(
                    origin,
                    Dir::new(if first { Car::W } else { Car::E }, 1),
                    sim,
                ) {
                    actions.push(action);
                }
                actions.extend(self.eat_neighbour(origin, sim));
                actions.push(Action::Burn(origin));
            }
            Element::Lava => {
                actions.push(Action::Burn(origin));
                let first = rng.gen_bool(0.5);
                if let Some(action) = self.move_to(origin, Dir::new(Car::S, 1), sim) {
                    actions.push(action);
                } else if let Some(action) = self.move_to(
                    origin,
                    Dir::new(if first { Car::SE } else { Car::SW }, 1),
                    sim,
                ) {
                    actions.push(action);
                } else if let Some(action) = self.move_to(
                    origin,
                    Dir::new(if first { Car::SW } else { Car::SE }, 1),
                    sim,
                ) {
                    actions.push(action);
                } else if let Some(action) = self.move_to(
                    origin,
                    Dir::new(if first { Car::E } else { Car::W }, 1),
                    sim,
                ) {
                    actions.push(action);
                } else if let Some(action) = self.move_to(
                    origin,
                    Dir::new(if first { Car::W } else { Car::E }, 1),
                    sim,
                ) {
                    actions.push(action);
                }
            }
            Element::Oil => {
                if let Some((cell, position)) = sim.at(&origin, Dir::new(Car::N, 1)) {
                    if cell.element() == Element::Air {
                        actions.push(Action::Burn(position))
                    }
                }

                let first = rng.gen_bool(0.5);
                if let Some(action) = self.move_to(origin, Dir::new(Car::S, 1), sim) {
                    actions.push(action);
                } else if let Some(action) = self.move_to(
                    origin,
                    Dir::new(if first { Car::SE } else { Car::SW }, 1),
                    sim,
                ) {
                    actions.push(action);
                } else if let Some(action) = self.move_to(
                    origin,
                    Dir::new(if first { Car::SW } else { Car::SE }, 1),
                    sim,
                ) {
                    actions.push(action);
                } else if let Some(action) = self.move_to(
                    origin,
                    Dir::new(if first { Car::E } else { Car::W }, 1),
                    sim,
                ) {
                    actions.push(action);
                } else if let Some(action) = self.move_to(
                    origin,
                    Dir::new(if first { Car::W } else { Car::E }, 1),
                    sim,
                ) {
                    actions.push(action);
                }
            }
            Element::Sand => {
                if let Some(action) = self.move_to(origin, Dir::new(Car::S, 2), sim) {
                    actions.push(action);
                } else if let Some(action) = self.move_to(origin, Dir::new(Car::SW, 2), sim) {
                    actions.push(action);
                } else if let Some(action) = self.move_to(origin, Dir::new(Car::SE, 2), sim) {
                    actions.push(action);
                }
            }
            Element::Salt => {
                if let Some(action) = self.move_to(origin, Dir::new(Car::S, 2), sim) {
                    actions.push(action);
                } else if let Some(action) = self.move_to(origin, Dir::new(Car::SW, 2), sim) {
                    actions.push(action);
                } else if let Some(action) = self.move_to(origin, Dir::new(Car::SE, 2), sim) {
                    actions.push(action);
                }
                actions.push(Action::Disolve(origin));
            }
            Element::CanonPowder => {
                if let Some(action) = self.move_to(origin, Dir::new(Car::S, 2), sim) {
                    actions.push(action);
                } else if let Some(action) = self.move_to(origin, Dir::new(Car::SW, 2), sim) {
                    actions.push(action);
                } else if let Some(action) = self.move_to(origin, Dir::new(Car::SE, 2), sim) {
                    actions.push(action);
                }
            }
            Element::Cinder => {
                if let Some(action) = self.move_to(origin, Dir::new(Car::S, 2), sim) {
                    actions.push(action);
                } else if let Some(action) = self.move_to(origin, Dir::new(Car::SW, 2), sim) {
                    actions.push(action);
                } else if let Some(action) = self.move_to(origin, Dir::new(Car::SE, 2), sim) {
                    actions.push(action);
                }
            }
            Element::Fire => {
                actions.push(Action::Burn(origin));
                let dir = rng.gen_range(0..12);
                if dir == 0 {
                    if let Some(action) = self.move_to(origin, Dir::new(Car::E, 1), sim) {
                        actions.push(action);
                    }
                } else if dir == 1 {
                    if let Some(action) = self.move_to(origin, Dir::new(Car::W, 1), sim) {
                        actions.push(action);
                    }
                } else if dir == 2 || dir == 3 {
                    if let Some(action) = self.move_to(origin, Dir::new(Car::NE, 1), sim) {
                        actions.push(action);
                    }
                } else if dir == 4 || dir == 5 {
                    if let Some(action) = self.move_to(origin, Dir::new(Car::NW, 1), sim) {
                        actions.push(action);
                    }
                } else {
                    if let Some(action) = self.move_to(origin, Dir::new(Car::N, 1), sim) {
                        actions.push(action);
                    }
                }
            }
            Element::Smoke => {
                let first = rng.gen_bool(0.5);
                if let Some(action) = self.move_to(origin, Dir::new(Car::N, 1), sim) {
                    actions.push(action);
                }
                if first {
                    if let Some(action) = self.move_to(origin, Dir::new(Car::NE, 1), sim) {
                        actions.push(action);
                    }
                    if let Some(action) = self.move_to(origin, Dir::new(Car::E, 1), sim) {
                        actions.push(action);
                    }
                } else {
                    if let Some(action) = self.move_to(origin, Dir::new(Car::NW, 1), sim) {
                        actions.push(action);
                    }
                    if let Some(action) = self.move_to(origin, Dir::new(Car::W, 1), sim) {
                        actions.push(action);
                    }
                }
            }
            Element::Ember => {
                actions.push(Action::Burn(origin));
            }
            Element::Moss => {
                actions.push(Action::Grow(origin));
            }
            Element::Ice => {
                actions.push(Action::Liquidize(origin));
            }
            Element::Gas => {
                let distance = rng.gen_range(1..=2);
                let direction = Dir::new_rng(distance);
                if let Some(action) = self.move_to(origin, direction, sim) {
                    actions.push(action);
                }
            }
            _ => {}
        }
        actions
    }

    fn move_to(
        &self,
        from: Vector2D<usize>,
        mut to: Direction,
        simulation: &Simulation,
    ) -> Option<Action> {
        let distance = to.distance;
        let mut destination: Option<Action> = None;
        for i in 1..=distance {
            to.set_distance(i);
            if let Some((cell, _)) = simulation.at(&from, to) {
                if cell.element.solid() {
                    break;
                }
                if cell.element == self.element {
                    break;
                }
                if self.element == Element::Gas && cell.element == Element::Air {
                    destination = Some(Action::Move(from, to));
                    continue;
                }
                if to.factor().y == 1 && self.element.density() > cell.element.density() {
                    destination = Some(Action::Move(from, to));
                    continue;
                } else if to.factor().y == -1 && self.element.density() < cell.element.density() {
                    destination = Some(Action::Move(from, to));
                    continue;
                } else if to.factor().y == 0 && self.element.density() >= cell.element.density() {
                    destination = Some(Action::Move(from, to));
                    continue;
                }
            }
            break;
        }
        destination
    }

    fn eat_neighbour(&self, from: Vector2D<usize>, simulation: &Simulation) -> Vec<Action> {
        let mut rng = thread_rng();
        let mut actions: Vec<Action> = vec![];
        for (cell, at) in simulation.get_neighbours(&from) {
            if cell.element == Element::Air || cell.element == Element::Acid {
                continue;
            }
            let eat = rng.gen_bool(0.2 * (1.0 / (cell.element.density() * 3.0)));
            if eat {
                actions.push(Action::Eat(at, Element::Air));
            }
        }
        actions
    }

    pub fn element(&self) -> Element {
        self.element
    }

    pub fn variant(&self) -> u8 {
        self.variant
    }

    pub fn set_update(&mut self) {
        self.updated = true;
    }

    pub fn reset_update(&mut self) {
        self.updated = false;
    }

    pub fn updated(&self) -> bool {
        self.updated
    }

    pub fn decay(&mut self) {
        let mut transform = Element::Air;
        match self.life {
            Some(life) => {
                match self.element {
                    Element::Fire => {
                        self.life = Some(life - 1);
                    }
                    Element::Smoke => {
                        self.life = Some(life - 1);
                    }
                    Element::Ember => {
                        self.life = Some(life - 1);
                        if rand::thread_rng().gen_bool(0.5) {
                            transform = Element::Cinder;
                        }
                    }
                    Element::Acid => {
                        self.life = Some(life - 1);
                    }
                    _ => {}
                }
                if life == 1 {
                    *self = Cell::new(transform);
                    self.set_update();
                }
            }
            None => {}
        }
    }
}
