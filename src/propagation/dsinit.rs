use crate::constants::{PI, TWO_PI, X2O3, XKE};

/*-----------------------------------------------------------------------------
*
*                           procedure dsinit
*
*  this procedure provides deep space contributions to mean motion dot due
*    to geopotential resonance with half day and one day orbits.
*
*  author        : david vallado                  719-573-2600   28 jun 2005
*
*  inputs        :
*    cosim, sinim-
*    emsq        - eccentricity squared
*    argpo       - argument of perigee
*    s1, s2, s3, s4, s5      -
*    ss1, ss2, ss3, ss4, ss5 -
*    sz1, sz3, sz11, sz13, sz21, sz23, sz31, sz33 -
*    t           - time
*    tc          -
*    gsto        - greenwich sidereal time                   rad
*    mo          - mean anomaly
*    mdot        - mean anomaly dot (rate)
*    no          - mean motion
*    nodeo       - right ascension of ascending node
*    nodedot     - right ascension of ascending node dot (rate)
*    xpidot      -
*    z1, z3, z11, z13, z21, z23, z31, z33 -
*    eccm        - eccentricity
*    argpm       - argument of perigee
*    inclm       - inclination
*    mm          - mean anomaly
*    xn          - mean motion
*    nodem       - right ascension of ascending node
*
*  outputs       :
*    em          - eccentricity
*    argpm       - argument of perigee
*    inclm       - inclination
*    mm          - mean anomaly
*    nm          - mean motion
*    nodem       - right ascension of ascending node
*    irez        - flag for resonance           0-none, 1-one day, 2-half day
*    atime       -
*    d2201, d2211, d3210, d3222, d4410, d4422, d5220, d5232, d5421, d5433    -
*    dedt        -
*    didt        -
*    dmdt        -
*    dndt        -
*    dnodt       -
*    domdt       -
*    del1, del2, del3        -
*    ses  , sghl , sghs , sgs  , shl  , shs  , sis  , sls
*    theta       -
*    xfact       -
*    xlamo       -
*    xli         -
*    xni
*
*  locals        :
*    ainv2       -
*    aonv        -
*    cosisq      -
*    eoc         -
*    f220, f221, f311, f321, f322, f330, f441, f442, f522, f523, f542, f543  -
*    g200, g201, g211, g300, g310, g322, g410, g422, g520, g521, g532, g533  -
*    sini2       -
*    temp        -
*    temp1       -
*    theta       -
*    xno2        -
*
*  coupling      :
*    getgravconst
*
*  references    :
*    hoots, roehrich, norad spacetrack report #3 1980
*    hoots, norad spacetrack report #6 1986
*    hoots, schumacher and glover 2004
*    vallado, crawford, hujsak, kelso  2006
----------------------------------------------------------------------------*/
#[derive(Debug)]
#[allow(dead_code)]
pub struct DsInitOption {
    pub cosim: f64,
    pub argpo: f64,
    pub s1: f64,
    pub s2: f64,
    pub s3: f64,
    pub s4: f64,
    pub s5: f64,
    pub sinim: f64,
    pub ss1: f64,
    pub ss2: f64,
    pub ss3: f64,
    pub ss4: f64,
    pub ss5: f64,
    pub sz1: f64,
    pub sz3: f64,
    pub sz11: f64,
    pub sz13: f64,
    pub sz21: f64,
    pub sz23: f64,
    pub sz31: f64,
    pub sz33: f64,
    pub t: f64,
    pub tc: f64,
    pub gsto: f64,
    pub mo: f64,
    pub mdot: f64,
    pub no: f64,
    pub nodeo: f64,
    pub nodedot: f64,
    pub xpidot: f64,
    pub z1: f64,
    pub z3: f64,
    pub z11: f64,
    pub z13: f64,
    pub z21: f64,
    pub z23: f64,
    pub z31: f64,
    pub z33: f64,
    pub ecco: f64,
    pub eccsq: f64,
    pub emsq: f64,
    pub em: f64,
    pub argpm: f64,
    pub inclm: f64,
    pub mm: f64,
    pub nm: f64,
    pub nodem: f64,
    pub irez: u32,
    pub atime: f64,
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
    pub didt: f64,
    pub dmdt: f64,
    pub dnodt: f64,
    pub domdt: f64,
    pub del1: f64,
    pub del2: f64,
    pub del3: f64,
    pub xfact: f64,
    pub xlamo: f64,
    pub xli: f64,
    pub xni: f64,
}
#[derive(Debug, PartialEq)]
pub struct DsInitResult {
    pub em: f64,
    pub argpm: f64,
    pub inclm: f64,
    pub mm: f64,
    pub nm: f64,
    pub nodem: f64,

