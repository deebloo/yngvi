use super::Length;
use std::ops::Neg;

impl Neg for Length {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            nanometers: -self.nanometers,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sub_same_unit() {
        let l1 = Length::from_m(3.0);
        let l2 = Length::from_m(1.0);
        let diff = l1 - l2;
        assert_eq!(diff.as_m(), 2.0);
    }

    #[test]
    fn test_sub_different_units() {
        let l1 = Length::from_m(2000.0);
        let l2 = Length::from_mi(1.0);
        let diff = l1 - l2;
        assert_eq!(diff.as_m(), 390.656);
    }

    #[test]
    fn test_sub_anticommutative() {
        let l1 = Length::from_m(1000.0);
        let l2 = Length::from_mi(1.0);
        let diff1 = l1 - l2;
        let diff2 = l2 - l1;
        assert_eq!(diff1, -diff2);
    }

    #[test]
    fn test_sub_zero() {
        let l1 = Length::from_m(1.0);
        let l2 = Length::from_m(0.0);
        let diff = l1 - l2;
        assert_eq!(diff, l1);
    }
}
