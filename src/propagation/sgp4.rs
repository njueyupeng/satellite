use crate::constants::{EARTH_RADIUS, J2, J3OJ2, PI, TWO_PI, VKMPERSEC, X2O3, XKE};
use crate::propagation::{
    dpper::{dpper, DpperOption},
    dspace::{dspace, DspaceOption},
};
use crate::types::DpperInit;
use crate::types::SatRec;
use crate::Vector3;

pub enum Sgp4Error {
    FF,
    FpFv,
}

pub struct Sgp4Result {
    position: Vector3,
    velocity: Vector3,
}

/*----------------------------------------------------------------------------
*
*                             procedure sgp4
*
*  this procedure is the sgp4 prediction model from space command. this is an
*    updated and combined version of sgp4 and sdp4, which were originally
*    published separately in spacetrack report //3. this version follows the
*    methodology from the aiaa paper (2006) describing the history and
*    development of the code.
*
*  author        : david vallado                  719-573-2600   28 jun 2005
*
*  inputs        :
*    satrec  - initialised structure from sgp4init() call.
*    tsince  - time since epoch (minutes)
*
*  outputs       :
*    r           - position vector                     km
*    v           - velocity                            km/sec
*  return code - non-zero on error.
*                   1 - mean elements, ecc >= 1.0 or ecc < -0.001 or a < 0.95 er
*                   2 - mean motion less than 0.0
*                   3 - pert elements, ecc < 0.0  or  ecc > 1.0
*                   4 - semi-latus rectum < 0.0
*                   5 - epoch elements are sub-orbital
*                   6 - satellite has decayed
*
*  locals        :
*    am          -
*    axnl, aynl        -
*    betal       -
*    cosim   , sinim   , cosomm  , sinomm  , cnod    , snod    , cos2u   ,
*    sin2u   , coseo1  , sineo1  , cosi    , sini    , cosip   , sinip   ,
*    cosisq  , cossu   , sinsu   , cosu    , sinu
*    delm        -
*    delomg      -
*    dndt        -
*    eccm        -
*    emsq        -
*    ecose       -
*    el2         -
*    eo1         -
*    eccp        -
*    esine       -
*    argpm       -
*    argpp       -
*    omgadf      -
*    pl          -
*    r           -
*    rtemsq      -
*    rdotl       -
*    rl          -
*    rvdot       -
*    rvdotl      -
*    su          -
*    t2  , t3   , t4    , tc
*    tem5, temp , temp1 , temp2  , tempa  , tempe  , templ
*    u   , ux   , uy    , uz     , vx     , vy     , vz
*    inclm       - inclination
*    mm          - mean anomaly
*    nm          - mean motion
*    nodem       - right asc of ascending node
*    xinc        -
*    xincp       -
*    xl          -
*    xlm         -
*    mp          -
*    xmdf        -
*    xmx         -
*    xmy         -
*    nodedf      -
*    xnode       -
*    nodep       -
*    np          -
*
*  coupling      :
*    getgravconst-
*    dpper
*    dspace
*
*  references    :
*    hoots, roehrich, norad spacetrack report //3 1980
*    hoots, norad spacetrack report //6 1986
*    hoots, schumacher and glover 2004
*    vallado, crawford, hujsak, kelso  2006
----------------------------------------------------------------------------*/

