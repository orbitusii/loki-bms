use crate::tracking::{LinkData, Position};
use chrono::prelude::*;

//#[cfg(all(feature = "server", feature = "dcs"))]
mod dcs;
//#[cfg(all(feature = "server", feature = "dcs"))]
pub use dcs::DcsData;

//#[cfg(all(feature = "server", feature = "generic"))]
mod generic;
//#[cfg(all(feature = "server", feature = "generic"))]
pub use generic::GenData;

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

#[derive(Debug, Clone)]
pub enum Datasource {
    SELF,
}
