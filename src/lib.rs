mod consts;

#[cfg(test)]
mod tests {
    use super::consts as _const;
    #[test]
    fn it_works() {
        assert_eq!(_const::AMU_MEV, 931.0 as f64);
    }
}