    pub irez: u32,
    pub atime: f64,

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
    pub didt: f64,
    pub dmdt: f64,
    pub dndt: f64,
    pub dnodt: f64,
    pub domdt: f64,

    pub del1: f64,
    pub del2: f64,
    pub del3: f64,

    pub xfact: f64,
    pub xlamo: f64,
    pub xli: f64,
    pub xni: f64,
}

pub fn dsinit(options: DsInitOption) -> DsInitResult {
    let cosim = options.cosim;
    let argpo = options.argpo;
    let s1 = options.s1;
    let s2 = options.s2;
    let s3 = options.s3;
    let s4 = options.s4;
    let s5 = options.s5;
    let sinim = options.sinim;
    let ss1 = options.ss1;
    let ss2 = options.ss2;
    let ss3 = options.ss3;
    let ss4 = options.ss4;
    let ss5 = options.ss5;
    let sz1 = options.sz1;
    let sz3 = options.sz3;
    let sz11 = options.sz11;
    let sz13 = options.sz13;
    let sz21 = options.sz21;
    let sz23 = options.sz23;
    let sz31 = options.sz31;
    let sz33 = options.sz33;
    let t = options.t;
    let tc = options.tc;
    let gsto = options.gsto;
    let mo = options.mo;
    let mdot = options.mdot;
    let no = options.no;
    let nodeo = options.nodeo;
    let nodedot = options.nodedot;
    let xpidot = options.xpidot;
    let z1 = options.z1;
    let z3 = options.z3;
    let z11 = options.z11;
    let z13 = options.z13;
    let z21 = options.z21;
    let z23 = options.z23;
    let z31 = options.z31;
    let z33 = options.z33;
    let ecco = options.ecco;
    let eccsq = options.eccsq;

    let mut emsq = options.emsq;
    let mut em: f64 = options.em;
    let mut argpm: f64 = options.argpm;
    let mut inclm: f64 = options.inclm;
    let mut mm: f64 = options.mm;
    let mut nm: f64 = options.nm;
    let mut nodem: f64 = options.nodem;
    let mut irez: u32 ;
    let mut atime: f64 = options.atime;
    let mut d2201: f64 = options.d2201;
    let mut d2211: f64 = options.d2211;
    let mut d3210: f64 = options.d3210;
    let mut d3222: f64 = options.d3222;
    let mut d4410: f64 = options.d4410;
    let mut d4422: f64 = options.d4422;
    let mut d5220: f64 = options.d5220;
    let mut d5232: f64 = options.d5232;
    let mut d5421: f64 = options.d5421;
    let mut d5433: f64 = options.d5433;
    let  dedt: f64 ;
    let  didt: f64 ;
    let  dmdt ;
    let mut dnodt;
    let mut domdt: f64 ;
    let mut del1: f64 = options.del1;
    let mut del2: f64 = options.del2;
    let mut del3: f64 = options.del3;
    let mut xfact: f64 = options.xfact;
    let mut xlamo: f64 = options.xlamo;
    let mut xli: f64 = options.xli;
    let mut xni: f64 = options.xni;

    let mut f220;
    let f221;
    let f311;
    let f321;
    let f322;
    let mut f330;
    let f441;
    let f442;
    let f522;
    let f523;
    let f542;
    let f543;
    let g200;
    let g201;
    let g211;
    let g300;
    let mut g310;
    let g322;
    let g410;
    let g422;
    let g520;
    let g521;
    let g532;
    let g533;
    let sini2;
    let mut temp;
    let mut temp1;
    let xno2;
    let ainv2;
    let aonv;
    let cosisq;
    let eoc;

    const Q22: f64 = 1.7891679e-6;
    const Q31: f64 = 2.1460748e-6;
    const Q33: f64 = 2.2123015e-7;
    const ROOT22: f64 = 1.7891679e-6;
    const ROOT44: f64 = 7.3636953e-9;
    const ROOT54: f64 = 2.1765803e-9;
    // eslint-disable-next-line no-loss-of-precision
    const RPTIM: f64 = 4.37526908801129966e-3; // equates to 7.29211514668855e-5 rad/sec
    const ROOT32: f64 = 3.7393792e-7;
    const ROOT52: f64 = 1.1428639e-7;
    const ZNL: f64 = 1.5835218e-4;
    const ZNS: f64 = 1.19459e-5;

    // -------------------- deep space initialization ------------
    irez = 0;
    if (nm < 0.0052359877) && (nm > 0.0034906585) {
        irez = 1;
    }
    if (nm >= 8.26e-3) && (nm <= 9.24e-3) && (em >= 0.5) {
        irez = 2;
    }

    // ------------------------ do solar terms -------------------
    let ses = ss1 * ZNS * ss5;
    let sis = ss2 * ZNS * (sz11 + sz13);
    let sls = -ZNS * ss3 * ((sz1 + sz3) - 14.0 - (6.0 * emsq));
    let sghs = ss4 * ZNS * ((sz31 + sz33) - 6.0);
    let mut shs = -ZNS * ss2 * (sz21 + sz23);

    // sgp4fix for 180 deg incl
    if inclm < 5.2359877e-2 || inclm > PI - 5.2359877e-2 {
        shs = 0.0;
    }
    if sinim != 0.0 {
        shs /= sinim;
    }
    let sgs = sghs - (cosim * shs);

    // ------------------------- do lunar terms ------------------
    dedt = ses + (s1 * ZNL * s5);
    didt = sis + (s2 * ZNL * (z11 + z13));
    dmdt = sls - (ZNL * s3 * ((z1 + z3) - 14.0 - (6.0 * emsq)));
    let sghl = s4 * ZNL * ((z31 + z33) - 6.0);
    let mut shll = -ZNL * s2 * (z21 + z23);

    // sgp4fix for 180 deg incl
    if (inclm < 5.2359877e-2) || (inclm > (PI - 5.2359877e-2)) {
        shll = 0.0;
    }
    domdt = sgs + sghl;
    dnodt = shs;
    if sinim != 0.0 {
        domdt -= (cosim / sinim) * shll;
        dnodt += shll / sinim;
    }

    // ----------- calculate deep space resonance effects --------
    let dndt = 0.0;
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
    //   inclm  = -inclm;
    //   argpm  = argpm - pi;
    //   nodem = nodem + pi;
    // }

    // -------------- initialize the resonance terms -------------
    if irez != 0 {
        aonv = (nm / XKE).powf(X2O3);

        // ---------- geopotential resonance for 12 hour orbits ------
        if irez == 2 {
            cosisq = cosim * cosim;
            let emo = em;
            em = ecco;
            let emsqo = emsq;
            emsq = eccsq;
            eoc = em * emsq;
            g201 = -0.306 - ((em - 0.64) * 0.440);

            if em <= 0.65 {
                g211 = (3.616 - (13.2470 * em)) + (16.2900 * emsq);
                g310 = ((-19.302 + (117.3900 * em)) - (228.4190 * emsq)) + (156.5910 * eoc);
                g322 = ((-18.9068 + (109.7927 * em)) - (214.6334 * emsq)) + (146.5816 * eoc);
                g410 = ((-41.122 + (242.6940 * em)) - (471.0940 * emsq)) + (313.9530 * eoc);
                g422 = ((-146.407 + (841.8800 * em)) - (1629.014 * emsq)) + (1083.4350 * eoc);
                g520 = ((-532.114 + (3017.977 * em)) - (5740.032 * emsq)) + (3708.2760 * eoc);
            } else {
                g211 = ((-72.099 + (331.819 * em)) - (508.738 * emsq)) + (266.724 * eoc);
                g310 = ((-346.844 + (1582.851 * em)) - (2415.925 * emsq)) + (1246.113 * eoc);
                g322 = ((-342.585 + (1554.908 * em)) - (2366.899 * emsq)) + (1215.972 * eoc);
                g410 = ((-1052.797 + (4758.686 * em)) - (7193.992 * emsq)) + (3651.957 * eoc);
                g422 = ((-3581.690 + (16178.110 * em)) - (24462.770 * emsq)) + (12422.520 * eoc);
                if em > 0.715 {
                    g520 = ((-5149.66 + (29936.92 * em)) - (54087.36 * emsq)) + (31324.56 * eoc);
                } else {
                    g520 = (1464.74 - (4664.75 * em)) + (3763.64 * emsq);
                }
            }
            if em < 0.7 {
                g533 = ((-919.22770 + (4988.6100 * em)) - (9064.7700 * emsq)) + (5542.21 * eoc);
                g521 = ((-822.71072 + (4568.6173 * em)) - (8491.4146 * emsq)) + (5337.524 * eoc);
                g532 = ((-853.66600 + (4690.2500 * em)) - (8624.7700 * emsq)) + (5341.4 * eoc);
            } else {
                g533 = ((-37995.780 + (161616.52 * em)) - (229838.20 * emsq)) + (109377.94 * eoc);
                g521 = ((-51752.104 + (218913.95 * em)) - (309468.16 * emsq)) + (146349.42 * eoc);
                g532 = ((-40023.880 + (170470.89 * em)) - (242699.48 * emsq)) + (115605.82 * eoc);
            }
            sini2 = sinim * sinim;
            f220 = 0.75 * (1.0 + (2.0 * cosim) + cosisq);
            f221 = 1.5 * sini2;
            f321 = 1.875 * sinim * (1.0 - (2.0 * cosim) - (3.0 * cosisq));
            f322 = -1.875 * sinim * ((1.0 + (2.0 * cosim)) - (3.0 * cosisq));
            f441 = 35.0 * sini2 * f220;
            f442 = 39.3750 * sini2 * sini2;

            f522 = 9.84375
                * sinim
                * ((sini2 * (1.0 - (2.0 * cosim) - (5.0 * cosisq)))
                    + (0.33333333 * (-2.0 + (4.0 * cosim) + (6.0 * cosisq))));
            f523 = sinim
                * ((4.92187512 * sini2 * ((-2.0 - (4.0 * cosim)) + (10.0 * cosisq)))
                    + (6.56250012 * ((1.0 + (2.0 * cosim)) - (3.0 * cosisq))));
            f542 = 29.53125
                * sinim
                * ((2.0 - (8.0 * cosim)) + (cosisq * (-12.0 + (8.0 * cosim) + (10.0 * cosisq))));
            f543 = 29.53125
                * sinim
                * ((-2.0 - (8.0 * cosim)) + (cosisq * ((12.0 + (8.0 * cosim)) - (10.0 * cosisq))));

            xno2 = nm * nm;
            ainv2 = aonv * aonv;
            temp1 = 3.0 * xno2 * ainv2;
            temp = temp1 * ROOT22;
            d2201 = temp * f220 * g201;
            d2211 = temp * f221 * g211;
            temp1 *= aonv;
            temp = temp1 * ROOT32;
            d3210 = temp * f321 * g310;
            d3222 = temp * f322 * g322;
            temp1 *= aonv;
            temp = 2.0 * temp1 * ROOT44;
            d4410 = temp * f441 * g410;
            d4422 = temp * f442 * g422;
            temp1 *= aonv;
            temp = temp1 * ROOT52;
            d5220 = temp * f522 * g520;
            d5232 = temp * f523 * g532;
            temp = 2.0 * temp1 * ROOT54;
            d5421 = temp * f542 * g521;
            d5433 = temp * f543 * g533;
            xlamo = ((mo + nodeo + nodeo) - (theta + theta)) % TWO_PI;
            xfact = (mdot + dmdt + (2.0 * ((nodedot + dnodt) - RPTIM))) - no;
            em = emo;
            emsq = emsqo;
        }

        //  ---------------- synchronous resonance terms --------------
        if irez == 1 {
            g200 = 1.0 + (emsq * (-2.5 + (0.8125 * emsq)));
            g310 = 1.0 + (2.0 * emsq);
            g300 = 1.0 + (emsq * (-6.0 + (6.60937 * emsq)));
            f220 = 0.75 * (1.0 + cosim) * (1.0 + cosim);
            f311 = (0.9375 * sinim * sinim * (1.0 + (3.0 * cosim))) - (0.75 * (1.0 + cosim));
            f330 = 1.0 + cosim;
            f330 *= 1.875 * f330 * f330;
            del1 = 3.0 * nm * nm * aonv * aonv;
            del2 = 2.0 * del1 * f220 * g200 * Q22;
            del3 = 3.0 * del1 * f330 * g300 * Q33 * aonv;
            del1 = del1 * f311 * g310 * Q31 * aonv;
            xlamo = ((mo + nodeo + argpo) - theta) % TWO_PI;
            xfact = (mdot + xpidot + dmdt + domdt + dnodt) - (no + RPTIM);
        }

        //  ------------ for sgp4, initialize the integrator ----------
        xli = xlamo;
        xni = no;
        atime = 0.0;
        nm = no + dndt;
    }

    DsInitResult {
        em,
        argpm,
        inclm,
        mm,
        nm,
        nodem,

        irez,
        atime,

        d2201,
        d2211,
        d3210,
        d3222,
        d4410,

        d4422,
        d5220,
        d5232,
        d5421,
        d5433,

        dedt,
        didt,
        dmdt,
        dndt,
        dnodt,
        domdt,

        del1,
        del2,
        del3,

        xfact,
        xlamo,
        xli,
        xni,
    }
}



