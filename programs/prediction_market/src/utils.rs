use anchor_lang::prelude::*;


use crate::error::CustomError;
use crate::constants::TOKEN_DECIMALS;
use crate::constants::SHARES_DECIMALS;

// Calculates the LMSR cost function
/// `q` is a vector of shares for each outcome.
/// `b` is the liquidity parameter.
pub fn calculate_cost(q: &Vec<u64>, b: u64) -> Result<u64> {
    use std::f64;

    // Convert b to f64 for calculations
    let b_f64 = b as f64;

    // Calculate exponentials of (q[i] / b)
    let exponentials: Vec<f64> = q
        .iter()
        .map(|&qi| (qi as f64 / b_f64).exp()) // e^(qi / b)
        .collect();

    // Sum all exponentials
    let sum_exp: f64 = exponentials.iter().sum();

    // Ensure sum_exp is valid
    if sum_exp <= 0.0 {
        return Err(error!(CustomError::MathError));
    }

    // Take the natural logarithm and scale back
    let cost: f64 = b_f64 * sum_exp.ln();

    let scale_factor = 10u64.pow((TOKEN_DECIMALS - SHARES_DECIMALS) as u32) as f64;
    let scaled_cost = cost * scale_factor;

    // Convert back to u64
    Ok(scaled_cost.round() as u64) // Round to nearest integer
}




/// Calculates the fee based on cost and fee percent
pub fn calculate_fee(cost: u64, fee_percent: u64) -> Result<u64> {
    // fee_percent is expected to be in basis points (e.g., 500 for 5%)
    let fee = (cost.checked_mul(fee_percent).ok_or(CustomError::Overflow)?)
        .checked_div(10000)
        .ok_or(CustomError::Overflow)?;
    Ok(fee)
}
pub fn calculate_required_initial_funds(b: u64, num_outcomes: usize) -> Result<u64> {
    use std::f64;

    // Handle edge cases
    if num_outcomes == 0 {
        return Err(error!(CustomError::InvalidInput)); // No outcomes means invalid input
    }

    // Convert inputs to floating-point for calculations
    let b_f64 = b as f64;
    let n_f64 = num_outcomes as f64;

    // Ensure b is greater than 0
    if b_f64 <= 0.0 {
        return Err(error!(CustomError::InvalidLiquidityParameter)); // b must be positive
    }

    // Calculate the required initial funds
    let required_funds = b_f64 * n_f64.ln(); // b * ln(num_outcomes)

    // Scale the result to match the precision of TOKEN_DECIMALS
    let scale_factor = 10u64.pow(TOKEN_DECIMALS as u32) as f64;
    let scaled_funds = required_funds * scale_factor;

    // Ensure the result is finite and positive
    if !scaled_funds.is_finite() || scaled_funds <= 0.0 {
        return Err(error!(CustomError::MathError)); // Invalid calculation
    }

    // Convert the result back to u64, rounding up to ensure adequacy
    Ok(scaled_funds.ceil() as u64)
}

