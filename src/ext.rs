use chrono::prelude::*;
use chrono::DateTime;

pub struct CustomDate {
    pub month: u64,
    pub day: u64,
    pub hour: u64,
    pub minute: u64,
    pub second: u64,
}

/* -----------------------------------------------------------------------------
 *
 *                           procedure days2mdhms
 *
 *  this procedure converts the day of the year, days, to the equivalent month
 *    day, hour, minute and second.
 *
 *  algorithm     : set up array for the number of days per month
 *                  find leap year - use 1900 because 2000 is a leap year
 *                  loop through a temp value while the value is < the days
 *                  perform int conversions to the correct day and month
 *                  convert remainder into h m s using type conversions
 *
 *  author        : david vallado                  719-573-2600    1 mar 2001
 *
 *  inputs          description                    range / units
 *    year        - year                           1900 .. 2100
 *    days        - julian day of the year         0.0  .. 366.0
 *
 *  outputs       :
 *    mon         - month                          1 .. 12
 *    day         - day                            1 .. 28,29,30,31
 *    hr          - hour                           0 .. 23
 *    min         - minute                         0 .. 59
 *    sec         - second                         0.0 .. 59.999
 *
 *  locals        :
 *    dayofyr     - day of year
 *    temp        - temporary extended values
 *    inttemp     - temporary int value
 *    i           - index
 *    lmonth[12]  - int array containing the number of days per month
 *
 *  coupling      :
 *    none.
 * --------------------------------------------------------------------------- */
pub fn days2mdhms(year: u32, days: f64) -> CustomDate {
    let lmonth = [
        31,
        if (year % 4) == 0 { 29 } else { 28 },
        31,
        30,
        31,
        30,
        31,
        31,
        30,
        31,
        30,
        31,
    ];

    let dayofyr = days.floor() as f64;

    //  ----------------- find month and day of month ----------------
    let mut i = 1;
    let mut inttemp = 0;
    while (dayofyr > (inttemp as f64 + lmonth[i - 1] as f64)) && i < 12 {
        inttemp += lmonth[i - 1];
        i += 1;
    }

    let month: u64 = i as u64;
    let day = dayofyr as u64 - inttemp as u64;

    //  ----------------- find hours minutes and seconds -------------
    let mut temp = (days - dayofyr) * 24.0;
    let hour = (temp).floor() as u64;
    temp = (temp - hour as f64) * 60.0;
    let minute = (temp).floor() as u64;
    let second = ((temp - minute as f64) * 60.0) as u64;

    CustomDate {
        month,
        day,
        hour,
        minute,
        second,
    }
}

/* -----------------------------------------------------------------------------
 *
 *                           procedure jday
 *
 *  this procedure finds the julian date given the year, month, day, and time.
 *    the julian date is defined by each elapsed day since noon, jan 1, 4713 bc.
 *
 *  algorithm     : calculate the answer in one step for efficiency
 *
 *  author        : david vallado                  719-573-2600    1 mar 2001
 *
 *  inputs          description                    range / units
 *    year        - year                           1900 .. 2100
 *    mon         - month                          1 .. 12
 *    day         - day                            1 .. 28,29,30,31
 *    hr          - universal time hour            0 .. 23
 *    min         - universal time min             0 .. 59
 *    sec         - universal time sec             0.0 .. 59.999
 *
 *  outputs       :
 *    jd          - julian date                    days from 4713 bc
 *
 *  locals        :
 *    none.
 *
 *  coupling      :
 *    none.
 *
 *  references    :
 *    vallado       2007, 189, alg 14, ex 3-14
 *
 * --------------------------------------------------------------------------- */
pub fn jday_internal(
    year: f64,
    mon: f64,
    day: f64,
    hr: f64,
    minute: f64,
    sec: f64,
    msec: f64,
) -> f64 {
    ((367.0 * year) - ((7.0 * (year + ((mon + 9.0) / 12.0).floor())) * 0.25).floor())
        + ((275.0 * mon) / 9.0).floor()
        + day
        + 1721013.5
        + (((((msec / 60000.0) + (sec / 60.0) + minute) / 60.0) + hr) / 24.0)
}

pub fn jday(year: f64, mon: f64, day: f64, hr: f64, minute: f64, sec: f64, msec: f64) -> f64 {
    // todo
    jday_internal(year, mon, day, hr, minute, sec, msec)
}

pub fn jday_date(datetime: DateTime<Utc>) -> f64 {
    jday(
        datetime.year() as f64,
        datetime.month() as f64,
        datetime.day() as f64,
        datetime.hour() as f64,
        datetime.minute() as f64,
        datetime.second() as f64,
        0 as f64,
    )
}

pub fn invjday(jd: f64, as_array: bool) {
    // --------------- find year and days of the year -
    let temp = jd - 2415019.5;
    let tu = temp / 365.25;
    let mut year = 1900.0 + (tu).floor();
    let mut leapyrs = ((year - 1901.0) * 0.25).floor();

    // optional nudge by 8.64x10-7 sec to get even outputs
    let mut days = (temp - (((year - 1900.0) * 365.0) + leapyrs)) + 0.00000000001;

    // ------------ check for case of beginning of a year -----------
    if days < 1.0 {
        year -= 1.0;
        leapyrs = ((year - 1901.0) * 0.25).floor();
        days = temp - (((year - 1900.0) * 365.0) + leapyrs);
    }

    // ----------------- find remaing data  -------------------------
    let mdhms = days2mdhms(year as u32, days as f64);

    let mon = mdhms.month;
    let day = mdhms.day;
    let hr = mdhms.hour;
    let minute = mdhms.minute;

    // todo
    // let sec = mdhms.second - 0.00000086400;

    // todo
    // if (as_array) {
    //     return [year, mon, day, hr, minute, Math.floor(sec)];
    // }

    // return new Date(Date.UTC(year, mon - 1, day, hr, minute, Math.floor(sec)));
}
