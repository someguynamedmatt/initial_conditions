mod consts;

pub mod _const {
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
}

#[cfg(test)]
mod tests {
    use super::consts as _const;
    #[test]
    fn it_works() {
        assert_eq!(_const::AMU_MEV, 931.0 as f64);
    }
}
