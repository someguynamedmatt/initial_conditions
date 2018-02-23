//! Useful Physical and Astronomical constants
extern crate num;

use std::f64::consts;

/// Speed of light in a vaccuum `c`: 3.00 * 10^(8) (m*s^(-1))
pub const c: f64 = 3.00e8_f64;

/// Gravitational constant `G`: 6.67 * 10^(-11) N*(m^(2)*k^(-2))
pub const G: f64 = 6.67e-11_f64;

/// Acceleration of gravity at Earth's surface `g`: 9.8 m * s(-2)
pub const g: f64 = 9.8;

/// Electron mass: 9.11 * 10^(-31) kg
pub const ELECTRON_MASS: f64 = 9.11e-31_f64;

/// Proton mass: 1.67 * 10^(-27) kg
pub const PROTON_MASS: f64 = 1.67e-27_f64;

/// Electron charge: 1.60218 * 10^(-19) C
pub const ELECTRON_CHARGE: f64 1.60218e-19_f64;

/// Planck's constant: 6.63 * 10^(-34) J*s
pub const PLANCKS_CONST: f64 = 6.63e-34_f64;

/// Mass of the Eart: 5.98 * 10^(24) kg
pub const EARTH_MASS: f64 = 5.98e24_f64;

/// Mean radius of the Earth: 6.37 * 10^(6) m
pub const EARTH_RADIUS: f64 = 6.37e6_f64;

/// Radius of Earth's orbit: 1.49 * 10^(11) m
pub const EARTH_RADIUS_ORBIT: f64 = 1.49e11_f64;

/// Period of Earth's rotation: 8.64 * 10^(4) s
pub const EARTH_PERIOD_ROT: f64 = 8.64e4_f64;

/// Period of Earth's revolution: 3.16 * 10^(7) s
pub const EARTH_PERIOD_REV: f64 = 3.16e7_f64;

/// Mass of Earth's Moon: 7.34 * 10^(22) kg
pub const MOON_MASS: f64 = 7.34e22_f64;

/// Radius of Earth's Moon: 1.74 * 10^(6) m
pub const MOON_RAD: f64 = 1.74e6_f64;

/// Radius of Earth's Moon's orbit: 3.84 * 10^(8) m
pub const MOON_ORBIT_RAD: f64 = 3.84e8_f64;

/// Mass of Earth's Sun: 1.99 * 10^(30) kg
pub const SUN_MASS: f64 = 1.99e30_f64;
