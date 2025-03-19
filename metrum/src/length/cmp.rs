use super::Length;

impl PartialEq for Length {
    fn eq(&self, other: &Self) -> bool {
        (self.nanometers - other.nanometers).abs() < f64::EPSILON
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equality_1() {
        let l1 = Length::from_m(1.0);
        let l2 = Length::from_km(0.001);
        let l3 = Length::from_mm(1000.0);
        let l4 = Length::from_um(1_000_000.0);
        let l5 = Length::from_nm(1_000_000_000.0);
        let l6 = Length::from_in(39.37007874015748);

        assert_eq!(l1, l2);
        assert_eq!(l1, l3);
        assert_eq!(l1, l4);
        assert_eq!(l1, l5);
        assert_eq!(l1, l6);
    }

    #[test]
    fn test_equality_2() {
        let l1 = Length::from_m(1.0);
        let l2 = Length::from_km(0.001);
        let l3 = Length::from_mm(1000.0);
        let l4 = Length::from_um(1_000_000.0);
        let l5 = Length::from_nm(1_000_000_000.0);
        let l6 = Length::from_in(39.37007874015748);

        assert_eq!(l2, l1);
        assert_eq!(l2, l3);
        assert_eq!(l2, l4);
        assert_eq!(l2, l5);
        assert_eq!(l2, l6);
    }

    #[test]
    fn test_equality_3() {
        let l1 = Length::from_m(1.0);
        let l2 = Length::from_km(0.001);
        let l3 = Length::from_mm(1000.0);
        let l4 = Length::from_um(1_000_000.0);
        let l5 = Length::from_nm(1_000_000_000.0);
        let l6 = Length::from_in(39.37007874015748);

        assert_eq!(l3, l1);
        assert_eq!(l3, l2);
        assert_eq!(l3, l4);
        assert_eq!(l3, l5);
        assert_eq!(l3, l6);
    }

    #[test]
    fn test_equality_4() {
        let l1 = Length::from_m(1.0);
        let l2 = Length::from_km(0.001);
        let l3 = Length::from_mm(1000.0);
        let l4 = Length::from_um(1_000_000.0);
        let l5 = Length::from_nm(1_000_000_000.0);
        let l6 = Length::from_in(39.37007874015748);

        assert_eq!(l4, l1);
        assert_eq!(l4, l2);
        assert_eq!(l4, l3);
        assert_eq!(l4, l5);
        assert_eq!(l4, l6);

        assert_eq!(l6, l5);
    }

    #[test]
    fn test_equality_5() {
        let l1 = Length::from_m(1.0);
        let l2 = Length::from_km(0.001);
        let l3 = Length::from_mm(1000.0);
        let l4 = Length::from_um(1_000_000.0);
        let l5 = Length::from_nm(1_000_000_000.0);
        let l6 = Length::from_in(39.37007874015748);

        assert_eq!(l5, l1);
        assert_eq!(l5, l2);
        assert_eq!(l5, l3);
        assert_eq!(l5, l4);
        assert_eq!(l5, l6);
    }

    #[test]
    fn test_equality_6() {
        let l1 = Length::from_m(1.0);
        let l2 = Length::from_km(0.001);
        let l3 = Length::from_mm(1000.0);
        let l4 = Length::from_um(1_000_000.0);
        let l5 = Length::from_nm(1_000_000_000.0);
        let l6 = Length::from_in(39.37007874015748);

        assert_eq!(l6, l1);
        assert_eq!(l6, l2);
        assert_eq!(l6, l3);
        assert_eq!(l6, l4);
        assert_eq!(l6, l5);
    }
}
