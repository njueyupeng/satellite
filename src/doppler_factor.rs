use crate::EciVec3;
use wasm_bindgen::prelude::*;
fn sign(value: f64) -> f64 {
    if value >= 0.0 {
        1.0
    } else {
        -1.0
    }
}
// 计算多普勒因子
#[wasm_bindgen(js_name = "dopplerFactor")]
pub fn doppler_factor(location: &EciVec3, position: &EciVec3, velocity: &EciVec3) -> f64 {
    const M_FACTOR: f64 = 7.292115E-5; // 地球自转角速度
    const LIGHT_SPEED: f64 = 299792.458; // 光速 km/s

    let range = EciVec3 {
        x: position.x - location.x,
        y: position.y - location.y,
        z: position.z - location.z,
    };
    let range_w = (range.x.powi(2) + range.y.powi(2) + range.z.powi(2)).sqrt();
    let range_vel = EciVec3 {
        x: velocity.x + M_FACTOR * location.y,
        y: velocity.y - M_FACTOR * location.x,
        z: velocity.z,
    };
    let range_rate =
        (range.x * range_vel.x + range.y * range_vel.y + range.z * range_vel.z) / range_w;

    return (1.0 + (range_rate / LIGHT_SPEED)) * sign(range_rate);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::EARTH_RADIUS;

    #[test]
    fn test_doppler_factor_without_observer_movement() {
        let observer_ecf = EciVec3 {
            x: 0.0,
            y: 0.0,
            z: EARTH_RADIUS,
        };
        let position_ecf = EciVec3 {
            x: 0.0,
            y: 0.0,
            z: EARTH_RADIUS + 500.0,
        };
        let velocity_ecf = EciVec3 {
            x: 7.91,
            y: 0.0,
            z: 0.0,
        };

        let dop_factor = doppler_factor(&observer_ecf, &position_ecf, &velocity_ecf);

        // Define the number of decimal places for the comparison
        let tolerance = 1e-6; // Example tolerance value
        assert!(
            (dop_factor - 1.0).abs() < tolerance,
            "Doppler factor is not close to 1.0"
        );
    }
}
