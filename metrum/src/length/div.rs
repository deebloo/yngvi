use super::Length;
use std::ops::Div;

impl Div<f64> for Length {
    type Output = Self;

    fn div(self, scalar: f64) -> Self {
        Self {
            nanometers: self.nanometers / scalar,
        }
    }
}

impl Div<Length> for Length {
    type Output = f64;

    fn div(self, other: Self) -> f64 {
        self.nanometers / other.nanometers
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_div_scalar() {
        let l1 = Length::from_m(6.0);
        let quotient = l1 / 3.0;
        assert_eq!(quotient.as_m(), 2.0);
    }

    #[test]
    fn test_div_length() {
        let l1 = Length::from_m(6.0);
        let l2 = Length::from_m(2.0);
        let ratio = l1 / l2;
        assert_eq!(ratio, 3.0);
    }

    #[test]
    fn test_div_different_units() {
        let l1 = Length::from_m(1609.344);
        let l2 = Length::from_mi(1.0);
        let ratio = l1 / l2;
        assert_eq!(ratio, 1.0);
    }

    #[test]
    fn test_div_one() {
        let l1 = Length::from_m(2.0);
        let quotient = l1 / 1.0;
        assert_eq!(quotient, l1);
    }

    #[test]
    fn test_div_self() {
        let l1 = Length::from_m(2.0);
        let ratio = l1 / l1;
        assert_eq!(ratio, 1.0);
    }
}
