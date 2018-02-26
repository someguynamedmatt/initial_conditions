extern crate num;
#[allow(unused_imports)] use num::Float;
use utils::{VecTraits};

/// ## First Equation of Motion:
///
/// Returns the final velocity (v) assuming a constant
/// acceleration (a) during a time (t).
///
/// # Args
/// * `v_i`: `Vec<f64>`   - Initial velocity (m * s)
/// * `a`: `mut Vec<f64>`     - constant acceleration (m * s^(2))
/// * `t`: `f64`     - time (s)
///
/// # Notes:
///   - This derivation can be [understood here](https://en.wikipedia.org/wiki/Equations_of_motion#Uniform_acceleration)
///   - This only holds under *constant acceleration*
pub fn first_velocity<T>(mut acc: Vec<T>, v_i: Vec<T>, time: T) -> Vec<T>
    where T: Float
{
    assert!(time >= T::zero(), "time cannot be negative");
    acc.scalar_mult(time);
    v_i.add_vec(acc)
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
pub fn first_accel<T>(v_f: Vec<T>, v_i: Vec<T>, time: T) -> Vec<T>
    where T: Float
{
    assert!(time >= T::zero(), "time cannot be negative");
    let mut a = v_f.sub_vec(v_i);
    a.scalar_div(time);
    a
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

pub fn dots<T>(v1: Vec<T>, v2: Vec<T>) -> T
    where T: Float
{
    v1.dot(v2)
}

#[cfg(test)]
mod tests {
    extern crate num;

    use super::*;
    #[allow(unused_imports)] use num::Float;
    use num::abs;

    const EPSILON: f64 = 1e-10;

    #[test]
    fn first_velocity_1() {
        let a = vec![1.0, 2.0, 3.0];
        let v_i = vec![1.0, 1.0, 1.0];
        let t = 10.0;
        let v = first_velocity(a, v_i, t);

        let expected = vec![11.0, 21.0, 31.0];
        assert_eq!(v, expected);
    }

    #[test]
    fn first_velocity_2() {
        let v_i = vec![200.0, 200.0, 200.0];
        let a = vec![0.0; 3];
        let t = 1200.0;
        let v = first_velocity(a, v_i, t);

        let expected = vec![200.0, 200.0, 200.0];
        assert_eq!(v, expected);
    }

    #[test]
    fn first_accel_1() {
        let v_i = vec![1.0, 1.0, 1.0];
        let v_f = vec![1.0, 1.0, 1.0];
        let t = 10.0;
        let a = first_accel(v_f, v_i, t);

        let expected = vec![0.0, 0.0, 0.0];
        assert_eq!(a, expected);
    }

    #[test]
    fn first_accel_2() {
        let v_i = vec![1.0, 1.0, 1.0];
        let v_f = vec![1.0, 1.0, 1.0];
        let t = 0.0;
        let a = first_accel(v_f, v_i, t);

        let expected = vec![Float::infinity(), Float::infinity(), Float::infinity()];
        assert_eq!(a, expected);
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
