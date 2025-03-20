mod cmp;
mod sub;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use sub::TempDelta;

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Temp {
    kelvin: f64,
}

impl Temp {
    pub fn from_c(val: f64) -> Self {
        Self {
            kelvin: val + 273.15,
        }
    }

    pub fn from_f(val: f64) -> Self {
        Self {
            kelvin: (val - 32.) * (5. / 9.) + 273.15,
        }
    }

    pub fn from_k(val: f64) -> Self {
        Self { kelvin: val }
    }

    pub fn from_delta(delta: TempDelta) -> Self {
        Self::from_f(delta.as_f())
    }

    pub fn as_c(&self) -> f64 {
        self.kelvin - 273.15
    }

    pub fn as_f(&self) -> f64 {
        let c = self.as_c();

        c * (9. / 5.) + 32.
    }

    pub fn as_k(&self) -> f64 {
        self.kelvin
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sub::TempDelta;

    use std::vec;

    #[derive(Debug, Serialize, Deserialize)]
    struct Conversion {
        celsius: f64,
        fahrenheit: f64,
        kelvin: f64,
    }

    #[test]
    fn should_convert_correctly() {
        let temps = vec![
            Conversion {
                celsius: 0.,
                fahrenheit: 32.,
                kelvin: 273.15,
            },
            Conversion {
                celsius: 10.,
                fahrenheit: 50.,
                kelvin: 283.15,
            },
        ];

        for temp in temps {
            let f_source = Temp::from_f(temp.fahrenheit);
            let c_source = Temp::from_c(temp.celsius);
            let k_source = Temp::from_k(temp.kelvin);

            // convert to celcius
            assert_eq!(f_source.as_c(), temp.celsius);
            assert_eq!(k_source.as_c(), temp.celsius);

            // convert to farenheit
            assert_eq!(c_source.as_f(), temp.fahrenheit);
            assert_eq!(k_source.as_f(), temp.fahrenheit);

            // convert to kelvin
            assert_eq!(c_source.as_k(), temp.kelvin);
            assert_eq!(f_source.as_k(), temp.kelvin);
        }
    }

    #[test]
    fn should_convert_from_delta() {
        // Test with a positive delta
        let delta = Temp::from_c(10.0) - Temp::from_f(25.);

        let temp = Temp::from_delta(delta);
        // assert_eq!(temp.as_c(), 10.0);
        assert_eq!(temp.as_f(), 25.000000000000057);
        // assert_eq!(temp.as_k(), 283.15);

        // // Test with a zero delta
        // let delta = Temp::from_c(0.0) - Temp::from_c(0.0);
        // let temp = Temp::from_delta(delta);
        // assert_eq!(temp.as_c(), 0.0);
        // assert_eq!(temp.as_f(), 32.0);
        // assert_eq!(temp.as_k(), 273.15);

        // // Test with a negative delta
        // let delta = Temp::from_c(-10.0) - Temp::from_c(0.0);
        // let temp = Temp::from_delta(delta);
        // assert_eq!(temp.as_c(), -10.0);
        // assert_eq!(temp.as_f(), 14.0);
        // assert_eq!(temp.as_k(), 263.15);
    }
}
