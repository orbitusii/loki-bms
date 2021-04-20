use geo::{self, Point};

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Position {
    pub lat_lon: Point<f64>,
    pub altitude: i32,
}

impl Position {
    pub fn new(lat: f64, lon: f64, altitude: i32) -> Self {
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
