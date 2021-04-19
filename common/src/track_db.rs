use std::{fmt::Error, ops::Index, usize};

use geo::geodesic_distance::GeodesicDistance;
use kd_tree;

use crate::tracking;

#[derive(Debug)]
pub struct TrackDb {
    pub tracks: Vec<tracking::Track>,
    pub free_data: Vec<tracking::Datapoint>,

    pub tree: kd_tree::KdTree<tracking::Track>,
}

impl TrackDb {
    pub fn update_tree(&mut self) {
        self.tree = kd_tree::KdTree::build_by_ordered_float(self.tracks.clone());
    }

    pub fn correlate_data(&mut self, datapoint: tracking::Datapoint) -> Result<(), Error> {
        let nearest = self.tree.nearest(&datapoint.position).unwrap().item;

        let dist = nearest
            .position
            .lat_lon
            .geodesic_distance(&datapoint.position.lat_lon);

        if dist < 3.0 * 1852.001 {
            let idx = self
                .tracks
                .iter()
                .position(|x: &tracking::Track| x == nearest)
                .unwrap();
            self.tracks[idx].update(datapoint);
            Ok(())
        } else {
            Err(Error::default())
        }
    }
}

#[cfg(tests)]
mod tests {
    #[test]
    fn track_correlates() {}
}
