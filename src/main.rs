use satellite;
fn main() {
    println!("Hello, satellite!");
    // Sample TLE
    let tle_line1 = "1 25544U 98067A   19156.50900463  .00003075  00000-0  59442-4 0  9992";
    let tle_line2 = "2 25544  51.6433  59.2583 0008217  16.4489 347.6017 15.51174618173442";
    let s = satellite::twoline2satrec(&tle_line1, &tle_line2);
    println!("{:#?}", s);
}
