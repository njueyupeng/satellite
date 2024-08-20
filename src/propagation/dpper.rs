use crate::constants::{PI, TWO_PI};
use crate::types::Satrec;

pub struct DpperOption {
    init: char,
    opsmode: char,
    ep: f64,
    inclp: f64,
    nodep: f64,
    argpp: f64,
    mp: f64,
}

pub struct DpperResult {
    ep: f64,
    inclp: f64,
    nodep: f64,
    argpp: f64,
    mp: f64,
}

// -----------------------------------------------------------------------------
//
//                            procedure dpper
//
//   this procedure provides deep space long period periodic contributions
//     to the mean elements.  by design, these periodics are zero at epoch.
//     this used to be dscom which included initialization, but it's really a
//     recurring function.
//
//   author        : david vallado                  719-573-2600   28 jun 2005
//
//   inputs        :
//     e3          -
//     ee2         -
//     peo         -
//     pgho        -
//     pho         -
//     pinco       -
//     plo         -
//     se2 , se3 , sgh2, sgh3, sgh4, sh2, sh3, si2, si3, sl2, sl3, sl4 -
//     t           -
//     xh2, xh3, xi2, xi3, xl2, xl3, xl4 -
//     zmol        -
//     zmos        -
//     ep          - eccentricity                           0.0 - 1.0
//     inclo       - inclination - needed for lyddane modification
//     nodep       - right ascension of ascending node
//     argpp       - argument of perigee
//     mp          - mean anomaly
//
//   outputs       :
//     ep          - eccentricity                           0.0 - 1.0
//     inclp       - inclination
//     nodep        - right ascension of ascending node
//     argpp       - argument of perigee
//     mp          - mean anomaly
//
//   locals        :
//     alfdp       -
//     betdp       -
//     cosip  , sinip  , cosop  , sinop  ,
//     dalf        -
//     dbet        -
//     dls         -
//     f2, f3      -
//     pe          -
//     pgh         -
//     ph          -
//     pinc        -
//     pl          -
//     sel   , ses   , sghl  , sghs  , shl   , shs   , sil   , sinzf , sis   ,
//     sll   , sls
//     xls         -
//     xnoh        -
//     zf          -
//     zm          -
//
//   coupling      :
//     none.
//
//   references    :
//     hoots, roehrich, norad spacetrack report #3 1980
//     hoots, norad spacetrack report #6 1986
//     hoots, schumacher and glover 2004
//     vallado, crawford, hujsak, kelso  2006
// ----------------------------------------------------------------------------

