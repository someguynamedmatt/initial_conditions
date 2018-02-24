extern crate num;

use std::f64::consts;
use num::Float;

/// ## Uniform circular motion
///
/// Returns the average speed in (m / s), given the
/// rotational period and the circular radius (r).
///
/// # Args
/// * `period`: `f64`   - rotational period (s)
/// * `r`: `f64`   - radius (m)
///
/// # Notes:
/// - This assumes a circular path
pub fn average_speed<T>(period: T, r: T) -> T
    where T: Float
{
    assert!(r >= 0, "Radius cannot be negative");
    assert!(period >= 0, "Period cannot be negative");

    let two = num::cast::<f64, T>(2.0).unwrap();
    let pi = num::cast::<f64, T>(consts::PI).unwrap();

    (two * pi) * r / period
}

/// ## Uniform circular motion
///
/// Returns the circular acceleration (m / s^(2)) given the velocity (v)
/// and radius (r)
///
/// # Args
/// * `v`: `f64`   - velocity (m / s)
/// * `r`: `f64`   - radius (m)
///
/// # Notes:
///   - This assumes a circular path
pub fn acceleration<T>(v: T, r: T) -> T
    where T: Float
{
    assert!(r >= 0, "Radius cannot be negative");

    (v * v) / num::cast(r).unwrap()
}

#[cfg(test)]
mod tests {
    extern crate num;

    use super::*;
    use std::f64::consts;
    use num::abs;
    use num::Float;

    const EPSILON: f64 = 1e-10;

    #[test]
    fn test_average_speed() {
        let period: f64 = 2.0;
        let radius: f64 = 5.0;
        let avg_speed = average_speed(period, radius);

        let diff = abs(avg_speed - 15.70796326794);
        assert!(diff <= EPSILON);
    }

    #[test]
    fn test_acceleration() {
        let v = 4.0;
        let r = 2.0;
        let a = acceleration(v, r);

        assert_eq!(a, 8.0);
    }
}

