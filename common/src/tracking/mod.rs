use std::{u32, usize};

pub use self::datapoint::*;
pub use self::link_data::*;
pub use self::position::*;
use kd_tree;

mod datapoint;
mod link_data;
mod position;

#[cfg(tests)]
mod tests;

#[derive(Debug, Clone)]
pub struct Track {
    pub track_number: u32,
    pub id: ID,

    pub position: Position,

    datapoints: Vec<Datapoint>,
    pub history: Vec<Datapoint>,

    pub link_data: Option<LinkData>,
}

impl Track {
    pub fn update(&mut self, datapoint: Datapoint) {
        self.datapoints.push(datapoint);
    }
}

impl Default for Track {
    fn default() -> Self {
        Track {
            track_number: 0,
            id: ID::PND,

            position: Position::default(),

            datapoints: Vec::default(),
            history: Vec::default(),

            link_data: None,
        }
    }
}

impl PartialEq for Track {
    fn eq(&self, other: &Track) -> bool {
        self.track_number == other.track_number
    }
}

impl kd_tree::KdPoint for Track {
    type Scalar = f64;
    type Dim = typenum::U2;

    fn at(&self, i: usize) -> Self::Scalar {
        self.position.at(i)
    }
}

#[derive(Debug, Clone)]
pub enum ID {
    FND,
    ASF,
    NEU,
    UNK,
    SUS,
    HOS,
    PND,
}
