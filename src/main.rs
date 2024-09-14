use satellite;

pub fn main(){
    let tle_line1 = "1 44714C 19074B   24257.74770833  .00012054  00000+0  80755-3 0  2576";
    let tle_line2 = "2 44714  53.0541  99.4927 0001373  86.0479  80.2511 15.06391223    18";
    let mut satrec = satellite::twoline2satrec(tle_line1, tle_line2);
    let _position_and_velocity = satellite::propagate(&mut satrec, 2024.0, 9.0, 22.0, 12.0, 12.0, 12.0, 0.0);
}