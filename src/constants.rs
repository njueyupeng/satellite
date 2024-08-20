use std::f64::consts;

pub const PI: f64 = consts::PI;
pub const TWO_PI: f64 = PI * 2.0;
pub const DEG2RAD: f64 = PI / 180.0;
pub const RAD2DEG: f64 = 180.0 / PI;
pub const MINUTES_PER_DAY: f64 = 1440.0;
pub const MU: f64 = 398600.8; // in km^3 / s^2
pub const EARTH_RADIUS: f64 = 6378.135; // in km

pub const XKE: f64 = 0.07436691613317342; // 60.0 / ((EARTH_RADIUS.powi(3) / MU).sqrt())
pub const VKMPERSEC: f64 = (EARTH_RADIUS * XKE) / 60.0;
pub const TUMIN: f64 = 1.0 / XKE;
pub const J2: f64 = 0.001082616;
pub const J3: f64 = -0.00000253881;
pub const J4: f64 = -0.00000165597;
pub const J3OJ2: f64 = J3 / J2;
pub const X2O3: f64 = 2.0 / 3.0;

#[cfg(test)]
mod test {
    use crate::constants::*;

    pub fn xke() -> f64 {
        60.0 / ((EARTH_RADIUS.powi(3) / MU).sqrt())
    }

    pub fn vkmpersec() -> f64 {
        (EARTH_RADIUS * XKE) / 60.0
    }
    pub fn tumin() -> f64 {
        1.0 / XKE
    }
    #[test]
    fn test() {
        assert_eq!(XKE, xke());
        assert_eq!(VKMPERSEC, vkmpersec());
        assert_eq!(TUMIN, tumin());
    }
}
