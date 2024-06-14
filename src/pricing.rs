use crate::{common::*, constants, lets_be_rational, Inputs, OptionType};
use num_traits::Float;
pub trait Pricing<T>
where
    T: Float,
{
    fn calc_price(&self) -> Result<T, String>;
    fn calc_rational_price(&self) -> Result<f64, String>;
}

macro_rules! impl_pricing {
    ($type:ty, $type_name:ident) => {
        impl Pricing<$type> for Inputs<$type> {
            /// Calculates the price of the option.
            /// # Requires
            /// s, k, r, q, t, sigma.
            /// # Returns
            /// $type of the price of the option.
            /// # Example
            /// ```
            /// use blackscholes::{Inputs, OptionType, Pricing};
            /// let inputs = Inputs::new(OptionType::Call, 100.0, 100.0, None, 0.05, 0.2, 20.0/365.25, Some(0.2));
            /// let price = inputs.calc_price().unwrap();
            /// ```
            fn calc_price(&self) -> Result<$type, String> {
                // Calculates the price of the option
                let (nd1, nd2): ($type, $type) = calc_nd1nd2(&self)?;
                let price: $type = match self.option_type {
                    OptionType::Call => <$type>::max(
                        0.0,
                        nd1 * self.s * constants::$type_name::E.powf(-self.q * self.t)
                            - nd2 * self.k * constants::$type_name::E.powf(-self.r * self.t),
                    ),
                    OptionType::Put => <$type>::max(
                        0.0,
                        nd2 * self.k * constants::$type_name::E.powf(-self.r * self.t)
                            - nd1 * self.s * constants::$type_name::E.powf(-self.q * self.t),
                    ),
                };
                Ok(price)
            }

            /// Calculates the price of the option using the "Let's Be Rational" implementation.
            /// # Requires
            /// s, k, r, q, t, sigma.
            /// # Returns
            /// f64 of the price of the option.
            /// # Example
            /// ```
            /// use blackscholes::{Inputs, OptionType, Pricing};
            /// let inputs = Inputs::new(OptionType::Call, 100.0, 100.0, None, 0.05, 0.2, 20.0/365.25, Some(0.2));
            /// let price = inputs.calc_rational_price().unwrap();
            /// ```
            fn calc_rational_price(&self) -> Result<f64, String> {
                let sigma = self
                    .sigma
                    .ok_or("Expected Some($type) for self.sigma, received None")?;

                // let's be rational wants the forward price, not the spot price.
                let forward = self.s * ((self.r - self.q) * self.t).exp();

                // convert the option type into \theta
                let q: f64 = match self.option_type {
                    OptionType::Call => 1.0,
                    OptionType::Put => -1.0,
                };

                // price using `black`
                let undiscounted_price = lets_be_rational::black(
                    forward as f64,
                    self.k as f64,
                    sigma as f64,
                    self.t as f64,
                    q,
                );

                // discount the price
                let price = undiscounted_price * (-self.r as f64 * self.t as f64).exp();
                Ok(price)
            }
        }
    };
}

impl_pricing!(f32, f32);
impl_pricing!(f64, f64);
