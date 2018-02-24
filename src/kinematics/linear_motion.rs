extern crate num;

use num::Float;

/// ## First Equation of Motion:
///
/// Returns the final velocity (v) assuming a constant
/// acceleration (a) during a time (t).
///
/// # Args
/// * `v_0`: `f64`   - Initial velocity (m * s)
/// * `a`: `f64`     - constant acceleration (m * s^(2))
/// * `t`: `f64`     - time (s)
///
/// # Notes:
///   - This derivation can be [understood here](https://en.wikipedia.org/wiki/Equations_of_motion#Uniform_acceleration)
///   - This only holds under *constant acceleration*
pub fn first_velocity<T>(v_0: T, a: T, t: T) -> T
    where T: Float
{
    assert!(t >= T::zero(), "time cannot be negative");
    v_0 + (a * t)
}

/// ## First Equation of Motion:
///
/// Returns the acceleration (a) given initial and final velocities (v_i and v_f, respectively),
/// over a time (t)
///
/// # Args
/// * `v_i`: `f64`   - Initial velocity (m * s)
/// * `v_f`: `f64`   - Final velocity (m * s)
/// * `t`: `f64`     - time (s)
///
/// # Notes:
///   - This derivation can be [understood here](https://en.wikipedia.org/wiki/Equations_of_motion#Uniform_acceleration)
pub fn first_accel<T>(v_f: T, v_i: T, t: T) -> T
    where T: Float
{
    assert!(t >= T::zero(), "time cannot be negative");
    (v_f - v_i) / t
}

/// ## Second Equation of Motion:
///
/// Returns the final position (x) given the initial velocity (v_i),
/// the total time (t), and the acceleration (a).
///
/// # Args
/// * `v_i`: `f64`   - Initial velocity (m * s)
/// * `x_i`: `f64`   - Initial position (m)
/// * `t`: `f64`     - time (s)
/// * `a`: `f64`     - acceleration (m * s^(2))
///
/// # Notes:
///   - This derivation can be [understood here](https://en.wikipedia.org/wiki/Equations_of_motion#Uniform_acceleration)
pub fn second_position<T>(x_i: T, v_i: T, t: T, a: T) -> T
    where T: Float
{
    x_i + (v_i * t) + ((a * (t * t)) / num::cast(2.0).unwrap())
}

#[cfg(test)]
mod tests {
    extern crate num;

    use super::*;
    use num::Float;
    use num::abs;

    const EPSILON: f64 = 1e-10;

    #[test]
    fn test_first_velocity() {
        let v_0 = num::zero();
        let a = 9.8;
        let t = 20.0;
        let v = first_velocity(v_0, a, t);

        assert_eq!(v, 196.0);
    }

    #[test]
    fn test_first_velocity2() {
        let v_0 = 200.0;
        let a = num::zero();
        let t = 1200.0;
        let v = first_velocity(v_0, a, t);

        assert_eq!(v, 200.0);
    }

    #[test]
    fn test_second_position() {
        let v_i = num::zero();
        let x_i = num::zero();
        let t = 10.0;
        let a = 9.8;
        let x = second_position(x_i, v_i, t, a);

        let diff = abs(x - 490.0);
        assert!(diff <= EPSILON);
    }
}