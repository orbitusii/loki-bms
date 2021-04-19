use std::{cell::Cell, u32, usize};

use chrono::prelude::*;
use geo::{self, Point};

use self::datalink::LinkData;
use kd_tree;

pub mod datalink;

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

/// A generic data point
#[derive(Debug, Clone)]
pub struct Datapoint {
    pub source: Datasource,
    pub source_number: u32,

    pub timestamp: DateTime<Utc>,

    pub position: Position,

    pub link_data: Option<LinkData>,
}

impl Default for Datapoint {
    fn default() -> Self {
        Datapoint {
            source: Datasource::SELF,
            source_number: 0,

            timestamp: Utc::now(),

            position: Position::default(),
            link_data: None,
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Position {
    pub lat_lon: Point<f64>,
    pub altitude: i32,
}

impl Position {
    fn new(lat: f64, lon: f64, altitude: i32) -> Self {
        Position {
            lat_lon: Point::new(lat, lon),
            altitude,
        }
    }
}

impl kd_tree::KdPoint for Position {
    type Scalar = f64;
    type Dim = typenum::U2;

    fn at(&self, i: usize) -> Self::Scalar {
        match i {
            0 => self.lat_lon.x(),
            1 => self.lat_lon.y(),
            _ => 0.0,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Datasource {
    SELF,
}

#[cfg(test)]
mod tests {
    pub struct TestDatapoint {
        pub id: i32,

        pub lat: f32,
        pub lon: f32,
        pub alt: f32,
    }

    use super::{Datapoint, Position};

    impl From<TestDatapoint> for Datapoint {
        fn from(test_data: TestDatapoint) -> Self {
            Datapoint {
                position: Position::new(
                    test_data.lat as f64,
                    test_data.lon as f64,
                    (test_data.alt * 3.28084) as i32,
                ),
                ..Datapoint::default()
            }
        }
    }

    #[test]
    fn test_from_test_point() {
        let tdp = TestDatapoint {
            id: 10000,
            lat: 0.0,
            lon: 45.0,
            alt: 100.0,
        };
        let point = Datapoint::from(tdp);

        assert_eq!(point.position, Position::new(0.0, 45.0, 328));
        assert_ne!(point.timestamp, chrono::Utc::now());
    }
}
