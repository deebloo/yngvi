use serde::{Deserialize, Serialize};
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum Temp {
    C(f32),
    F(f32),
}

impl Temp {
    pub fn to_f(&self) -> Self {
        match self {
            Temp::C(temp) => Temp::F((temp * 9.) / 5. + 32.),
            Temp::F(val) => Temp::F(*val),
        }
    }

    pub fn to_c(&self) -> Self {
        match self {
            Temp::C(val) => Temp::C(*val),
            Temp::F(val) => Temp::C((val - 32.) * (5. / 9.)),
        }
    }
}

impl Into<f32> for Temp {
    fn into(self) -> f32 {
        match self {
            Self::C(val) => val,
            Self::F(val) => val,
        }
    }
}

impl Sub for Temp {
    type Output = Temp;

    fn sub(self, rhs: Self) -> Self::Output {
        let target: f32 = rhs.into();

        match self {
            Self::C(val) => Self::C(val - target),
            Self::F(val) => Self::F(val - target),
        }
    }
}

impl Add for Temp {
    type Output = Temp;

    fn add(self, rhs: Self) -> Self::Output {
        let target: f32 = rhs.into();

        match self {
            Self::C(val) => Self::C(val + target),
            Self::F(val) => Self::F(val + target),
        }
    }
}

impl Mul for Temp {
    type Output = Temp;

    fn mul(self, rhs: Self) -> Self::Output {
        let target: f32 = rhs.into();

        match self {
            Self::C(val) => Self::C(val * target),
            Self::F(val) => Self::F(val * target),
        }
    }
}

impl Div for Temp {
    type Output = Temp;

    fn div(self, rhs: Self) -> Self::Output {
        let target: f32 = rhs.into();

        match self {
            Self::C(val) => Self::C(val / target),
            Self::F(val) => Self::F(val / target),
        }
    }
}

impl PartialOrd for Temp {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let source: f32 = (*self).into();
        let target: f32 = (*other).into();

        let mut res = std::cmp::Ordering::Equal;

        if source > target {
            res = std::cmp::Ordering::Greater;
        } else {
            res = std::cmp::Ordering::Less;
        }

        Some(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_subtract_two_temps() {
        let res = Temp::F(100.) - Temp::F(100.);

        assert_eq!(res, Temp::F(0.));
    }

    #[test]
    fn should_add_two_temps() {
        let res = Temp::F(100.) + Temp::F(100.);

        assert_eq!(res, Temp::F(200.));
    }

    #[test]
    fn should_multiply_two_temps() {
        let res = Temp::F(10.) * Temp::F(2.);

        assert_eq!(res, Temp::F(20.));
    }

    #[test]
    fn should_divide_two_temps() {
        let res = Temp::F(10.) / Temp::F(2.);

        assert_eq!(res, Temp::F(5.));
    }
}