#[cfg(test)]
mod test{
    use super::{
        dsinit,
        DsInitOption,
        DsInitResult
    };
    struct TestSet {
        options: DsInitOption,
        results: DsInitResult,
    }
    
    #[test]
fn geopotential_resonance_for_12_hour_orbits() {
    const TEST_DATA1: TestSet = TestSet {
        options: DsInitOption {
            argpm: 0.0,
            argpo: 3.1731953303556546,
            atime: 0.0,
            cosim: 0.8265818908073872,
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
            ecco: 0.5,
            eccsq: 0.25,
            em: 0.5,
            emsq: 0.25,
            gsto: 5.220883431349307,
            inclm: 0.5977892314420737,
            irez: 0,
            mdot: 0.008289877617360376,
            mm: 0.0,
            mo: 3.097818050620523,
            nm: 0.00828929401348305,
            no: 0.00828929401348305,
            nodedot: -0.0000010615465631002147,
            nodem: 0.0,
            nodeo: 5.684425672673404,
            s1: -0.0003758603820133531,
            s2: -0.00003340981173452028,
            s3: 0.000057867491395499995,
            s4: 0.00005011471760178041,
            s5: -0.0814586266145132,
            sinim: 0.5628164690103556,
            ss1: -0.0023400973145719425,
            ss2: -0.0002080086501841727,
            ss3: 0.0003602815505328084,
            ss4: 0.000312012975276259,
            ss5: -0.05506592822076295,
            sz1: 10.0266126658567,
            sz3: 11.96755448889018,
            sz11: -0.26695105200172087,
            sz13: 2.85835691924874,
            sz21: -0.10879395395455185,
            sz23: -0.9817900146669546,
            sz31: 3.2268633174603867,
            sz33: 5.297255866008874,
            t: 0.0,
            tc: 0.0,
            xfact: 0.0,
            xlamo: 0.0,
            xli: 0.0,
            xni: 0.0,
            xpidot: 4.898991347062741e-7,
            z1: 4.101694478238579,
            z3: 16.932733892867283,
            z11: 1.429557720331177,
            z13: 1.9707634473989213,
            z21: -1.4644752902058293,
            z23: 0.2884276608142515,
            z31: -0.8856293737107999,
            z33: 8.908977597159325,
        },
        results: DsInitResult {
            argpm: 0.0,
            atime: 0.0,
            d2201: -1.2099438856510436e-11,
            d2211: 1.0011494893832913e-11,
            d3210: -5.086681199981555e-12,
            d3222: -3.9879779675343986e-13,
            d4410: 4.10948324131911e-13,
            d4422: 9.349597736749112e-14,
            d5220: 2.4097823461671757e-13,
            d5232: 1.6478747122823163e-13,
            d5421: -2.1387372659636362e-13,
            d5433: -9.311219761949575e-15,
            dedt: 6.387624124699951e-9,
            del1: 0.0,
            del2: 0.0,
            del3: 0.0,
            didt: -2.4428711570103973e-8,
            dmdt: -7.866458523057076e-8,
            dndt: 0.0,
            dnodt: -1.5869893761619698e-8,
            domdt: 3.8582690952174514e-8,
            em: 0.5,
            inclm: 0.5977892314420737,
            irez: 2,
            mm: 0.0,
            nm: 0.00828929401348305,
            nodem: 0.0,
            xfact: -0.008752188069644229,
            xlamo: 4.024902533268717,
            xli: 4.024902533268717,
            xni: 0.00828929401348305,
        },
    };
    const TEST_DATA2: TestSet = TestSet {
        options: DsInitOption {
            argpm: 0.0,
            argpo: 3.1731953303556546,
            atime: 0.0,
            cosim: 0.8265818908073872,
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
            ecco: 0.9,
            eccsq: 0.81,
            em: 0.9,
            emsq: 0.81,
            gsto: 5.220883431349307,
            inclm: 0.5977892314420737,
            irez: 0,
            mdot: 0.008289882914214362,
            mm: 0.0,
            mo: 3.097818050620523,
            nm: 0.008285301381233555,
            no: 0.008285301381233555,
            nodedot: -0.000016615977679572516,
            nodem: 0.0,
            nodeo: 5.684425672673404,
            s1: -0.00034068613392845755,
            s2: -0.0000664105524227013,
            s3: 0.000057895377359053036,
            s4: 0.000025236009920626485,
            s5: -0.0814586266145132,
            sinim: 0.5628164690103556,
            ss1: -0.0021211033279095576,
            ss2: -0.00041347043429036223,
            ss3: 0.00036045516784271263,
            ss4: 0.0001571187650303376,
            ss5: -0.05506592822076295,
            sz1: 11.833656123634515,
            sz3: 14.934017773855151,
            sz11: -0.559025056566349,
            sz13: 6.1122684947234225,
            sz21: -0.012026488033904603,
            sz23: -0.41905685490385947,
            sz31: 3.2268633174603867,
            sz33: 5.297255866008874,
            t: 0.0,
            tc: 0.0,
            xfact: 0.0,
            xlamo: 0.0,
            xli: 0.0,
            xni: 0.0,
            xpidot: 0.000007654165958004881,
            z1: 3.6057420289605315,
            z3: 21.9217613472765,
            z11: 3.1143324610298544,
            z13: 4.157342147158473,
            z21: -0.4580055602764077,
            z23: -0.042575346093936206,
            z31: -0.8856293737107999,
            z33: 8.908977597159325,
        },
        results: DsInitResult {
            argpm: 0.0,
            atime: 0.0,
            d2201: -2.077922237635543e-11,
            d2211: 8.354974473880257e-11,
            d3210: -7.971713322895552e-11,
            d3222: -1.5879563982136425e-11,
            d4410: 1.575743786950506e-11,
            d4422: 7.56868465456182e-12,
            d5220: 3.642159807380104e-11,
            d5232: 6.0905980525896e-11,
            d5421: -4.636775124768956e-11,
            d5433: -5.740323981213709e-12,
            dedt: 5.789849295568645e-9,
            del1: 0.0,
            del2: 0.0,
            del3: 0.0,
            didt: -1.0389979451500435e-7,
            dmdt: -9.517687076400692e-8,
            dndt: 0.0,
            dnodt: -1.3136567693239499e-8,
            domdt: 2.3681689509031805e-8,
            em: 0.9,
            inclm: 0.5977892314420737,
            irez: 2,
            mm: 0.0,
            nm: 0.008285301381233555,
            nodem: 0.0,
            xfact: -0.00877931004840709,
            xlamo: 4.024902533268717,
            xli: 4.024902533268717,
            xni: 0.008285301381233555,
        },
    };

    assert_eq!(dsinit(TEST_DATA1.options), TEST_DATA1.results);
    assert_eq!(dsinit(TEST_DATA2.options), TEST_DATA2.results);
}

    
}