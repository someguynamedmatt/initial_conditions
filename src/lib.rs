//! initial_conditions
//!
//! A simple physics library

extern crate num;

mod consts;
pub use consts::{
    LIGHT_SPEED,
    G,
    LITTLE_G,
    ELECTRON_MASS,
    PROTON_MASS,
    ELECTRON_CHARGE,
    PLANCKS_CONST,
    EARTH_MASS,
    EARTH_RADIUS,
    EARTH_RADIUS_ORBIT,
    EARTH_PERIOD_ROT,
    EARTH_PERIOD_REV,
    MOON_MASS,
    MOON_RAD,
    MOON_ORBIT_RAD,
    SUN_MASS,
    ANGSTROM,
    AMU_KG,
    AMU_MEV
};

pub mod kinematics;

mod utils;
