use super::Temp;
use std::ops::Sub;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct TempDelta {
    value: f64,
}

impl TempDelta {
    pub fn as_f(&self) -> f64 {
        self.value * (9. / 5.)
    }

    pub fn as_c(&self) -> f64 {
        self.value
    }

    pub fn as_k(&self) -> f64 {
        self.value
    }
}

impl Sub for Temp {
    type Output = TempDelta;

    fn sub(self, rhs: Self) -> Self::Output {
        let k1 = self.as_k();
        let k2 = rhs.as_k();

        TempDelta {
            value: (k1 - k2).abs(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foo() {
        let cc = Temp::from_c(10.) - Temp::from_f(25.);

        assert_eq!(cc.as_f(), 25.000000000000046);
    }

    #[test]
    fn should_sub_from_c() {
        let cc = Temp::from_c(10.) - Temp::from_c(10.);
        let cf = Temp::from_c(10.) - Temp::from_f(50.);
        let ck = Temp::from_c(10.) - Temp::from_k(283.15);

        assert_eq!(cc, TempDelta { value: 0. });
        assert_eq!(cf, TempDelta { value: 0. });
        assert_eq!(ck, TempDelta { value: 0. });
    }

    #[test]
    fn should_sub_from_f() {
        let ff = Temp::from_f(32.) - Temp::from_f(32.);
        let fc = Temp::from_f(32.) - Temp::from_c(0.);
        let fk = Temp::from_f(32.) - Temp::from_k(273.15);

        assert_eq!(ff, TempDelta { value: 0. });
        assert_eq!(fc, TempDelta { value: 0. });
        assert_eq!(fk, TempDelta { value: 0. });
    }

    #[test]
    fn should_sub_from_k() {
        let kk = Temp::from_k(300.) - Temp::from_k(300.);
        let kc = Temp::from_k(300.) - Temp::from_c(26.85);
        let kf = Temp::from_k(300.) - Temp::from_f(80.33);

        assert_eq!(kk, TempDelta { value: 0. });
        assert_eq!(kf, TempDelta { value: 0. });
        assert_eq!(kc, TempDelta { value: 0. });
    }

    #[test]
    fn should_handle_negative_temperatures() {
        let cc = Temp::from_c(-10.) - Temp::from_c(-20.);
        let ff = Temp::from_f(14.) - Temp::from_f(-4.);

        assert_eq!(cc, TempDelta { value: 10. });
        assert_eq!(ff, TempDelta { value: 10. });
    }
}
