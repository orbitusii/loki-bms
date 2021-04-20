use crate::tracking::{Datapoint, Datasource, LinkData, Position};

#[derive(Debug, Default, Clone)]
pub struct DcsData {
    pub id: u32,

    pub lat: f32,
    pub lon: f32,
    pub alt: f32,

    pub callsign: String,
    pub fuel: f32,
}

impl From<DcsData> for Datapoint {
    fn from(data: DcsData) -> Self {
        Datapoint {
            source: Datasource::SELF,
            source_number: data.id,

            position: Position::new(
                data.lat as f64,
                data.lon as f64,
                (data.alt * 3.28084) as i32,
            ),

            link_data: Some(LinkData {
                callsign: data.callsign,
                fuel: Some(data.fuel),
                weapons: None,
            }),

            ..Datapoint::default()
        }
    }
}
