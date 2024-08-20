use std::option;

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
    pub irez: f64,
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
pub struct DsInitResult {}

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
    let mo = option.mo;
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
}
