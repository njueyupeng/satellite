
use wasm_bindgen::prelude::*;
use crate::constants::MINUTES_PER_DAY;
use serde_wasm_bindgen::to_value;
use crate::ext::{jday, jday_date};
use crate::propagation::sgp4::{sgp4, Sgp4Error, Sgp4Result};
use crate::SatRec;

use chrono::prelude::*;
use chrono::DateTime;


pub fn propagate(
    satrec: &mut SatRec,
    year: f64,
    mon: f64,
    day: f64,
    hour: f64,
    minute: f64,
    sec: f64,
    msec: f64,
) -> Result<Sgp4Result, Sgp4Error> {
    let j = jday(year, mon, day, hour, minute, sec, msec);
    let m = (j - satrec.jdsatepoch) * MINUTES_PER_DAY;
    sgp4(satrec, m)
}

#[wasm_bindgen(js_name="propagate")]
pub fn js_propagate( 
    satrec: &mut SatRec,
    year: f64,
    mon: f64,
    day: f64,
    hour: f64,
    minute: f64,
    sec: f64,
    msec: f64,)->Result<JsValue, JsValue>{
    let result =  propagate(satrec, year, mon, day, hour, minute, sec, msec);
    match result {
        Ok(sgp4_result) => Ok(to_value(&sgp4_result).unwrap()),
        Err(sgp4_error) => Err(to_value(&sgp4_error).unwrap()),
    }
}

pub fn propagate_date(satrec: &mut SatRec, date: &DateTime<Utc>) -> Result<Sgp4Result, Sgp4Error> {
    let j = jday_date(*date);
    let m = (j - satrec.jdsatepoch) * MINUTES_PER_DAY;
    sgp4(satrec, m)
}
