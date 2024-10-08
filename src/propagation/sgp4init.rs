use crate::constants::{EARTH_RADIUS, J2, J3OJ2, J4, PI, TUMIN, X2O3};
use crate::propagation::{
    dpper::{dpper, DpperOption},
    dscom::{dscom, DscomOption},
    dsinit::{dsinit, DsInitOption},
    initl::{initl, InitOptions},
    sgp4::sgp4,
};
use crate::{DpperInit, DpperOpsMode, SatRec};
#[allow(dead_code)]
pub struct Sgp4InitOptions {
    pub opsmode: DpperOpsMode,
    pub satn: f64,
    pub epoch: f64,
    pub xbstar: f64,
    pub xecco: f64,
    pub xargpo: f64,
    pub xinclo: f64,
    pub xmo: f64,
    pub xno: f64,
    pub xnodeo: f64,
}

/*-----------------------------------------------------------------------------
*
*                             procedure sgp4init
*
*  this procedure initializes variables for sgp4.
*
*  author        : david vallado                  719-573-2600   28 jun 2005
*  author        : david vallado                  719-573-2600   28 jun 2005
*
*  inputs        :
*    opsmode     - mode of operation afspc or improved 'a', 'i'
*    satn        - satellite number
*    bstar       - sgp4 type drag coefficient              kg/m2er
*    ecco        - eccentricity
*    epoch       - epoch time in days from jan 0, 1950. 0 hour
*    argpo       - argument of perigee (output if ds)
*    inclo       - inclination
*    mo          - mean anomaly (output if ds)
*    no          - mean motion
*    nodeo       - right ascension of ascending node
*
*  outputs       :
*    rec      - common values for subsequent calls
*    return code - non-zero on error.
*                   1 - mean elements, ecc >= 1.0 or ecc < -0.001 or a < 0.95 er
*                   2 - mean motion less than 0.0
*                   3 - pert elements, ecc < 0.0  or  ecc > 1.0
*                   4 - semi-latus rectum < 0.0
*                   5 - epoch elements are sub-orbital
*                   6 - satellite has decayed
*
*  locals        :
*    cnodm  , snodm  , cosim  , sinim  , cosomm , sinomm
*    cc1sq  , cc2    , cc3
*    coef   , coef1
*    cosio4      -
*    day         -
*    dndt        -
*    em          - eccentricity
*    emsq        - eccentricity squared
*    eeta        -
*    etasq       -
*    gam         -
*    argpm       - argument of perigee
*    nodem       -
*    inclm       - inclination
*    mm          - mean anomaly
*    nm          - mean motion
*    perige      - perigee
*    pinvsq      -
*    psisq       -
*    qzms24      -
*    rtemsq      -
*    s1, s2, s3, s4, s5, s6, s7          -
*    sfour       -
*    ss1, ss2, ss3, ss4, ss5, ss6, ss7         -
*    sz1, sz2, sz3
*    sz11, sz12, sz13, sz21, sz22, sz23, sz31, sz32, sz33        -
*    tc          -
*    temp        -
*    temp1, temp2, temp3       -
*    tsi         -
*    xpidot      -
*    xhdot1      -
*    z1, z2, z3          -
*    z11, z12, z13, z21, z22, z23, z31, z32, z33         -
*
*  coupling      :
*    getgravconst-
*    initl       -
*    dscom       -
*    dpper       -
*    dsinit      -
*    sgp4        -
*
*  references    :
*    hoots, roehrich, norad spacetrack report #3 1980
*    hoots, norad spacetrack report #6 1986
*    hoots, schumacher and glover 2004
*    vallado, crawford, hujsak, kelso  2006
----------------------------------------------------------------------------*/

