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
