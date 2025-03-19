mod cmp;
mod sub;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

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
}
