extern crate num;

use num::Float;

/// Returns work in Newton-meters. Work, here, is calculated assuming a constant Force (f)
/// over an angle (theta) across a distance (d).
///
/// # Args
/// * `f`     - force in newtons
/// * `theta` - angle in radians
/// * `d`     - distance in meters
///
/// # Notes:
///   See [here](https://en.wikipedia.org/wiki/Work_(physics)#Mathematical_calculation) for more
///   info
pub fn work<T>(f: &T, theta: &T, d: &T) -> T
    where T: Float
{
    (*f * T::cos(*theta)) * *d
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
    fn test_work() {
        let dist = 10.0;
        let force = 12.0;
        let theta = consts::FRAC_PI_4;
        let work = work(&force, &theta, &dist);

        let diff = abs(work - 84.8528137424);
        assert!(diff <= EPSILON);
    }

    #[test]
    fn test_work2() {
        let dist = 10.0;
        let force = 12.0;
        let theta = num::zero();
        let work = work(&force, &theta, &dist);

        assert_eq!(work, 120.0);
    }
}