pub fn sgp4init(satrec: &mut SatRec, options: Sgp4InitOptions) -> () {
    let opsmode = options.opsmode;
    let _satn:f64;
    let epoch = options.epoch;
    let xbstar = options.xbstar;
    let xecco = options.xecco;
    let xargpo = options.xargpo;
    let xinclo = options.xinclo;
    let xmo = options.xmo;
    let xno = options.xno;
    let xnodeo = options.xnodeo;

    let cosim;
    let sinim;
    let cc1sq;
    let cc2;
    let mut cc3;
    let coef;
    let coef1;
    let cosio4;
    let em;
    let emsq;
    let eeta;
    let etasq;
    let argpm;
    let nodem;
    let inclm;
    let mm;
    let nm;
    let perige;
    let pinvsq;
    let psisq;
    let mut qzms24;
    let s1;
    let s2;
    let s3;
    let s4;
    let s5;
    let mut sfour;
    let ss1;
    let ss2;
    let ss3;
    let ss4;
    let ss5;
    let sz1;
    let sz3;
    let sz11;
    let sz13;
    let sz21;
    let sz23;
    let sz31;
    let sz33;
    let tc;
    let temp;
    let temp1;
    let temp2;
    let temp3;
    let tsi;
    let xpidot;
    let xhdot1;
    let z1;
    let z3;
    let z11;
    let z13;
    let z21;
    let z23;
    let z31;
    let z33;

    /* ------------------------ initialization --------------------- */
    // sgp4fix divisor for divide by zero check on inclination
    // the old check used 1.0 + Math.cos(pi-1.0e-9), but then compared it to
    // 1.5 e-12, so the threshold was changed to 1.5e-12 for consistency
    let temp4 = 1.5e-12;

    // ----------- set all near earth variables to zero ------------
    satrec.isimp = 0;
    satrec.method = 'n';
    satrec.aycof = 0.0;
    satrec.con41 = 0.0;
    satrec.cc1 = 0.0;
    satrec.cc4 = 0.0;
    satrec.cc5 = 0.0;
    satrec.d2 = 0.0;
    satrec.d3 = 0.0;
    satrec.d4 = 0.0;
    satrec.delmo = 0.0;
    satrec.eta = 0.0;
    satrec.argpdot = 0.0;
    satrec.omgcof = 0.0;
    satrec.sinmao = 0.0;
    satrec.t = 0.0;
    satrec.t2cof = 0.0;
    satrec.t3cof = 0.0;
    satrec.t4cof = 0.0;
    satrec.t5cof = 0.0;
    satrec.x1mth2 = 0.0;
    satrec.x7thm1 = 0.0;
    satrec.mdot = 0.0;
    satrec.nodedot = 0.0;
    satrec.xlcof = 0.0;
    satrec.xmcof = 0.0;
    satrec.nodecf = 0.0;

    // ----------- set all deep space variables to zero ------------
    satrec.irez = 0;
    satrec.d2201 = 0.0;
    satrec.d2211 = 0.0;
    satrec.d3210 = 0.0;
    satrec.d3222 = 0.0;
    satrec.d4410 = 0.0;
    satrec.d4422 = 0.0;
    satrec.d5220 = 0.0;
    satrec.d5232 = 0.0;
    satrec.d5421 = 0.0;
    satrec.d5433 = 0.0;
    satrec.dedt = 0.0;
    satrec.del1 = 0.0;
    satrec.del2 = 0.0;
    satrec.del3 = 0.0;
    satrec.didt = 0.0;
    satrec.dmdt = 0.0;
    satrec.dnodt = 0.0;
    satrec.domdt = 0.0;
    satrec.e3 = 0.0;
    satrec.ee2 = 0.0;
    satrec.peo = 0.0;
    satrec.pgho = 0.0;
    satrec.pho = 0.0;
    satrec.pinco = 0.0;
    satrec.plo = 0.0;
    satrec.se2 = 0.0;
    satrec.se3 = 0.0;
    satrec.sgh2 = 0.0;
    satrec.sgh3 = 0.0;
    satrec.sgh4 = 0.0;
    satrec.sh2 = 0.0;
    satrec.sh3 = 0.0;
    satrec.si2 = 0.0;
    satrec.si3 = 0.0;
    satrec.sl2 = 0.0;
    satrec.sl3 = 0.0;
    satrec.sl4 = 0.0;
    satrec.gsto = 0.0;
    satrec.xfact = 0.0;
    satrec.xgh2 = 0.0;
    satrec.xgh3 = 0.0;
    satrec.xgh3 = 0.0;
    satrec.xh2 = 0.0;
    satrec.xh3 = 0.0;
    satrec.xi2 = 0.0;
    satrec.xi3 = 0.0;
    satrec.xl2 = 0.0;
    satrec.xl3 = 0.0;
    satrec.xl4 = 0.0;
    satrec.xlamo = 0.0;
    satrec.zmol = 0.0;
    satrec.zmos = 0.0;
    satrec.atime = 0.0;
    satrec.xli = 0.0;
    satrec.xni = 0.0;

    // sgp4fix - note the following variables are also passed directly via satrec.
    // it is possible to streamline the sgp4init call by deleting the "x"
    // variables, but the user would need to set the satrec.* values first. we
    // include the additional assignments in case twoline2rv is not used.

    satrec.bstar = xbstar;
    satrec.ecco = xecco;
    satrec.argpo = xargpo;
    satrec.inclo = xinclo;
    satrec.mo = xmo;
    satrec.no = xno;
    satrec.nodeo = xnodeo;

    //  sgp4fix add opsmode
    satrec.operationmode = opsmode;

    // ------------------------ earth constants -----------------------
    // sgp4fix identify constants and allow alternate values

    let ss = (78.0 / EARTH_RADIUS) + 1.0;
    // sgp4fix use multiply for speed instead of pow
    let qzms2ttemp = (120.0 - 78.0) / EARTH_RADIUS;
    let qzms2t = qzms2ttemp * qzms2ttemp * qzms2ttemp * qzms2ttemp;

    satrec.init = DpperInit::Y.clone();
    satrec.t = 0.0;

    let init_options = InitOptions {
        ecco: satrec.ecco,
        epoch: epoch,
        inclo: satrec.inclo,
        no: satrec.no,
        opsmode: satrec.operationmode.clone(),
    };

    let init_result = initl(init_options);

    let ao = init_result.ao;
    let con42 = init_result.con42;
    let cosio = init_result.cosio;
    let cosio2 = init_result.cosio2;
    let eccsq = init_result.eccsq;
    let omeosq = init_result.omeosq;
    let posq = init_result.posq;
    let rteosq = init_result.rteosq;
    let rp = init_result.rp;
    let sinio = init_result.sinio;

    satrec.no = init_result.no;
    satrec.con41 = init_result.con41;
    satrec.gsto = init_result.gsto;
    satrec.a = (satrec.no * TUMIN).powf(-2.0 / 3.0);
    satrec.alta = satrec.a * (1.0 + satrec.ecco) - 1.0;
    satrec.altp = satrec.a * (1.0 - satrec.ecco) - 1.0;
    satrec.error = 0;

    // sgp4fix remove this check as it is unnecessary
    // the mrt check in sgp4 handles decaying satellite cases even if the starting
    // condition is below the surface of te earth
    // if (rp < 1.0)
    // {
    //   printf("// *** satn%d epoch elts sub-orbital ***\n", satn);
    //   satrec.error = 5;
    // }

    if omeosq >= 0.0 || satrec.no >= 0.0 {
        satrec.isimp = 0;
        if rp < (220.0 / EARTH_RADIUS + 1.0) {
            satrec.isimp = 1;
        }
        sfour = ss;
        qzms24 = qzms2t;
        perige = (rp - 1.0) * EARTH_RADIUS;

        // - for perigees below 156 km, s and qoms2t are altered -
        if perige < 156.0 {
            sfour = perige - 78.0;
            if perige < 98.0 {
                sfour = 20.0;
            }

            // sgp4fix use multiply for speed instead of pow
            let qzms24temp = (120.0 - sfour) / EARTH_RADIUS;
            qzms24 = qzms24temp * qzms24temp * qzms24temp * qzms24temp;
            sfour = (sfour / EARTH_RADIUS) + 1.0;
        }
        pinvsq = 1.0 / posq;

        tsi = 1.0 / (ao - sfour);
        satrec.eta = ao * satrec.ecco * tsi;
        etasq = satrec.eta * satrec.eta;
        eeta = satrec.ecco * satrec.eta;
        psisq = (1.0 - etasq).abs();
        coef = qzms24 * (tsi.powf(4.0));
        coef1 = coef / (psisq.powf(3.5));
        cc2 = coef1
            * satrec.no
            * ((ao * (1.0 + (1.5 * etasq) + (eeta * (4.0 + etasq))))
                + (((0.375 * J2 * tsi) / psisq)
                    * satrec.con41
                    * (8.0 + (3.0 * etasq * (8.0 + etasq)))));
        satrec.cc1 = satrec.bstar * cc2;
        cc3 = 0.0;
        if satrec.ecco > 1.0e-4 {
            cc3 = (-2.0 * coef * tsi * J3OJ2 * satrec.no * sinio) / satrec.ecco;
        }
        satrec.x1mth2 = 1.0 - cosio2;
        satrec.cc4 = 2.0
            * satrec.no
            * coef1
            * ao
            * omeosq
            * (((satrec.eta * (2.0 + (0.5 * etasq))) + (satrec.ecco * (0.5 + (2.0 * etasq))))
                - (((J2 * tsi) / (ao * psisq))
                    * ((-3.0
                        * satrec.con41
                        * ((1.0 - (2.0 * eeta)) + (etasq * (1.5 - (0.5 * eeta)))))
                        + (0.75
                            * satrec.x1mth2
                            * ((2.0 * etasq) - (eeta * (1.0 + etasq)))
                            * (2.0 * satrec.argpo).cos()))));
        satrec.cc5 = 2.0 * coef1 * ao * omeosq * (1.0 + (2.75 * (etasq + eeta)) + (eeta * etasq));
        cosio4 = cosio2 * cosio2;
        temp1 = 1.5 * J2 * pinvsq * satrec.no;
        temp2 = 0.5 * temp1 * J2 * pinvsq;
        temp3 = -0.46875 * J4 * pinvsq * pinvsq * satrec.no;
        satrec.mdot = satrec.no
            + (0.5 * temp1 * rteosq * satrec.con41)
            + (0.0625 * temp2 * rteosq * ((13.0 - (78.0 * cosio2)) + (137.0 * cosio4)));
        satrec.argpdot = (-0.5 * temp1 * con42)
            + (0.0625 * temp2 * ((7.0 - (114.0 * cosio2)) + (395.0 * cosio4)))
            + (temp3 * ((3.0 - (36.0 * cosio2)) + (49.0 * cosio4)));
        xhdot1 = -temp1 * cosio;
        satrec.nodedot = xhdot1
            + (((0.5 * temp2 * (4.0 - (19.0 * cosio2))) + (2.0 * temp3 * (3.0 - (7.0 * cosio2))))
                * cosio);
        xpidot = satrec.argpdot + satrec.nodedot;
        satrec.omgcof = satrec.bstar * cc3 * (satrec.argpo).cos();
        satrec.xmcof = 0.0;
        if satrec.ecco > 1.0e-4 {
            satrec.xmcof = (-X2O3 * coef * satrec.bstar) / eeta;
        }
        satrec.nodecf = 3.5 * omeosq * xhdot1 * satrec.cc1;
        satrec.t2cof = 1.5 * satrec.cc1;

        // sgp4fix for divide by zero with xinco = 180 deg
        if (cosio + 1.0).abs() > 1.5e-12 {
            satrec.xlcof = (-0.25 * J3OJ2 * sinio * (3.0 + (5.0 * cosio))) / (1.0 + cosio);
        } else {
            satrec.xlcof = (-0.25 * J3OJ2 * sinio * (3.0 + (5.0 * cosio))) / temp4;
        }
        satrec.aycof = -0.5 * J3OJ2 * sinio;

        // sgp4fix use multiply for speed instead of pow
        let delmotemp = 1.0 + (satrec.eta * (satrec.mo).cos());
        satrec.delmo = delmotemp * delmotemp * delmotemp;
        satrec.sinmao = (satrec.mo).sin();
        satrec.x7thm1 = (7.0 * cosio2) - 1.0;

        // --------------- deep space initialization -------------
        if (2.0 * PI) / satrec.no >= 225.0 {
            satrec.method = 'd';
            satrec.isimp = 1;
            tc = 0.0;
            inclm = satrec.inclo;

            let dscom_option = DscomOption {
                epoch: epoch,
                ep: satrec.ecco,
                argpp: satrec.argpo,
                tc: tc,
                inclp: satrec.inclo,
                nodep: satrec.nodeo,
                np: satrec.no,
            };

            let dscom_result = dscom(&dscom_option);

            satrec.e3 = dscom_result.e3;
            satrec.ee2 = dscom_result.ee2;

            satrec.peo = dscom_result.peo;
            satrec.pgho = dscom_result.pgho;
            satrec.pho = dscom_result.pho;

            satrec.pinco = dscom_result.pinco;
            satrec.plo = dscom_result.plo;
            satrec.se2 = dscom_result.se2;
            satrec.se3 = dscom_result.se3;

            satrec.sgh2 = dscom_result.sgh2;
            satrec.sgh3 = dscom_result.sgh3;
            satrec.sgh4 = dscom_result.sgh4;
            satrec.sh2 = dscom_result.sh2;
            satrec.sh3 = dscom_result.sh3;

            satrec.si2 = dscom_result.si2;
            satrec.si3 = dscom_result.si3;
            satrec.sl2 = dscom_result.sl2;
            satrec.sl3 = dscom_result.sl3;
            satrec.sl4 = dscom_result.sl4;

            sinim = dscom_result.sinim;
            cosim = dscom_result.cosim;
            em = dscom_result.em;
            emsq = dscom_result.emsq;
            s1 = dscom_result.s1;
            s2 = dscom_result.s2;
            s3 = dscom_result.s3;
            s4 = dscom_result.s4;
            s5 = dscom_result.s5;
            ss1 = dscom_result.ss1;
            ss2 = dscom_result.ss2;
            ss3 = dscom_result.ss3;
            ss4 = dscom_result.ss4;
            ss5 = dscom_result.ss5;
            sz1 = dscom_result.sz1;
            sz3 = dscom_result.sz3;
            sz11 = dscom_result.sz11;
            sz13 = dscom_result.sz13;
            sz21 = dscom_result.sz21;
            sz23 = dscom_result.sz23;
            sz31 = dscom_result.sz31;
            sz33 = dscom_result.sz33;

            satrec.xgh2 = dscom_result.xgh2;
            satrec.xgh3 = dscom_result.xgh3;
            satrec.xgh4 = dscom_result.xgh4;
            satrec.xh2 = dscom_result.xh2;
            satrec.xh3 = dscom_result.xh3;
            satrec.xi2 = dscom_result.xi2;
            satrec.xi3 = dscom_result.xi3;
            satrec.xl2 = dscom_result.xl2;
            satrec.xl3 = dscom_result.xl3;
            satrec.xl4 = dscom_result.xl4;
            satrec.zmol = dscom_result.zmol;
            satrec.zmos = dscom_result.zmos;

            nm = dscom_result.nm;
            z1 = dscom_result.z1;
            z3 = dscom_result.z3;
            z11 = dscom_result.z11;
            z13 = dscom_result.z13;
            z21 = dscom_result.z21;
            z23 = dscom_result.z23;
            z31 = dscom_result.z31;
            z33 = dscom_result.z33;

            let dpper_options = DpperOption {
                init: satrec.init.clone(),
                ep: satrec.ecco,
                inclp: satrec.inclo,
                nodep: satrec.nodeo,
                argpp: satrec.argpo,
                mp: satrec.mo,
                opsmode: satrec.operationmode.clone(),
            };

            let dpper_result = dpper(&satrec, &dpper_options);

            satrec.ecco = dpper_result.ep;
            satrec.inclo = dpper_result.inclp;
            satrec.nodeo = dpper_result.nodep;
            satrec.argpo = dpper_result.argpp;
            satrec.mo = dpper_result.mp;

            argpm = 0.0;
            nodem = 0.0;
            mm = 0.0;

            let dsinit_options = DsInitOption {
                cosim,
                emsq,
                argpo: satrec.argpo,
                s1,
                s2,
                s3,
                s4,
                s5,
                sinim,
                ss1,
                ss2,
                ss3,
                ss4,
                ss5,
                sz1,
                sz3,
                sz11,
                sz13,
                sz21,
                sz23,
                sz31,
                sz33,
                t: satrec.t,
                tc,
                gsto: satrec.gsto,
                mo: satrec.mo,
                mdot: satrec.mdot,
                no: satrec.no,
                nodeo: satrec.nodeo,
                nodedot: satrec.nodedot,
                xpidot,
                z1,
                z3,
                z11,
                z13,
                z21,
                z23,
                z31,
                z33,
                ecco: satrec.ecco,
                eccsq,
                em,
                argpm,
                inclm,
                mm,
                nm,
                nodem,
                irez: satrec.irez,
                atime: satrec.atime,
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
                didt: satrec.didt,
                dmdt: satrec.dmdt,
                dnodt: satrec.dnodt,
                domdt: satrec.domdt,
                del1: satrec.del1,
                del2: satrec.del2,
                del3: satrec.del3,
                xfact: satrec.xfact,
                xlamo: satrec.xlamo,
                xli: satrec.xli,
                xni: satrec.xni,
            };

            let dsinit_result = dsinit(dsinit_options);

            satrec.irez = dsinit_result.irez;
            satrec.atime = dsinit_result.atime;
            satrec.d2201 = dsinit_result.d2201;
            satrec.d2211 = dsinit_result.d2211;

            satrec.d3210 = dsinit_result.d3210;
            satrec.d3222 = dsinit_result.d3222;
            satrec.d4410 = dsinit_result.d4410;
            satrec.d4422 = dsinit_result.d4422;
            satrec.d5220 = dsinit_result.d5220;

            satrec.d5232 = dsinit_result.d5232;
            satrec.d5421 = dsinit_result.d5421;
            satrec.d5433 = dsinit_result.d5433;
            satrec.dedt = dsinit_result.dedt;
            satrec.didt = dsinit_result.didt;

            satrec.dmdt = dsinit_result.dmdt;
            satrec.dnodt = dsinit_result.dnodt;
            satrec.domdt = dsinit_result.domdt;
            satrec.del1 = dsinit_result.del1;

            satrec.del2 = dsinit_result.del2;
            satrec.del3 = dsinit_result.del3;
            satrec.xfact = dsinit_result.xfact;
            satrec.xlamo = dsinit_result.xlamo;
            satrec.xli = dsinit_result.xli;

            satrec.xni = dsinit_result.xni;
        }

        // ----------- set variables if not deep space -----------
        if satrec.isimp != 1 {
            cc1sq = satrec.cc1 * satrec.cc1;
            satrec.d2 = 4.0 * ao * tsi * cc1sq;
            temp = (satrec.d2 * tsi * satrec.cc1) / 3.0;
            satrec.d3 = ((17.0 * ao) + sfour) * temp;
            satrec.d4 = 0.5 * temp * ao * tsi * ((221.0 * ao) + (31.0 * sfour)) * satrec.cc1;
            satrec.t3cof = satrec.d2 + (2.0 * cc1sq);
            satrec.t4cof =
                0.25 * ((3.0 * satrec.d3) + (satrec.cc1 * ((12.0 * satrec.d2) + (10.0 * cc1sq))));
            satrec.t5cof = 0.2
                * ((3.0 * satrec.d4)
                    + (12.0 * satrec.cc1 * satrec.d3)
                    + (6.0 * satrec.d2 * satrec.d2)
                    + (15.0 * cc1sq * ((2.0 * satrec.d2) + cc1sq)));
        }

        /* finally propogate to zero epoch to initialize all others. */
        // sgp4fix take out check to let satellites process until they are actually below earth surface
        // if(satrec.error == 0)
    }

    let _ = sgp4(satrec, 0.0);

    satrec.init = DpperInit::N;
}
