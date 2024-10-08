//! # Satellite
//!
//! Modular set of functions for SGP4 and SDP4 propagation of TLEs.
extern crate wasm_bindgen;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
pub mod constants;
use serde::{Deserialize, Serialize};

mod doppler_factor;
mod ext;
mod io;
mod propagation;
mod transforms;
pub use ext::{jday, jday_date};
pub use io::twoline2satrec;
pub use propagation::{
    gstime::gstime,
    propagate::{propagate, propagate_date},
    sgp4::{sgp4, Sgp4Error, Sgp4Result},
};

pub use doppler_factor::doppler_factor;

pub use transforms::{
    degrees_lat, degrees_long, degrees_to_radians, ecf_to_eci, ecf_to_look_angles, eci_to_ecf,
    eci_to_geodetic, geodetic_to_ecf, radians_lat, radians_long, radians_to_degrees,
};
#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct EciVec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
#[wasm_bindgen]
pub struct EcfVec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
#[wasm_bindgen]
pub struct GeodeticLocation {
    pub longitude: f64,
    pub latitude: f64,
    pub height: f64,
}

#[wasm_bindgen]
pub struct Topocentric {
    top_s: f64,
    top_e: f64,
    top_z: f64,
}
#[allow(dead_code)]
#[wasm_bindgen]
pub struct LookAngles {
    azimuth: f64,
    elevation: f64,
    range_sat: f64,
}

/// Satellite record containing description of orbit.

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct SatRec {
    // near earth variables
    pub isimp: u32,
    pub method: char,
    pub aycof: f64,
    pub con41: f64,
    pub cc1: f64,
    pub cc4: f64,
    pub cc5: f64,
    pub d2: f64,
    pub d3: f64,
    pub d4: f64,
    pub delmo: f64,
    pub eta: f64,
    pub argpdot: f64,
    pub omgcof: f64,
    pub sinmao: f64,
    /// Full four-digit year of this element set's epoch moment.
    pub epochyr: u32,
    pub t: f64,
    pub t2cof: f64,
    pub t3cof: f64,
    pub t4cof: f64,
    pub t5cof: f64,
    pub x1mth2: f64,
    pub x7thm1: f64,
    pub mdot: f64,
    /// Second time derivative of the mean motion (ignored by SGP4).
    pub nddot: f64,
    /// First time derivative of the mean motion (ignored by SGP4).
    pub ndot: f64,
    pub nodedot: f64,
    pub xlcof: f64,
    pub xmcof: f64,
    pub nodecf: f64,
    /// Julian date of the epoch (computed from epochyr and epochdays).
    pub jdsatepoch: f64,

    // deep space variables
    pub irez: u32,
    pub d2201: f64,
    pub d2211: f64,
    pub d3210: f64,
    pub d3222: f64,
    pub d4410: f64,
    pub d4422: f64,
    pub d5220: f64,
    pub d5232: f64,
    pub d5421: f64,
    pub d5433: f64,
    pub dedt: f64,
    pub del1: f64,
    pub del2: f64,
    pub del3: f64,
    pub didt: f64,
    pub dmdt: f64,
    pub dnodt: f64,
    pub domdt: f64,
    /// Unique satellite number given in the TLE file.
    satnum: String,
    pub e3: f64,
    pub ee2: f64,
    pub peo: f64,
    pub pgho: f64,
    pub pho: f64,
    pub pinco: f64,
    pub plo: f64,
    pub se2: f64,
    pub se3: f64,
    pub sgh2: f64,
    pub sgh3: f64,
    pub sgh4: f64,
    pub sh2: f64,
    pub sh3: f64,
    pub si2: f64,
    pub si3: f64,
    pub sl2: f64,
    pub sl3: f64,
    pub sl4: f64,
    pub gsto: f64,
    pub xfact: f64,
    pub xgh2: f64,
    pub xgh3: f64,
    pub xgh4: f64,
    pub xh3: f64,
    pub xi2: f64,
    pub xh2: f64,
    pub xi3: f64,
    pub xl2: f64,
    pub xlamo: f64,
    pub xl3: f64,
    pub xl4: f64,
    pub zmol: f64,
    pub zmos: f64,
    pub xni: f64,
    pub atime: f64,
    pub xli: f64,
    /// Fractional days into the year of the epoch moment.
    pub epochdays: f64,
    /// Ballistic drag coefficient B* in inverse earth radii.
    pub bstar: f64,
    /// Eccentricity
    pub ecco: f64,
    /// Argument of perigee in radians.
    pub argpo: f64,
    /// Inclination in radians.
    pub inclo: f64,
    /// Mean anomaly in radians.
    pub mo: f64,
    /// Mean motion in radians per minute.
    pub no: f64,
    /// Right ascension of ascending node in radians.
    pub nodeo: f64,
    operationmode: DpperOpsMode,
    init: DpperInit,

    pub a: f64,
    pub alta: f64,
    pub altp: f64,
    /// Error code indicating propagation failure type.
    pub error: u32,
}