pub fn deper(satrec: &Satrec, options: &DpperOption) -> DpperResult {
    let e3 = satrec.e3;
    let ee2 = satrec.ee2;
    let peo = satrec.peo;
    let pgho = satrec.pgho;
    let pho = satrec.pho;
    let pinco = satrec.pinco;
    let plo = satrec.plo;
    let se2 = satrec.se2;
    let se3 = satrec.se3;
    let sgh2 = satrec.sgh2;
    let sgh3 = satrec.sgh3;
    let sgh4 = satrec.sgh4;
    let sh2 = satrec.sh2;
    let sh3 = satrec.sh3;
    let si2 = satrec.si2;
    let si3 = satrec.si3;
    let sl2 = satrec.sl2;
    let sl3 = satrec.sl3;
    let sl4 = satrec.sl4;
    let t = satrec.t;
    let xgh2 = satrec.xgh2;
    let xgh3 = satrec.xgh3;
    let xgh4 = satrec.xgh4;
    let xh2 = satrec.xh2;
    let xh3 = satrec.xh3;
    let xi2 = satrec.xi2;
    let xi3 = satrec.xi3;
    let xl2 = satrec.xl2;
    let xl3 = satrec.xl3;
    let xl4 = satrec.xl4;
    let zmol = satrec.zmol;
    let zmos = satrec.zmos;

    let init = options.init;
    let opsmode = options.opsmode;
    let mut ep = options.ep;
    let mut inclp = options.inclp;
    let mut nodep = options.nodep;
    let mut argpp = options.argpp;
    let mut mp = options.mp;

    // Copy satellite attributes into local variables for convenience
    // and symmetry in writing formulae.

    let mut alfdp;
    let mut betdp;
    let cosip;
    let sinip;
    let cosop;
    let sinop;
    let dalf;
    let dbet;
    let dls;
    let mut f2;
    let mut f3;
    let mut pe;
    let mut pgh;
    let mut ph;
    let mut pinc;
    let mut pl;
    let mut sinzf;
    let mut xls;
    let xnoh;
    let mut zf;
    let mut zm;

    //  ---------------------- constants -----------------------------
    const ZNS: f64 = 1.19459e-5;
    const ZES: f64 = 0.01675;
    const ZNL: f64 = 1.5835218e-4;
    const ZEL: f64 = 0.05490;

    //  --------------- calculate time varying periodics -----------
    zm = zmos + (ZNS * t);

    if init == 'y' {
        zm = zmos;
    }
    zf = zm + (2.0 * ZES * zm.sin());
    sinzf = zf.sin();
    f2 = (0.5 * sinzf * sinzf) - 0.25;
    f3 = -0.5 * sinzf * zf.cos();

    let ses = (se2 * f2) + (se3 * f3);
    let sis = (si2 * f2) + (si3 * f3);
    let sls = (sl2 * f2) + (sl3 * f3) + (sl4 * sinzf);
    let sghs = (sgh2 * f2) + (sgh3 * f3) + (sgh4 * sinzf);
    let shs = (sh2 * f2) + (sh3 * f3);

    zm = zmol + (ZNL * t);
    if init == 'y' {
        zm = zmol;
    }

    zf = zm + (2.0 * ZEL * zm.sin());
    sinzf = zf.sin();
    f2 = (0.5 * sinzf * sinzf) - 0.25;
    f3 = -0.5 * sinzf * zf.cos();

    let sel = (ee2 * f2) + (e3 * f3);
    let sil = (xi2 * f2) + (xi3 * f3);
    let sll = (xl2 * f2) + (xl3 * f3) + (xl4 * sinzf);
    let sghl = (xgh2 * f2) + (xgh3 * f3) + (xgh4 * sinzf);
    let shll = (xh2 * f2) + (xh3 * f3);

    pe = ses + sel;
    pinc = sis + sil;
    pl = sls + sll;
    pgh = sghs + sghl;
    ph = shs + shll;

    if init == 'n' {
        pe -= peo;
        pinc -= pinco;
        pl -= plo;
        pgh -= pgho;
        ph -= pho;
        inclp += pinc;
        ep += pe;
        sinip = inclp.sin();
        cosip = inclp.cos();
        /* ----------------- apply periodics directly ------------ */
        // sgp4fix for lyddane choice
        // strn3 used original inclination - this is technically feasible
        // gsfc used perturbed inclination - also technically feasible
        // probably best to readjust the 0.2 limit value and limit discontinuity
        // 0.2 rad = 11.45916 deg
        // use next line for original strn3 approach and original inclination
        // if (inclo >= 0.2)
        // use next line for gsfc version and perturbed inclination
        if inclp >= 0.2 {
            ph /= sinip;
            pgh -= cosip * ph;
            argpp += pgh;
            nodep += ph;
            mp += pl;
        } else {
            //  ---- apply periodics with lyddane modification ----
            sinop = nodep.sin();
            cosop = nodep.cos();
            alfdp = sinip * sinop;
            betdp = sinip * cosop;
            dalf = (ph * cosop) + (pinc * cosip * sinop);
            dbet = (-ph * sinop) + (pinc * cosip * cosop);
            alfdp += dalf;
            betdp += dbet;
            nodep %= TWO_PI;

            //  sgp4fix for afspc written intrinsic functions
            //  nodep used without a trigonometric function ahead
            if nodep < 0.0 && opsmode == 'a' {
                nodep += TWO_PI;
            }
            xls = mp + argpp + (cosip * nodep);
            dls = (pl + pgh) - (pinc * nodep * sinip);
            xls += dls;
            xnoh = nodep;
            nodep = alfdp.atan2(betdp);

            //  sgp4fix for afspc written intrinsic functions
            //  nodep used without a trigonometric function ahead
            if nodep < 0.0 && opsmode == 'a' {
                nodep += TWO_PI;
            }
            if (xnoh - nodep).abs() > PI {
                if nodep < xnoh {
                    nodep += TWO_PI;
                } else {
                    nodep -= TWO_PI;
                }
            }
            mp += pl;
            argpp = xls - mp - (cosip * nodep);
        }
    }

    DpperResult {
        ep,
        inclp,
        nodep,
        argpp,
        mp,
    }
}