pub fn sgp4(satrec: &mut SatRec, tsince: f64) -> Result<Sgp4Result, Sgp4Error> {
    let mut coseo1 = 0.0;
    let mut sineo1 = 0.0;
    let mut cosip;
    let mut sinip;
    let cosisq;
    let delm;
    let delomg;
    let mut eo1;
    let mut argpm;
    let mut argpp;
    let mut su;
    let t3;
    let t4;
    let tc;
    let mut tem5;
    let mut temp;
    let mut tempa;
    let mut tempe;
    let mut templ;
    let mut inclm;
    let mut mm;
    let mut nm;
    let mut nodem;
    let mut xincp;
    let mut xlm;
    let mut mp;
    let mut nodep;

    /* ------------------ set mathematical constants --------------- */
    // sgp4fix divisor for divide by zero check on inclination
    // the old check used 1.0 + cos(pi-1.0e-9), but then compared it to
    // 1.5 e-12, so the threshold was changed to 1.5e-12 for consistency

    const temp4: f64 = 1.5e-12;

    // --------------------- clear sgp4 error flag -----------------
    satrec.t = tsince;
    satrec.error = 0;

    //  ------- update for secular gravity and atmospheric drag -----
    let xmdf = satrec.mo + (satrec.mdot * satrec.t);
    let argpdf = satrec.argpo + (satrec.argpdot * satrec.t);
    let nodedf = satrec.nodeo + (satrec.nodedot * satrec.t);
    argpm = argpdf;
    mm = xmdf;
    let t2 = satrec.t * satrec.t;
    nodem = nodedf + (satrec.nodecf * t2);
    tempa = 1.0 - (satrec.cc1 * satrec.t);
    tempe = satrec.bstar * satrec.cc4 * satrec.t;
    templ = satrec.t2cof * t2;

    if satrec.isimp != 1 {
        delomg = satrec.omgcof * satrec.t;
        //  sgp4fix use mutliply for speed instead of pow
        let delmtemp = 1.0 + (satrec.eta * (xmdf).cos());
        delm = satrec.xmcof * ((delmtemp * delmtemp * delmtemp) - satrec.delmo);
        temp = delomg + delm;
        mm = xmdf + temp;
        argpm = argpdf - temp;
        t3 = t2 * satrec.t;
        t4 = t3 * satrec.t;
        tempa = tempa - (satrec.d2 * t2) - (satrec.d3 * t3) - (satrec.d4 * t4);
        tempe += satrec.bstar * satrec.cc5 * ((mm).sin() - satrec.sinmao);
        templ = templ + (satrec.t3cof * t3) + (t4 * (satrec.t4cof + (satrec.t * satrec.t5cof)));
    }
    nm = satrec.no;
    let mut em = satrec.ecco;
    inclm = satrec.inclo;
    if satrec.method == 'd' {
        tc = satrec.t;

        let dspace_options = DspaceOption {
            irez: satrec.irez,
            d2201: satrec.d2201,
            d2211: satrec.d2211,
            d3210: satrec.d3210,
            d3222: satrec.d3222,
            d4410: satrec.d4410,
            d4422: satrec.d4422,
            d5220: satrec.d5220,
            d5232: satrec.d5232,
            d5421: satrec.d5421,
            d5433: satrec.d5433,
            dedt: satrec.dedt,
            del1: satrec.del1,
            del2: satrec.del2,
            del3: satrec.del3,
            didt: satrec.didt,
            dmdt: satrec.dmdt,
            dnodt: satrec.dnodt,
            domdt: satrec.domdt,
            argpo: satrec.argpo,
            argpdot: satrec.argpdot,
            t: satrec.t,
            tc,
            gsto: satrec.gsto,
            xfact: satrec.xfact,
            xlamo: satrec.xlamo,
            no: satrec.no,
            atime: satrec.atime,
            em,
            argpm,
            inclm,
            xli: satrec.xli,
            mm,
            xni: satrec.xni,
            nodem,
            nm,
        };

        let dspace_result = dspace(dspace_options);
        em = dspace_result.em;
        argpm = dspace_result.argpm;
        inclm = dspace_result.inclm;
        mm = dspace_result.mm;
        nodem = dspace_result.nodem;
        nm = dspace_result.nm;
    }

    if nm <= 0.0 {
        // printf("// error nm %f\n", nm);
        satrec.error = 2;
        // sgp4fix add return
        return Err(Sgp4Error::FF);
    }

    let am = ((XKE / nm).powf(X2O3)) * tempa * tempa;
    nm = XKE / (am.powf(1.5));
    em -= tempe;

    // fix tolerance for error recognition
    // sgp4fix am is fixed from the previous nm check
    if em >= 1.0 || em < -0.001 {
        // || (am < 0.95)
        // printf("// error em %f\n", em);
        satrec.error = 1;
        // sgp4fix to return if there is an error in eccentricity
        return Err(Sgp4Error::FF);
    }

    //  sgp4fix fix tolerance to avoid a divide by zero
    if em < 1.0e-6 {
        em = 1.0e-6;
    }
    mm += satrec.no * templ;
    xlm = mm + argpm + nodem;

    nodem %= TWO_PI;
    argpm %= TWO_PI;
    xlm %= TWO_PI;
    mm = (xlm - argpm - nodem) % TWO_PI;

    // ----------------- compute extra mean quantities -------------
    let sinim = (inclm).sin();
    let cosim = (inclm).cos();

    // -------------------- add lunar-solar periodics --------------
    let mut ep = em;
    xincp = inclm;
    argpp = argpm;
    nodep = nodem;
    mp = mm;
    sinip = sinim;
    cosip = cosim;
    if satrec.method == 'd' {
        let dpper_parameters = DpperOption {
            init: DpperInit::N,
            ep,
            inclp: xincp,
            nodep,
            argpp,
            mp,
            opsmode: satrec.operationmode.clone(),
        };

        let dpper_result = dpper(&satrec, &dpper_parameters);

        ep = dpper_result.ep;
        nodep = dpper_result.nodep;
        argpp = dpper_result.argpp;
        mp = dpper_result.mp;

        xincp = dpper_result.inclp;

        if xincp < 0.0 {
            xincp = -xincp;
            nodep += PI;
            argpp -= PI;
        }
        if ep < 0.0 || ep > 1.0 {
            //  printf("// error ep %f\n", ep);
            satrec.error = 3;
            //  sgp4fix add return
            return Err(Sgp4Error::FF);
        }
    }

    //  -------------------- long period periodics ------------------
    if satrec.method == 'd' {
        sinip = (xincp).sin();
        cosip = (xincp).cos();
        satrec.aycof = -0.5 * J3OJ2 * sinip;

        //  sgp4fix for divide by zero for xincp = 180 deg
        if (cosip + 1.0).abs() > 1.5e-12 {
            satrec.xlcof = (-0.25 * J3OJ2 * sinip * (3.0 + (5.0 * cosip))) / (1.0 + cosip);
        } else {
            satrec.xlcof = (-0.25 * J3OJ2 * sinip * (3.0 + (5.0 * cosip))) / temp4;
        }
    }

    let axnl = ep * (argpp.cos());
    temp = 1.0 / (am * (1.0 - (ep * ep)));
    let aynl = (ep * (argpp.sin())) + (temp * satrec.aycof);
    let xl = mp + argpp + nodep + (temp * satrec.xlcof * axnl);

    // --------------------- solve kepler's equation ---------------
    let u = (xl - nodep) % TWO_PI;
    eo1 = u;
    tem5 = 9999.9_f64;
    let mut ktr = 1;

    //    sgp4fix for kepler iteration
    //    the following iteration needs better limits on corrections
    while tem5.abs() >= 1.0e-12 && ktr <= 10 {
        sineo1 = eo1.sin();
        coseo1 = eo1.cos();
        tem5 = 1.0 - (coseo1 * axnl) - (sineo1 * aynl);
        tem5 = (((u - (aynl * coseo1)) + (axnl * sineo1)) - eo1) / tem5;
        if tem5.abs() >= 0.95 {
            if tem5 > 0.0 {
                tem5 = 0.95;
            } else {
                tem5 = -0.95;
            }
        }
        eo1 += tem5;
        ktr += 1;
    }

    //  ------------- short period preliminary quantities -----------
    let ecose = (axnl * coseo1) + (aynl * sineo1);
    let esine = (axnl * sineo1) - (aynl * coseo1);
    let el2 = (axnl * axnl) + (aynl * aynl);
    let pl = am * (1.0 - el2);
    if pl < 0.0 {
        //  printf("// error pl %f\n", pl);
        satrec.error = 4;
        //  sgp4fix add return
        return Err(Sgp4Error::FF);
    }

    let rl = am * (1.0 - ecose);
    let rdotl = (am.sqrt() * esine) / rl;
    let rvdotl = pl.sqrt() / rl;
    let betal = (1.0 - el2).sqrt();
    temp = esine / (1.0 + betal);
    let sinu = (am / rl) * (sineo1 - aynl - (axnl * temp));
    let cosu = (am / rl) * ((coseo1 - axnl) + (aynl * temp));
    su = sinu.atan2(cosu);
    let sin2u = (cosu + cosu) * sinu;
    let cos2u = 1.0 - (2.0 * sinu * sinu);
    temp = 1.0 / pl;
    let temp1 = 0.5 * J2 * temp;
    let temp2 = temp1 * temp;

    // -------------- update for short period periodics ------------
    if satrec.method == 'd' {
        cosisq = cosip * cosip;
        satrec.con41 = (3.0 * cosisq) - 1.0;
        satrec.x1mth2 = 1.0 - cosisq;
        satrec.x7thm1 = (7.0 * cosisq) - 1.0;
    }

    let mrt =
        (rl * (1.0 - (1.5 * temp2 * betal * satrec.con41))) + (0.5 * temp1 * satrec.x1mth2 * cos2u);

    // sgp4fix for decaying satellites
    if mrt < 1.0 {
        // printf("// decay condition %11.6f \n",mrt);
        satrec.error = 6;
        return Err(Sgp4Error::FpFv);
    }

    su -= 0.25 * temp2 * satrec.x7thm1 * sin2u;
    let xnode = nodep + (1.5 * temp2 * cosip * sin2u);
    let xinc = xincp + (1.5 * temp2 * cosip * sinip * cos2u);
    let mvt = rdotl - ((nm * temp1 * satrec.x1mth2 * sin2u) / XKE);
    let rvdot = rvdotl + ((nm * temp1 * ((satrec.x1mth2 * cos2u) + (1.5 * satrec.con41))) / XKE);

    // --------------------- orientation vectors -------------------
    let sinsu = su.sin();
    let cossu = su.cos();
    let snod = xnode.sin();
    let cnod = xnode.cos();
    let sini = xinc.sin();
    let cosi = xinc.cos();
    let xmx = -snod * cosi;
    let xmy = cnod * cosi;
    let ux = (xmx * sinsu) + (cnod * cossu);
    let uy = (xmy * sinsu) + (snod * cossu);
    let uz = sini * sinsu;
    let vx = (xmx * cossu) - (cnod * sinsu);
    let vy = (xmy * cossu) - (snod * sinsu);
    let vz = sini * cossu;

    // --------- position and velocity (in km and km/sec) ----------
    let r = Vector3 {
        x: (mrt * ux) * EARTH_RADIUS,
        y: (mrt * uy) * EARTH_RADIUS,
        z: (mrt * uz) * EARTH_RADIUS,
    };
    let v = Vector3 {
        x: ((mvt * ux) + (rvdot * vx)) * VKMPERSEC,
        y: ((mvt * uy) + (rvdot * vy)) * VKMPERSEC,
        z: ((mvt * uz) + (rvdot * vz)) * VKMPERSEC,
    };

    return Ok(Sgp4Result {
        position: r,
        velocity: v,
    });
}