#[wasm_bindgen]
impl SatRec {
    pub fn new() -> SatRec {
        SatRec {
            isimp: 0,
            method: 'n',
            aycof: 0.0,
            con41: 0.0,
            cc1: 0.0,
            cc4: 0.0,
            cc5: 0.0,
            d2: 0.0,
            d3: 0.0,
            d4: 0.0,
            delmo: 0.0,
            eta: 0.0,
            argpdot: 0.0,
            omgcof: 0.0,
            sinmao: 0.0,
            t: 0.0,
            t2cof: 0.0,
            t3cof: 0.0,
            t4cof: 0.0,
            t5cof: 0.0,
            x1mth2: 0.0,
            x7thm1: 0.0,
            mdot: 0.0,
            nddot: 0.0,
            ndot: 0.0,
            nodedot: 0.0,
            xlcof: 0.0,
            xmcof: 0.0,
            nodecf: 0.0,
            irez: 0,
            d2201: 0.0,
            d2211: 0.0,
            d3210: 0.0,
            d3222: 0.0,
            d4410: 0.0,
            d4422: 0.0,
            d5220: 0.0,
            d5232: 0.0,
            d5421: 0.0,
            d5433: 0.0,
            dedt: 0.0,
            del1: 0.0,
            del2: 0.0,
            del3: 0.0,
            didt: 0.0,
            dmdt: 0.0,
            dnodt: 0.0,
            domdt: 0.0,
            jdsatepoch: 0.0,
            e3: 0.0,
            ee2: 0.0,
            peo: 0.0,
            pgho: 0.0,
            pho: 0.0,
            pinco: 0.0,
            epochyr: 0,
            satnum: String::new(),
            epochdays: 0.0,
            plo: 0.0,
            se2: 0.0,
            se3: 0.0,
            sgh2: 0.0,
            sgh3: 0.0,
            sgh4: 0.0,
            sh2: 0.0,
            sh3: 0.0,
            si2: 0.0,
            si3: 0.0,
            sl2: 0.0,
            sl3: 0.0,
            sl4: 0.0,
            gsto: 0.0,
            xfact: 0.0,
            xgh2: 0.0,
            xgh3: 0.0,
            xgh4: 0.0,
            xh2: 0.0,
            xh3: 0.0,
            xi2: 0.0,
            xi3: 0.0,
            xl2: 0.0,
            xlamo: 0.0,
            xl3: 0.0,
            xl4: 0.0,
            zmol: 0.0,
            zmos: 0.0,
            xni: 0.0,
            atime: 0.0,
            xli: 0.0,
            bstar: 0.0,
            ecco: 0.0,
            argpo: 0.0,
            inclo: 0.0,
            mo: 0.0,
            no: 0.0,
            nodeo: 0.0,
            operationmode: DpperOpsMode::I.clone(),
            init: DpperInit::N.clone(),

            a: 0.0,
            alta: 0.0,
            altp: 0.0,
            error: 0,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn satnum(&self) -> String {
        self.satnum.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn operationmode(&self) -> char {
        match self.operationmode {
            DpperOpsMode::A => 'a',
            DpperOpsMode::I => 'i',
            _ => 'i',
        }
    }

    #[wasm_bindgen(getter)]
    pub fn init(&self) -> char {
        match self.init {
            DpperInit::Y => 'y',
            DpperInit::N => 'n',
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum DpperOpsMode {
    A,
    I,
    NONE,
}

#[derive(PartialEq, Clone, Debug)]
pub enum DpperInit {
    Y,
    N,
}
#[allow(dead_code)]
#[wasm_bindgen]
pub struct RangeErr {
    err: String,
}

//  wasm-bridger

// #[wasm_bindgen]
// pub fn twoline2satrec(line1:&str,line2:&str)->SatRec{
//   _twoline2satrec(line1,line2)
// }

#[wasm_bindgen]
pub fn get_constants() -> JsValue {
    let mut map = HashMap::new();

    map.insert("pi", constants::PI);
    map.insert("twoPi", constants::TWO_PI);
    map.insert("deg2rad", constants::DEG2RAD);
    map.insert("rad2deg", constants::RAD2DEG);
    map.insert("minutesPerDay", constants::MINUTES_PER_DAY);
    map.insert("mu", constants::MU);
    map.insert("earthRadius", constants::EARTH_RADIUS);
    map.insert("xke", constants::XKE);
    map.insert("vkmpersec", constants::VKMPERSEC);
    map.insert("tumin", constants::TUMIN);
    map.insert("j2", constants::J2);
    map.insert("j3", constants::J3);
    map.insert("j4", constants::J4);
    map.insert("j3oj2", constants::J3OJ2);
    map.insert("x2o3", constants::X2O3);

    // 将HashMap序列化为JSON字符串
    let json_str = serde_json::to_string(&map).unwrap();
    JsValue::from_str(&json_str)
}
