pub mod constants;
pub mod doppler_factor;
pub mod propagation;
pub mod transforms; // todo remove
pub mod types;
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
pub struct Position {
    pub longitude: f64,
    pub latitude: f64,
    pub height: f64,
}

pub struct Topocentric {
    top_s: f64,
    top_e: f64,
    top_z: f64,
}

pub struct LookAngles {
    azimuth: f64,
    elevation: f64,
    range_sat: f64,
}
