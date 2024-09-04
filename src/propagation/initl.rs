use crate::constants::{J2, TWO_PI, X2O3, XKE};
use crate::propagation::gstime::gstime;
use crate::DpperOpsMode;

/*-----------------------------------------------------------------------------
*
*                           procedure initl
*
*  this procedure initializes the sgp4 propagator. all the initialization is
*    consolidated here instead of having multiple loops inside other routines.
*
*  author        : david vallado                  719-573-2600   28 jun 2005
*
*  inputs        :
*    ecco        - eccentricity                           0.0 - 1.0
*    epoch       - epoch time in days from jan 0, 1950. 0 hour
*    inclo       - inclination of satellite
*    no          - mean motion of satellite
*    satn        - satellite number
*
*  outputs       :
*    ainv        - 1.0 / a
*    ao          - semi major axis
*    con41       -
*    con42       - 1.0 - 5.0 cos(i)
*    cosio       - cosine of inclination
*    cosio2      - cosio squared
*    eccsq       - eccentricity squared
*    method      - flag for deep space                    'd', 'n'
*    omeosq      - 1.0 - ecco * ecco
*    posq        - semi-parameter squared
*    rp          - radius of perigee
*    rteosq      - square root of (1.0 - ecco*ecco)
*    sinio       - sine of inclination
*    gsto        - gst at time of observation               rad
*    no          - mean motion of satellite
*
*  locals        :
*    ak          -
*    d1          -
*    del         -
*    adel        -
*    po          -
*
*  coupling      :
*    getgravconst
*    gstime      - find greenwich sidereal time from the julian date
*
*  references    :
*    hoots, roehrich, norad spacetrack report #3 1980
*    hoots, norad spacetrack report #6 1986
*    hoots, schumacher and glover 2004
*    vallado, crawford, hujsak, kelso  2006
----------------------------------------------------------------------------*/

pub struct InitOptions {
    pub ecco: f64,
    pub epoch: f64,
    pub inclo: f64,
    pub opsmode: DpperOpsMode,
    pub no: f64,
}

#[derive(PartialEq, Debug)]
pub enum InitlMethod {
    N,
    D,
}

pub struct InitlResult {
    pub no: f64,

    pub method: InitlMethod,

    pub ainv: f64,
    pub ao: f64,
    pub con41: f64,
    pub con42: f64,
    pub cosio: f64,

    pub cosio2: f64,
    pub eccsq: f64,
    pub omeosq: f64,
    pub posq: f64,

    pub rp: f64,
    pub rteosq: f64,
    pub sinio: f64,
    pub gsto: f64,
}
pub fn initl(options: InitOptions) -> InitlResult {
    let ecco = options.ecco;
    let epoch = options.epoch;
    let inclo = options.inclo;
    let opsmode = options.opsmode;
    let mut no = options.no;

    // sgp4fix use old way of finding gst
    // ----------------------- earth constants ---------------------
    // sgp4fix identify constants and allow alternate values

    // ------------- calculate auxillary epoch quantities ----------
    let eccsq = ecco * ecco;
    let omeosq = 1.0 - eccsq;
    let rteosq = (omeosq).sqrt();
    let cosio = (inclo).cos();
    let cosio2 = cosio * cosio;

    // ------------------ un-kozai the mean motion -----------------
    let ak = (XKE / no).powf(X2O3);
    let d1 = (0.75 * J2 * ((3.0 * cosio2) - 1.0)) / (rteosq * omeosq);
    let mut del_prime = d1 / (ak * ak);
    let adel = ak
        * (1.0
            - (del_prime * del_prime)
            - (del_prime * ((1.0 / 3.0) + ((134.0 * del_prime * del_prime) / 81.0))));
    del_prime = d1 / (adel * adel);
    no /= 1.0 + del_prime;

    let ao = (XKE / no).powf(X2O3);
    let sinio = (inclo).sin();
    let po = ao * omeosq;
    let con42 = 1.0 - (5.0 * cosio2);
    let con41 = -con42 - cosio2 - cosio2;
    let ainv = 1.0 / ao;
    let posq = po * po;
    let rp = ao * (1.0 - ecco);
    let method = InitlMethod::N;

    //  sgp4fix modern approach to finding sidereal time
    let mut gsto;
    if opsmode == DpperOpsMode::A {
        //  sgp4fix use old way of finding gst
        //  count integer number of days from 0 jan 1970
        let ts70 = epoch - 7305.0;
        let ds70 = (ts70 + 1.0e-8).floor();
        let tfrac = ts70 - ds70;

        //  find greenwich location at epoch
        let c1 = 1.72027916940703639e-2;
        let thgr70 = 1.7321343856509374;
        let fk5r = 5.07551419432269442e-15;
        let c1p2p = c1 + TWO_PI;
        gsto = (thgr70 + (c1 * ds70) + (c1p2p * tfrac) + (ts70 * ts70 * fk5r)) % TWO_PI;
        if gsto < 0.0 {
            gsto += TWO_PI;
        }
    } else {
        gsto = gstime(epoch + 2433281.5);
    }

    InitlResult {
        no,

        method,

        ainv,
        ao,
        con41,
        con42,
        cosio,

        cosio2,
        eccsq,
        omeosq,
        posq,

        rp,
        rteosq,
        sinio,
        gsto,
    }
}


#[cfg(test)]
mod test{
    use super::{
        initl,
        InitOptions,
        DpperOpsMode,
        InitlMethod
    };
    fn is_close(actual: f64, ed: f64, epsilon: f64) -> bool {
        (actual - ed).abs() < epsilon
    }
    
    #[test]
    pub fn legacy_sidereal_time_calculations() {
        const OPTIONS: InitOptions = InitOptions {
            ecco: 0.1846988,
            epoch: 25938.538312919904,
            inclo: 0.0,
            no: 0.0037028783237264057,
            opsmode: DpperOpsMode::A,
        };
        let results = initl(OPTIONS);
        let epsilon = 1e-3;
    
        assert!(is_close(results.ainv, 0.1353414893496189, epsilon));
        assert!(is_close(results.ao, 7.3887172721793, epsilon));
        assert_eq!(results.con41, 2.0);
        assert_eq!(results.con42, -4.0);
        assert_eq!(results.cosio, 1.0);
        assert_eq!(results.cosio2, 1.0);
        assert!(is_close(results.eccsq, 0.034113646721439995, epsilon));
        assert!(is_close(results.gsto, 5.220883431398299, epsilon));
        assert_eq!(results.method, InitlMethod::N);
        assert!(is_close(results.no, 0.003702762286531528, epsilon));
        assert!(is_close(results.omeosq, 0.96588635327856, epsilon));
        assert!(is_close(results.posq, 50.931932818552305, epsilon));
        assert!(is_close(results.rp, 6.02403005846851, epsilon));
        assert!(is_close(results.rteosq, 0.9827951736137902, epsilon));
        assert_eq!(results.sinio, 0.0);
    }
    
}