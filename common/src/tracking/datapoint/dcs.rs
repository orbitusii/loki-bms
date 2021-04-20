use crate::tracking::{DataType, Datapoint, LinkData, Position};

#[derive(Debug, Default, Clone)]
pub struct DcsData {
    pub id: u32,

    pub lat: f32,
    pub lon: f32,
    pub alt: f32,

    pub callsign: String,
    pub fuel: f32,
}

impl Into<Datapoint> for DcsData {
    fn into(self) -> Datapoint {
        Datapoint {
            source: DataType::Real,
            source_number: self.id,

            position: Position::new(
                self.lat as f64,
                self.lon as f64,
                (self.alt * 3.28084) as i32,
            ),

            link_data: Some(LinkData {
                callsign: self.callsign,
                fuel: Some(self.fuel),
                weapons: None,
            }),

            ..Datapoint::default()
        }
    }
}
