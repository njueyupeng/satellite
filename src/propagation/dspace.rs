use crate::constants::TWO_PI;

pub struct DspaceOption {
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
    pub argpo: f64,
    pub argpdot: f64,
    pub t: f64,
    pub tc: f64,
    pub gsto: f64,
    pub xfact: f64,
    pub xlamo: f64,
    pub no: f64,
    pub atime: f64,
    pub em: f64,
    pub argpm: f64,
    pub inclm: f64,
    pub xli: f64,
    pub mm: f64,
    pub xni: f64,
    pub nodem: f64,
    pub nm: f64,
}

#[allow(dead_code)]
pub struct DspaceResult {
    pub atime: f64,
    pub em: f64,
    pub argpm: f64,
    pub inclm: f64,
    pub xli: f64,
    pub mm: f64,
    pub xni: f64,
    pub nodem: f64,
    pub dndt: f64,
    pub nm: f64,
}

/*-----------------------------------------------------------------------------
*
*                           procedure dspace
*
*  this procedure provides deep space contributions to mean elements for
*    perturbing third body.  these effects have been averaged over one
*    revolution of the sun and moon.  for earth resonance effects, the
*    effects have been averaged over no revolutions of the satellite.
*    (mean motion)
*
*  author        : david vallado                  719-573-2600   28 jun 2005
*
*  inputs        :
*    d2201, d2211, d3210, d3222, d4410, d4422, d5220, d5232, d5421, d5433 -
*    dedt        -
*    del1, del2, del3  -
*    didt        -
*    dmdt        -
*    dnodt       -
*    domdt       -
*    irez        - flag for resonance           0-none, 1-one day, 2-half day
*    argpo       - argument of perigee
*    argpdot     - argument of perigee dot (rate)
*    t           - time
*    tc          -
*    gsto        - gst
*    xfact       -
*    xlamo       -
*    no          - mean motion
*    atime       -
*    em          - eccentricity
*    ft          -
*    argpm       - argument of perigee
*    inclm       - inclination
*    xli         -
*    mm          - mean anomaly
*    xni         - mean motion
*    nodem       - right ascension of ascending node
*
*  outputs       :
*    atime       -
*    em          - eccentricity
*    argpm       - argument of perigee
*    inclm       - inclination
*    xli         -
*    mm          - mean anomaly
*    xni         -
*    nodem       - right ascension of ascending node
*    dndt        -
*    nm          - mean motion
*
*  locals        :
*    delt        -
*    ft          -
*    theta       -
*    x2li        -
*    x2omi       -
*    xl          -
*    xldot       -
*    xnddt       -
*    xndt        -
*    xomi        -
*
*  coupling      :
*    none        -
*
*  references    :
*    hoots, roehrich, norad spacetrack report #3 1980
*    hoots, norad spacetrack report #6 1986
*    hoots, schumacher and glover 2004
*    vallado, crawford, hujsak, kelso  2006
----------------------------------------------------------------------------*/
pub fn dspace(options: DspaceOption) -> DspaceResult {
    let irez = options.irez;
    let d2201 = options.d2201;
    let d2211 = options.d2211;
    let d3210 = options.d3210;
    let d3222 = options.d3222;
    let d4410 = options.d4410;
    let d4422 = options.d4422;
    let d5220 = options.d5220;
    let d5232 = options.d5232;
    let d5421 = options.d5421;
    let d5433 = options.d5433;
    let dedt = options.dedt;
    let del1 = options.del1;
    let del2 = options.del2;
    let del3 = options.del3;
    let didt = options.didt;
    let dmdt = options.dmdt;
    let dnodt = options.dnodt;
    let domdt = options.domdt;
    let argpo = options.argpo;
    let argpdot = options.argpdot;
    let t = options.t;
    let tc = options.tc;
    let gsto = options.gsto;
    let xfact = options.xfact;
    let xlamo = options.xlamo;
    let no = options.no;

    let mut atime = options.atime;
    let mut em = options.em;
    let mut argpm = options.argpm;
    let mut inclm = options.inclm;
    let mut xli = options.xli;
    let mut mm = options.mm;
    let mut xni = options.xni;
    let mut nodem = options.nodem;
    let mut nm = options.nm;

    const FASX2: f64 = 0.13130908;
    const FASX4: f64 = 2.8843198;
    const FASX6: f64 = 0.37448087;
    const G22: f64 = 5.7686396;
    const G32: f64 = 0.95240898;
    const G44: f64 = 1.8014998;
    const G52: f64 = 1.0508330;
    const G54: f64 = 4.4108898;
    // eslint-disable-next-line no-loss-of-precision
    const RPTIM: f64 = 4.37526908801129966e-3; // equates to 7.29211514668855e-5 rad/sec
    const STEPP: f64 = 720.0;
    const STEPN: f64 = -720.0;
    const STEP2: f64 = 259200.0;

    let delt;
    let mut x2li;
    let mut x2omi;
    let xl;
    let mut xldot = 0.0;
    let mut xnddt = 0.0;
    let mut xndt = 0.0;
    let mut xomi ;
    let mut dndt = 0.0;
    let mut ft = 0.0;

    //  ----------- calculate deep space resonance effects -----------
    let theta = (gsto + (tc * RPTIM)) % TWO_PI;
    em += dedt * t;

    inclm += didt * t;
    argpm += domdt * t;
    nodem += dnodt * t;
    mm += dmdt * t;

    // sgp4fix for negative inclinations
    // the following if statement should be commented out
    // if (inclm < 0.0)
    // {
    //   inclm = -inclm;
    //   argpm = argpm - pi;
    //   nodem = nodem + pi;
    // }

    /* - update resonances : numerical (euler-maclaurin) integration - */
    /* ------------------------- epoch restart ----------------------  */
    //   sgp4fix for propagator problems
    //   the following integration works for negative time steps and periods
    //   the specific changes are unknown because the original code was so convoluted

    // sgp4fix take out atime = 0.0 and fix for faster operation

    if irez != 0 {
        //  sgp4fix streamline check
        if atime == 0.0 || t * atime <= 0.0 || t.abs() < atime.abs() {
            atime = 0.0;
            xni = no;
            xli = xlamo;
        }

        // sgp4fix move check outside loop
        if t > 0.0 {
            delt = STEPP;
        } else {
            delt = STEPN;
        }

        let mut iretn = 381; // added for do loop
        while iretn == 381 {
            //  ------------------- dot terms calculated -------------
            //  ----------- near - synchronous resonance terms -------
            if irez != 2 {
                xndt = (del1 * (xli - FASX2).sin())
                    + (del2 * (2.0 * (xli - FASX4).sin()))
                    + (del3 * (3.0 * (xli - FASX6)).sin());
                xldot = xni + xfact;
                xnddt = (del1 * (xli - FASX2).cos())
                    + (2.0 * del2 * (2.0 * (xli - FASX4).cos()))
                    + (3.0 * del3 * (3.0 * (xli - FASX6)).cos());
                xnddt *= xldot;
            } else {
                // --------- near - half-day resonance terms --------
                xomi = argpo + (argpdot * atime);
                x2omi = xomi + xomi;
                x2li = xli + xli;
                xndt = (d2201 * ((x2omi + xli) - G22).sin())
                    + (d2211 * (xli - G22).sin())
                    + (d3210 * ((xomi + xli) - G32).sin())
                    + (d3222 * ((-xomi + xli) - G32).sin())
                    + (d4410 * ((x2omi + x2li) - G44).sin())
                    + (d4422 * (x2li - G44).sin())
                    + (d5220 * ((xomi + xli) - G52).sin())
                    + (d5232 * ((-xomi + xli) - G52).sin())
                    + (d5421 * ((xomi + x2li) - G54).sin())
                    + (d5433 * ((-xomi + x2li) - G54).sin());
                xldot = xni + xfact;
                xnddt = (d2201 * ((x2omi + xli) - G22).cos())
                    + (d2211 * (xli - G22).cos())
                    + (d3210 * ((xomi + xli) - G32).cos())
                    + (d3222 * ((-xomi + xli) - G32).cos())
                    + (d5220 * ((xomi + xli) - G52).cos())
                    + (d5232 * ((-xomi + xli) - G52).cos())
                    + 2.0
                        * ((d4410 * ((x2omi + x2li) - G44).cos())
                            + (d4422 * (x2li - G44).cos())
                            + (d5421 * ((xomi + x2li) - G54).cos())
                            + (d5433 * ((-xomi + x2li) - G54).cos()));
                xnddt *= xldot;
            }

            //  ----------------------- integrator -------------------
            //  sgp4fix move end checks to end of routine
            if (t - atime).abs() >= STEPP {
                iretn = 381;
            } else {
                ft = t - atime;
                iretn = 0;
            }

            if iretn == 381 {
                xli += (xldot * delt) + (xndt * STEP2);
                xni += (xndt * delt) + (xnddt * STEP2);
                atime += delt;
            }
        }

        nm = xni + (xndt * ft) + (xnddt * ft * ft * 0.5);
        xl = xli + (xldot * ft) + (xndt * ft * ft * 0.5);
        if irez != 1 {
            mm = (xl - (2.0 * nodem)) + (2.0 * theta);
            dndt = nm - no;
        } else {
            mm = (xl - nodem - argpm) + theta;
            dndt = nm - no;
        }
        nm = no + dndt;
    }

    DspaceResult {
        atime,
        em,
        argpm,
        inclm,
        xli,
        mm,
        xni,
        nodem,
        dndt,
        nm,
    }
}
