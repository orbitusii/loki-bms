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

    //TODO: implement according to Issue #2 and move to Server crate
    #[cfg(feature = "server")]
    pub fn correlate_data(&mut self, datapoint: tracking::Datapoint) -> Result<(), Error> {}
}
