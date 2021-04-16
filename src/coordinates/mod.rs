use std::fmt;

/// Linear data unit scale, 100 units = 1 meter (i.e. cm scale)
pub const SCALE_LINEAR: i32 = 100;
/// Inverse of SCALE_LINEAR for conversion back to meter-normalized coordinates
pub const INV_SCALE_LIN: f32 = 1.0 / SCALE_LINEAR as f32;

/// Angular data unit scale, 10,000 units = 1 degree
pub const SCALE_ANGULAR: i32 = 10000;
/// Inverse of SCALE_ANGULAR for conversion back to degree-normalized coordinates
pub const INV_SCALE_ANG: f32 = 1.0 / SCALE_ANGULAR as f32;

/// Earth's radius in meters
pub const RADIUS_F: f32 = 6378137.0;
/// Earth's radius in data units
pub const RADIUS_I: i32 = RADIUS_F as i32 * SCALE_LINEAR;

/// Excessively long value of Pi for use in detailed angular conversions
pub const PI: f64 = 
3.1415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421171;

/// Point in linear/cartesian space, measured in data units
// PtLinear::zero(), or (0, 0, 0), is Earth's center
pub struct PtLinear {
    x: i32,
    y: i32,
    z: i32,
}

impl PtLinear {
    pub fn zero() -> PtLinear {
        PtLinear {
            x: 0,
            y: 0,
            z: 0,
        }
    }

    /// Converts linear coordinates to angular coordinates
    pub fn to_angular(&self) -> PtAngular {
        PtAngular {
            lat: 0,
            lon: 0,
            alt: -RADIUS_I,
        }
    }

    pub fn to_string(&self) -> String {
        format!( "({}, {}, {})",
            self.x as f32 * INV_SCALE_LIN,
            self.y as f32 * INV_SCALE_LIN,
            self.z as f32 * INV_SCALE_LIN)
    }
}

impl PartialEq for PtLinear {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x
        && self.y == other.y
        && self.z == other.z
    }
}

impl fmt::Display for PtLinear {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})",
            self.x as f32 * INV_SCALE_LIN,
            self.y as f32 * INV_SCALE_LIN,
            self.z as f32 * INV_SCALE_LIN)
    }
}

impl fmt::Debug for PtLinear {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}

/// Point in angular space (i.e. lat/long/alt), measured in data units
// PtAngular::zero(), or (0N, 0E, 0), is MSL (1 Earth radius)
// at 0 degrees Latitude, 0 degrees Longitude
// +lat is North, -lat is South
// +lon is East, -lon is West
// +alt is above sea level, -alt is subsurface
pub struct PtAngular {
    lat: i32,
    lon: i32,
    alt: i32,
}

impl PtAngular {
    pub fn zero() -> PtAngular {
        PtAngular {
            lat: 0,
            lon: 0,
            alt: 0,
        }
    }
    
    /// Converts angular coordinates to linear coordinates
    pub fn to_linear(&self) -> PtLinear {
        PtLinear {
            x: 0,
            y: 0,
            z: -RADIUS_I,
        }
    }

    pub fn to_string(&self) -> String {
        let lat_char;
        if self.lat < 0 {
            lat_char = 'S';
        }
        else {
            lat_char = 'N';
        }

        let lon_char;
        if self.lon < 0 {
            lon_char = 'W';
        }
        else {
            lon_char = 'E';
        }

        format!("({}{}, {}{}, {})",
            self.lat.abs() as f32 * INV_SCALE_ANG, lat_char,
            self.lon.abs() as f32 * INV_SCALE_ANG, lon_char,
            self.alt as f32 * INV_SCALE_LIN)
    }
}

impl PartialEq for PtAngular {
    fn eq(&self, other: &Self) -> bool {
        self.lat == other.lat
        && self.lon == other.lon
        && self.alt == other.alt
    }
}

impl fmt::Display for PtAngular {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lat_char;
        if self.lat < 0 {
            lat_char = 'S';
        }
        else {
            lat_char = 'N';
        }

        let lon_char;
        if self.lon < 0 {
            lon_char = 'W';
        }
        else {
            lon_char = 'E';
        }

        write!(f, "({}{}, {}{}, {})",
            self.lat.abs() as f32 * INV_SCALE_ANG, lat_char,
            self.lon.abs() as f32 * INV_SCALE_ANG, lon_char,
            self.alt as f32 * INV_SCALE_LIN)
    }
}

impl fmt::Debug for PtAngular {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linear_to_angular () {
        let lin_temp = PtLinear::zero();
        let ang_temp = PtAngular {
            lat: 0,
            lon: 0,
            alt: -RADIUS_I,
        };

        assert_eq!(lin_temp.to_angular(), ang_temp);
    }

    #[test]
    fn angular_to_linear () {
        let ang_temp = PtAngular::zero();
        let lin_temp = PtLinear {
            x: 0,
            y: 0,
            z: -RADIUS_I,
        };

        assert_eq!(ang_temp.to_linear(), lin_temp);
    }

    #[test]
    fn print_linear() {
        let lin_temp = PtLinear {
            x: 1011,
            y: 1011,
            z: 1011,
        };

        assert_eq!(lin_temp.to_string(), "(10.11, 10.11, 10.11)");
    }

    #[test]
    fn print_angular() {
        let ang_temp = PtAngular {
            lat: 101010,
            lon: 101010,
            alt: 1011,
        };

        assert_eq!(ang_temp.to_string(), "(10.101N, 10.101E, 10.11)");
    }

}