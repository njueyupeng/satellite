use crate::constants::MINUTES_PER_DAY;
use crate::ext::jday_date;
use crate::propagation::sgp4::{sgp4, Sgp4Error, Sgp4Result};
use crate::types::SatRec;

use chrono::prelude::*;
use chrono::DateTime;

pub fn propagate(satrec: &mut SatRec, date: &DateTime<Utc>) -> Result<Sgp4Result, Sgp4Error> {
    let j = jday_date(*date);
    let m = (j - satrec.jdsatepoch) * MINUTES_PER_DAY;
    sgp4(satrec, m)
}
