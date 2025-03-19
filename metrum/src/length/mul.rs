use super::Length;
use std::ops::Mul;

impl Mul<f64> for Length {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Self {
            nanometers: self.nanometers * scalar,
        }
    }
}

impl Mul<Length> for f64 {
    type Output = Length;

    fn mul(self, length: Length) -> Length {
        Length {
            nanometers: self * length.nanometers,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mul_scalar() {
        let l1 = Length::from_m(2.0);
        let product = l1 * 3.0;
        assert_eq!(product.as_m(), 6.0);
    }

    #[test]
    fn test_mul_commutative() {
        let l1 = Length::from_m(2.0);
        let scalar = 3.0;
        let product1 = l1 * scalar;
        let product2 = scalar * l1;
        assert_eq!(product1, product2);
    }

    #[test]
    fn test_mul_associative() {
        let l1 = Length::from_m(2.0);
        let scalar1 = 3.0;
        let scalar2 = 4.0;
        let product1 = (l1 * scalar1) * scalar2;
        let product2 = l1 * (scalar1 * scalar2);
        assert_eq!(product1, product2);
    }

    #[test]
    fn test_mul_one() {
        let l1 = Length::from_m(2.0);
        let product = l1 * 1.0;
        assert_eq!(product, l1);
    }

    #[test]
    fn test_mul_zero() {
        let l1 = Length::from_m(2.0);
        let product = l1 * 0.0;
        assert_eq!(product.as_m(), 0.0);
    }

    #[test]
    fn test_mul_different_units() {
        let l1 = Length::from_mi(1.0);
        let product = l1 * 2.0;
        assert_eq!(product.as_m(), 3218.688);
    }
}
