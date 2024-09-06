use core::str;

use crate::constants::{DEG2RAD, PI};
use crate::ext::{days2mdhms, jday};
use crate::propagation::sgp4init::{sgp4init, Sgp4InitOptions};
use crate::{DpperOpsMode, SatRec};

fn parse_float(str: &str) -> f64 {
    return str.parse::<f64>().unwrap();
}
fn parse_int(str: &str) -> i32 {
    return str.parse::<i32>().unwrap_or(0);
}

/* -----------------------------------------------------------------------------
*
*                           function twoline2rv
*
*  this function converts the two line element set character string data to
*    variables and initializes the sgp4 variables. several intermediate varaibles
*    and quantities are determined. note that the result is a structure so multiple
*    satellites can be processed simultaneously without having to reinitialize. the
*    verification mode is an important option that permits quick checks of any
*    changes to the underlying technical theory. this option works using a
*    modified tle file in which the start, stop, and delta time values are
*    included at the end of the second line of data. this only works with the
*    verification mode. the catalog mode simply propagates from -1440 to 1440 min
*    from epoch and is useful when performing entire catalog runs.
*
*  author        : david vallado                  719-573-2600    1 mar 2001
*
*  inputs        :
*    longstr1    - first line of the tle
*    longstr2    - second line of the tle
*    typerun     - type of run                    verification 'v', catalog 'c',
*                                                 manual 'm'
*    typeinput   - type of manual input           mfe 'm', epoch 'e', dayofyr 'd'
*    opsmode     - mode of operation afspc or improved 'a', 'i'
*    whichconst  - which set of constants to use  72, 84
*
*  outputs       :
*    satrec      - structure containing all the sgp4 satellite information
*
*  coupling      :
*    getgravconst-
*    days2mdhms  - conversion of days to month, day, hour, minute, second
*    jday        - convert day month year hour minute second into julian date
*    sgp4init    - initialize the sgp4 variables
*
*  references    :
*    norad spacetrack report #3
*    vallado, crawford, hujsak, kelso  2006
--------------------------------------------------------------------------- */

/**
 * Return a Satellite imported from two lines of TLE data.
 *
 * Provide the two TLE lines as strings `longstr1` and `longstr2`,
 * and select which standard set of gravitational constants you want
 * by providing `gravity_constants`:
 *
 * `sgp4.propagation.wgs72` - Standard WGS 72 model
 * `sgp4.propagation.wgs84` - More recent WGS 84 model
 * `sgp4.propagation.wgs72old` - Legacy support for old SGP4 behavior
 *
 * Normally, computations are made using letious recent improvements
 * to the algorithm.  If you want to turn some of these off and go
 * back into "afspc" mode, then set `afspc_mode` to `True`.
 */
pub fn twoline2satrec(longstr1: &str, longstr2: &str) -> SatRec {
    let opsmode = DpperOpsMode::I;
    let xpdotp = 1440.0 / (2.0 * PI); // 229.1831180523293;
    let  year;

    let mut satrec = SatRec::new();
    satrec.error = 0;

    satrec.satnum = String::from(longstr1[2..7].trim());
    satrec.epochyr = parse_int(longstr1[18..20].trim()) as u32;
    satrec.epochdays = parse_float(longstr1[20..32].trim());
    satrec.ndot = parse_float(longstr1[33..43].trim());
    let temp = parse_int(longstr1[44..50].trim());
    let temp2 = parse_float(longstr1[50..52].trim());
    let nddot_str = format!("{}.{}E{}",0,&temp.to_string(),&temp2.to_string());
    satrec.nddot =
        parse_float(&nddot_str);
    let bs_temp1 = parse_int(longstr1[53..54].trim());
    let bs_temp2 = parse_int(longstr1[54..59].trim());
    let bs_temp3 = parse_int(longstr1[59..61].trim());
    let bstar_str = format!("{}.{}E{}",bs_temp1,bs_temp2,bs_temp3);
    satrec.bstar = parse_float(&bstar_str);

    satrec.inclo = parse_float(longstr2[8..16].trim());
    satrec.nodeo = parse_float(longstr2[17..25].trim());
    let ecco_str = format!("0.{}",longstr2[26..33].trim());
    satrec.ecco = parse_float(&ecco_str);
    satrec.argpo = parse_float(longstr2[34..42].trim());
    satrec.mo = parse_float(longstr2[43..51].trim());
    satrec.no = parse_float(longstr2[52..63].trim());

    // ---- find no, ndot, nddot ----
    satrec.no /= xpdotp; //   rad/min
                         // satrec.nddot= satrec.nddot * Math.pow(10.0, nexp);
                         // satrec.bstar= satrec.bstar * Math.pow(10.0, ibexp);

    // ---- convert to sgp4 units ----
    // satrec.ndot /= (xpdotp * 1440.0); // ? * minperday
    // satrec.nddot /= (xpdotp * 1440.0 * 1440);

    // ---- find standard orbital elements ----
    satrec.inclo *= DEG2RAD;
    satrec.nodeo *= DEG2RAD;
    satrec.argpo *= DEG2RAD;
    satrec.mo *= DEG2RAD;

    // ----------------------------------------------------------------
    // find sgp4epoch time of element set
    // remember that sgp4 uses units of days from 0 jan 1950 (sgp4epoch)
    // and minutes from the epoch (time)
    // ----------------------------------------------------------------

    // ---------------- temp fix for years from 1957-2056 -------------------
    // --------- correct fix will occur when year is 4-digit in tle ---------

    if satrec.epochyr < 57 {
        year = satrec.epochyr + 2000;
    } else {
        year = satrec.epochyr + 1900;
    }

    let mdhms_result = days2mdhms(year, satrec.epochdays);
    let mon = mdhms_result.month;
    let day = mdhms_result.day;
    let hour = mdhms_result.hour;
    let minute = mdhms_result.minute;
    let sec = mdhms_result.second;

    satrec.jdsatepoch = jday(
        year as f64,
        mon as f64,
        day as f64,
        hour as f64,
        minute as f64,
        sec as f64,
        0.0,
    );

    //  ---------------- initialize the orbit at sgp4epoch -------------------
    let satn = (satrec.satnum).parse::<f64>().unwrap();
    let epoch = satrec.jdsatepoch - 2433281.5;
    let xbstar = satrec.bstar;
    let xecco = satrec.ecco;
    let xargpo = satrec.argpo;
    let xinclo = satrec.inclo;
    let xmo = satrec.mo;
    let xno = satrec.no;
    let xnodeo = satrec.nodeo;
    sgp4init(
        &mut satrec,
        Sgp4InitOptions {
            opsmode,
            satn: satn,
            epoch,
            xbstar,
            xecco,
            xargpo,
            xinclo,
            xmo,
            xno,
            xnodeo,
        },
    );

    satrec
}
