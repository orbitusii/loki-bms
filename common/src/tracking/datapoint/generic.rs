use crate::tracking::{Datapoint, Datasource, Position};

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

impl From<GenData> for Datapoint {
    fn from(data: GenData) -> Self {
        let altitude = match data.alt {
            Distance::Meters(m) => (m * 3.28084) as i32,
            Distance::Feet(f) => f,
        };

        Datapoint {
            source: Datasource::SELF,
            source_number: data.source_number,

            position: Position::new(data.lat, data.lon, altitude),

            ..Datapoint::default()
        }
    }
}
