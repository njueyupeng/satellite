use chrono::prelude::*;
use chrono::DateTime;
use wasm_bindgen::prelude::*;

use crate::constants::{DEG2RAD, TWO_PI};
use crate::ext::jday_date;

/* -----------------------------------------------------------------------------
 *
 *                           function gstime
 *
 *  this function finds the greenwich sidereal time.
 *
 *  author        : david vallado                  719-573-2600    1 mar 2001
 *
 *  inputs          description                    range / units
 *    jdut1       - julian date in ut1             days from 4713 bc
 *
 *  outputs       :
 *    gstime      - greenwich sidereal time        0 to 2pi rad
 *
 *  locals        :
 *    temp        - temporary variable for doubles   rad
 *    tut1        - julian centuries from the
 *                  jan 1, 2000 12 h epoch (ut1)
 *
 *  coupling      :
 *    none
 *
 *  references    :
 *    vallado       2004, 191, eq 3-45
 * --------------------------------------------------------------------------- */

fn gstime_internal(jdut1: f64) -> f64 {
    let tut1 = (jdut1 - 2451545.0) / 36525.0;

    let mut temp = (-6.2e-6 * tut1 * tut1 * tut1)
        + (0.093104 * tut1 * tut1)
        + (((876600.0 * 3600.0) + 8640184.812866) * tut1)
        + 67310.54841; // # sec
    temp = ((temp * DEG2RAD) / 240.0) % TWO_PI; // 360/86400 = 1/240, to deg, to rad

    //  ------------------------ check quadrants ---------------------
    if temp < 0.0 {
        temp += TWO_PI;
    }

    temp
}
#[wasm_bindgen]
pub fn gstime(args: f64) -> f64 {
    gstime_internal(args)
}
#[allow(dead_code)]
pub fn gstime_date(datetime: DateTime<Utc>) -> f64 {
    let j_day = jday_date(datetime);
    gstime_internal(j_day)
}
