extern crate num;
#[allow(unused_imports)] use num::Float;
use utils::vec_traits::{ScalarMult,
                        ScalarDiv,
                        AddVec,
                        SubVec};

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
pub fn second_position<T>(x_i: Vec<T>, mut v_i: Vec<T>, time: T, mut acc: Vec<T>) -> Vec<T>
    where T: Float
{
    assert!(time >= T::zero(), "Time cannot be negative");

    v_i.scalar_mult(time);
    let t_sqrd = time * time;
    acc.scalar_mult(t_sqrd);
    acc.scalar_div(num::cast(2.0).unwrap());

    x_i.add_vec(v_i).add_vec(acc)
}

#[cfg(test)]
mod tests {
    extern crate num;

    use super::*;
    #[allow(unused_imports)] use num::Float;

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
    fn test_second_position1() {
        let v_i = vec![1.0, 1.0, 1.0];
        let x_i = vec![0.0, 0.0, 0.0];
        let t = 10.0;
        let a = vec![2.0, 2.0, 2.0];
        let x = second_position(x_i, v_i, t, a);

        let expected = vec![110.0, 110.0, 110.0];
        assert_eq!(x, expected);
    }

    #[test]
    #[should_panic]
    fn test_second_position2() {
        let v_i = vec![0.0, 0.0, 0.0];
        let x_i = vec![0.0, 0.0, 0.0];
        let t = -30.0;
        let a = vec![0.0, 0.0, -9.8];
        let x = second_position(x_i, v_i, t, a);

        let expected = vec![0.0, 0.0, -4410.0];
        assert_eq!(x, expected);
    }
}
