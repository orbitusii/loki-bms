use crate::tracking::{DataType, Datapoint, Position};

#[derive(Debug, Default, Clone)]
pub struct GenData {
    pub source_number: u32,

    pub lat: f64,
    pub lon: f64,
    pub alt: Distance,

    pub callsign: Option<String>,
}

#[derive(Debug, Clone)]
pub enum Distance {
    Meters(f32),
    Feet(i32),
}

impl Default for Distance {
    fn default() -> Self {
        Self::Feet(0)
    }
}

impl Into<Datapoint> for GenData {
    fn into(self) -> Datapoint {
        let altitude = match self.alt {
            Distance::Meters(m) => (m * 3.28084) as i32,
            Distance::Feet(f) => f,
        };

        Datapoint {
            source: DataType::Real,
            source_number: self.source_number,

            position: Position::new(self.lat, self.lon, altitude),

            ..Datapoint::default()
        }
    }
}
