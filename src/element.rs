extern crate cfg_if;
extern crate wasm_bindgen;
extern crate web_sys;
use wasm_bindgen::prelude::*;
use std::fmt::{Display, Formatter};

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Element {
    Air,
    Water,
    Sand,
    Wood,
    Fire,
    Smoke,
    Acid,
    Ember,
    Gas,
    Stone,
    Coal,
    Salt,
    Cinder,
    Lava,
    Oil,
    Moss,
    CanonPowder,
    Ice,
}

pub trait Physics {
    fn density(&self) -> f64;
    fn flammability(&self) -> f64;
    fn solid(&self) -> bool;
    fn heat(&self) -> f64;
}

impl Physics for Element {
    fn density(&self) -> f64 {
        match self {
            Self::Air => 1.0,
            Self::Water => 2.0,
            Self::Sand => 3.0,
            Self::Wood => 10.0,
            Self::Fire => 0.1,
            Self::Smoke => 0.5,
            Self::Acid => 1.5,
            Self::Ember => 4.0,
            Self::Gas => 1.0,
            Self::Stone => 20.0,
            Self::Coal => 10.0,
            Self::Salt => 2.2,
            Self::Cinder => 1.2,
            Self::Lava => 5.0,
            Self::Oil => 1.5,
            Self::Moss => 1.5,
            Self::CanonPowder => 3.0,
            Self::Ice => 1.8,
        }
    }
    fn flammability(&self) -> f64 {
        match self {
            Self::Air => 0.0,
            Self::Water => 0.0,
            Self::Sand => 0.0,
            Self::Wood => 0.05,
            Self::Fire => 0.0,
            Self::Smoke => 0.0,
            Self::Acid => 0.0,
            Self::Ember => 0.0,
            Self::Gas => 1.0,
            Self::Stone => 0.0,
            Self::Coal => 0.01,
            Self::Salt => 0.0,
            Self::Cinder => 0.0,
            Self::Lava => 0.0,
            Self::Oil => 0.01,
            Self::Moss => 0.1,
            Self::CanonPowder => 1.0,
            Self::Ice => 0.0,
        }
    }
    fn solid(&self) -> bool {
        match self {
            Self::Air => false,
            Self::Water => false,
            Self::Sand => true,
            Self::Wood => true,
            Self::Fire => false,
            Self::Smoke => false,
            Self::Acid => false,
            Self::Ember => true,
            Self::Gas => false,
            Self::Stone => true,
            Self::Coal => true,
            Self::Salt => true,
            Self::Cinder => false,
            Self::Lava => false,
            Self::Oil => false,
            Self::Moss => true,
            Self::CanonPowder => true,
            Self::Ice => true,
        }
    }
    fn heat(&self) -> f64 {
        match self {
            Self::Air => 0.0,
            Self::Water => 0.0,
            Self::Sand => 0.0,
            Self::Wood => 0.0,
            Self::Fire => 1.0,
            Self::Smoke => 0.0,
            Self::Acid => 0.0,
            Self::Ember => 0.4,
            Self::Gas => 0.0,
            Self::Stone => 0.0,
            Self::Coal => 0.0,
            Self::Salt => 0.0,
            Self::Cinder => 0.0,
            Self::Lava => 1.0,
            Self::Oil => 0.0,
            Self::Moss => 0.0,
            Self::CanonPowder => 0.0,
            Self::Ice => 0.0,
        }
    }
}

impl Display for Element {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::Air => {
                write!(f, "·").unwrap();
            }
            Self::Water => {
                write!(f, "~").unwrap();
            }
            Self::Sand => {
                write!(f, "¤").unwrap();
            }
            Self::Wood => {
                write!(f, "▓").unwrap();
            }
            _ => {}
        }
        Ok(())
    }
}

impl Element {
    pub fn from_byte(byte: u8) -> Result<Self, ()> {
        match byte {
            0 => Ok(Self::Air),
            1 => Ok(Self::Water),
            2 => Ok(Self::Sand),
            3 => Ok(Self::Wood),
            4 => Ok(Self::Fire),
            5 => Ok(Self::Smoke),
            6 => Ok(Self::Acid),
            7 => Ok(Self::Ember),
            8 => Ok(Self::Gas),
            9 => Ok(Self::Stone),
            10 => Ok(Self::Coal),
            11 => Ok(Self::Salt),
            12 => Ok(Self::Cinder),
            13 => Ok(Self::Lava),
            14 => Ok(Self::Oil),
            15 => Ok(Self::Moss),
            16 => Ok(Self::CanonPowder),
            17 => Ok(Self::Ice),
            _ => Err(()),
        }
    }
    pub fn to_byte(&self) -> u8 {
        match self {
            Self::Air => 0,
            Self::Water => 1,
            Self::Sand => 2,
            Self::Wood => 3,
            Self::Fire => 4,
            Self::Smoke => 5,
            Self::Acid => 6,
            Self::Ember => 7,
            Self::Gas => 8,
            Self::Stone => 9,
            Self::Coal => 10,
            Self::Salt => 11,
            Self::Cinder => 12,
            Self::Lava => 13,
            Self::Oil => 14,
            Self::Moss => 15,
            Self::CanonPowder => 16,
            Self::Ice => 17,
        }
    }
}
