use satellite::propagation::initl;
use satellite::types::DpperOpsMode;

fn is_close(actual: f64, ed: f64, epsilon: f64) -> bool {
    (actual - ed).abs() < epsilon
}

#[test]
pub fn legacy_sidereal_time_calculations() {
    const OPTIONS: initl::InitOptions = initl::InitOptions {
        ecco: 0.1846988,
        epoch: 25938.538312919904,
        inclo: 0.0,
        no: 0.0037028783237264057,
        opsmode: DpperOpsMode::A,
    };
    let results = initl::initl(OPTIONS);
    let epsilon = 1e-3;

    assert!(is_close(results.ainv, 0.1353414893496189, epsilon));
    assert!(is_close(results.ao, 7.3887172721793, epsilon));
    assert_eq!(results.con41, 2.0);
    assert_eq!(results.con42, -4.0);
    assert_eq!(results.cosio, 1.0);
    assert_eq!(results.cosio2, 1.0);
    assert!(is_close(results.eccsq, 0.034113646721439995, epsilon));
    assert!(is_close(results.gsto, 5.220883431398299, epsilon));
    assert_eq!(results.method, initl::InitlMethod::N);
    assert!(is_close(results.no, 0.003702762286531528, epsilon));
    assert!(is_close(results.omeosq, 0.96588635327856, epsilon));
    assert!(is_close(results.posq, 50.931932818552305, epsilon));
    assert!(is_close(results.rp, 6.02403005846851, epsilon));
    assert!(is_close(results.rteosq, 0.9827951736137902, epsilon));
    assert_eq!(results.sinio, 0.0);
}
