extern crate num;

use num::Float;

/// Work as a dot product of a *constant* force (F) applied at an angle
/// (theta) over a distance (d). `W = F . d`
///
///   - force: N
///   - theta: rad
///   - dist:  m
///
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
