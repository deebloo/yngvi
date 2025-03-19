use super::Length;
use std::ops::Add;

impl Add for Length {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            nanometers: self.nanometers + other.nanometers,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_same_unit() {
        let l1 = Length::from_m(1.0);
        let l2 = Length::from_m(2.0);
        let sum = l1 + l2;
        assert_eq!(sum.as_m(), 3.0);
    }

    #[test]
    fn test_add_different_units() {
        let l1 = Length::from_m(1000.0);
        let l2 = Length::from_mi(1.0);
        let sum = l1 + l2;
        assert_eq!(sum.as_m(), 2609.344);
    }

    #[test]
    fn test_add_commutative() {
        let l1 = Length::from_m(1000.0);
        let l2 = Length::from_mi(1.0);
        let sum1 = l1 + l2;
        let sum2 = l2 + l1;
        assert_eq!(sum1, sum2);
    }

    #[test]
    fn test_add_associative() {
        let l1 = Length::from_m(1000.0);
        let l2 = Length::from_mi(1.0);
        let l3 = Length::from_yd(100.0);
        let sum1 = (l1 + l2) + l3;
        let sum2 = l1 + (l2 + l3);
        assert_eq!(sum1, sum2);
    }
}
