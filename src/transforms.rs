use crate::constants::{DEG2RAD, PI, RAD2DEG, TWO_PI};
use crate::LookAngles;
use crate::Position;
use crate::Topocentric;
use crate::Vector3;

const A: f64 = 6378.137;
const B: f64 = 6356.7523142;
const F: f64 = (A - B) / A;
const E2: f64 = (2.0 * F) - (F * F);

pub fn radians_to_degrees(radians: f64) -> f64 {
    radians * RAD2DEG
}

pub fn degrees_to_radians(degree: f64) -> f64 {
    degree * DEG2RAD
}

pub fn degrees_lat(radians: f64) -> f64 {
    if radians < (-PI / 2.0) || radians > (PI / 2.0) {
        panic!("Latitude radians must be in range [-pi/2; pi/2].")
    }
    radians_to_degrees(radians)
}

pub fn degrees_long(radians: f64) -> f64 {
    if radians < (-PI) || radians > (PI) {
        panic!("Latitude radians must be in range [-pi; pi].")
    }
    radians_to_degrees(radians)
}

pub fn radians_lat(degrees: f64) -> f64 {
    if degrees < (-90.0) || degrees > (90.0) {
        panic!("Latitude degrees must be in range [-90; 90].")
    }
    degrees_to_radians(degrees)
}

pub fn radians_long(degrees: f64) -> f64 {
    if degrees < (-180.0) || degrees > (180.0) {
        panic!("Longitude degrees must be in range [-180; 180].")
    }
    degrees_to_radians(degrees)
}

pub fn geodetic_to_ecf(geodetic: &Position) -> Vector3 {
    let latitude: f64 = geodetic.latitude;
    let longitude: f64 = geodetic.longitude;
    let height: f64 = geodetic.height;

    let normal = A / (1.0 - (E2 * latitude.sin() * latitude.sin())).sqrt();

    let x = (normal + height) * latitude.cos() * longitude.cos();
    let y = (normal + height) * latitude.cos() * longitude.sin();
    let z = ((normal * (1.0 - E2)) + height) * latitude.sin();

    Vector3 { x, y, z }
}

pub fn eci_to_geodetic(eci: &Vector3, gmst: f64) -> Position {
    // http://www.celestrak.com/columns/v02n03/
    let r: f64 = (eci.x * eci.x + eci.y * eci.y).sqrt();
    let mut longitude = eci.y.atan2(eci.x) - gmst;

    while longitude < -PI {
        longitude += TWO_PI;
    }

    while longitude > PI {
        longitude -= TWO_PI;
    }

    const KMAX: i32 = 20;
    let mut k = 0;
    let mut latitude = eci.z.atan2(r);

    let mut c = 1.0;
    while k < KMAX {
        c = 1.0 / (1.0 - (E2 * (latitude.sin().powi(2)))).sqrt();
        latitude = (eci.z + (A * c * E2 * latitude.sin())).atan2(r);
        k += 1;
    }

    let height = (r / latitude.cos()) - (A * c);
    return Position {
        longitude: longitude,
        latitude: latitude,
        height: height,
    };
}

pub fn ecf_to_eci(ecf: &Vector3, gmst: f64) -> Vector3 {
    //
    // [X]     [C -S  0][X]
    // [Y]  =  [S  C  0][Y]
    // [Z]eci  [0  0  1][Z]ecf
    //
    let x = ecf.x * gmst.cos() - ecf.y * gmst.sin();
    let y = ecf.x * gmst.sin() + ecf.y * gmst.cos();
    let z = ecf.z;

    Vector3 { x, y, z }
}

pub fn eci_to_ecf(eci: &Vector3, gmst: f64) -> Vector3 {
    // ccar.colorado.edu/ASEN5070/handouts/coordsys.doc
    //
    // [X]     [C -S  0][X]
    // [Y]  =  [S  C  0][Y]
    // [Z]eci  [0  0  1][Z]ecf
    //
    //
    // Inverse:
    // [X]     [C  S  0][X]
    // [Y]  =  [-S C  0][Y]
    // [Z]ecf  [0  0  1][Z]eci

    let x = eci.x * gmst.cos() + eci.y * gmst.sin();
    let y = -eci.x * gmst.sin() + eci.y * gmst.cos();
    let z = eci.z;

    Vector3 { x, y, z }
}

pub fn topocentric(observer_geodetic: &Position, satellite_ecf: &Vector3) -> Topocentric {
    let latitude = observer_geodetic.latitude;
    let longitude = observer_geodetic.longitude;
    let observer_ecf = geodetic_to_ecf(observer_geodetic);

    let rx = satellite_ecf.x - observer_ecf.x;
    let ry = satellite_ecf.y - observer_ecf.y;
    let rz = satellite_ecf.z - observer_ecf.z;

    let top_s = (latitude.sin() * longitude.cos() * rx) + (latitude.sin() * longitude.sin() * ry)
        - (latitude.cos() * rz);

    let top_e = (longitude.sin() * rx) + (longitude.cos() * ry);

    let top_z = (latitude.cos() * longitude.cos() * rx)
        + (latitude.cos() * longitude.sin() * ry)
        + latitude * rz;

    Topocentric {
        top_s,
        top_e,
        top_z,
    }
}

pub fn topocentric_to_look_angles(tc: &Topocentric) -> LookAngles {
    let top_s = tc.top_s;
    let top_e = tc.top_e;
    let top_z = tc.top_z;

    let range_sat = (top_s * top_s + top_e * top_e + top_z * top_z).sqrt();
    let el = (top_z / range_sat).asin();
    let az = (-top_e).atan2(top_s) + PI;

    LookAngles {
        azimuth: az,
        elevation: el,
        range_sat,
    }
}

pub fn ecf_to_look_angles(observer_geodetic: &Position, satellite_ecf: &Vector3) -> LookAngles {
    let topocentric_coords = topocentric(observer_geodetic, satellite_ecf);
    topocentric_to_look_angles(&topocentric_coords)
}
