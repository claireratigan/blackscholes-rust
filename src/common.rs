use crate::{constants::*, Inputs, OptionType};
use num_traits::Float;
use num_traits::FromPrimitive;
use num_traits::NumCast;
use statrs::distribution::{ContinuousCDF, Normal};

/// Calculates the d1 and d2 values for the option.
/// # Requires
/// s, k, r, q, t, sigma.
/// # Returns
/// Tuple (f32, f32) of (d1, d2)
pub fn calc_d1d2<T>(inputs: &Inputs<T>) -> Result<(T, T), String>
where
    T: Float + FromPrimitive,
{
    let sigma = inputs
        .sigma
        .ok_or("Expected Some(f32) for self.sigma, received None")?;
    // Calculating numerator of d1
    let numd1 = (inputs.s / inputs.k).ln()
        + (inputs.r - inputs.q + (sigma.powi(2)) * T::from(0.5).unwrap()) * inputs.t;

    // Calculating denominator of d1 and d2
    let den = sigma * (inputs.t.sqrt());

    let d1 = numd1 / den;
    let d2 = d1 - den;

    Ok((d1, d2))
}

/// Calculates the nd1 and nd2 values for the option.
/// # Requires
/// s, k, r, q, t, sigma
/// # Returns
/// Tuple (f32, f32) of (nd1, nd2)
pub fn calc_nd1nd2<T>(inputs: &Inputs<T>) -> Result<(T, T), String>
where
    T: Float + FromPrimitive,
{
    let nd1nd2 = {
        let d1d2 = calc_d1d2(inputs)?;

        let n: Normal = Normal::new(N_MEAN, N_STD_DEV).unwrap();

        let num_cast_err: String = "Failed to cast f64 to f32".into();
        // Calculates the nd1 and nd2 values
        // Checks if OptionType is Call or Put
        match inputs.option_type {
            OptionType::Call => (
                NumCast::from(n.cdf(NumCast::from(d1d2.0).ok_or(&num_cast_err)?))
                    .ok_or(&num_cast_err)?,
                NumCast::from(n.cdf(NumCast::from(d1d2.1).ok_or(&num_cast_err)?))
                    .ok_or(&num_cast_err)?,
            ),
            OptionType::Put => (
                NumCast::from(n.cdf(NumCast::from(-d1d2.0).ok_or(&num_cast_err)?))
                    .ok_or(&num_cast_err)?,
                NumCast::from(n.cdf(NumCast::from(-d1d2.1).ok_or(&num_cast_err)?))
                    .ok_or(&num_cast_err)?,
            ),
        }
    };
    Ok(nd1nd2)
}

/// Calculates the n probability density function (PDF) for the given input.
/// # Returns
/// f32 of the value of the n probability density function.
pub fn calc_npdf<T>(x: T) -> T
where
    T: Float + FromPrimitive,
{
    let d: T = (x - T::from(N_MEAN).unwrap()) / T::from(N_STD_DEV).unwrap();
    (-T::from(HALF).unwrap() * d * d).exp() / T::from(SQRT_2PI * N_STD_DEV).unwrap()
}

/// # Returns
/// f32 of the derivative of the nd1.
pub fn calc_nprimed1<T>(inputs: &Inputs<T>) -> Result<T, String>
where
    T: Float + FromPrimitive,
{
    let (d1, _) = calc_d1d2(&inputs)?;

    // Get the standard n probability density function value of d1
    let nprimed1 = calc_npdf(d1);
    Ok(nprimed1)
}

/// # Returns
/// f32 of the derivative of the nd2.
pub fn calc_nprimed2<T>(inputs: &Inputs<T>) -> Result<T, String>
where
    T: Float + FromPrimitive,
{
    let (_, d2) = calc_d1d2(&inputs)?;

    // Get the standard n probability density function value of d1
    let nprimed2 = calc_npdf(d2);
    Ok(nprimed2)
}
