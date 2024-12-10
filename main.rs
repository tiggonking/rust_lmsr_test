+ /// Calculates the price for a given outcome index based on the quantities `q` and the parameter `b`.
+ /// The price is calculated as the ratio of the exponential of the `outcome_index`-th quantity divided by `b`
+ /// to the sum of the exponentials of all quantities divided by `b`.
+ 
+ /// Calculates the total cost based on the given quantities `q` and the parameter `b`.
+ /// The cost is calculated using the formula `b * sum(exp(q_i / b))`.
+ 
use std::f64::consts::E;

/// Calculates the total cost based on the given quantities `q` and the parameter `b`.
/// The cost is calculated using the formula `b * sum(exp(q_i / b))`.
fn calculate_cost(q: &[f64], b: f64) -> f64 {
    let sum_exp_q_over_b: f64 = q.iter().map(|qi| (qi / b).exp()).sum();
    b * sum_exp_q_over_b.ln()
}

/// Calculates the price for a given outcome index based on the quantities `q` and the parameter `b`.
/// The price is calculated as the ratio of the exponential of the `outcome_index`-th quantity divided by `b`
/// to the sum of the exponentials of all quantities divided by `b`.
fn calculate_price(q: &[f64], b: f64, outcome_index: usize) -> f64 {
    let sum_exp_q_over_b: f64 = q.iter().map(|qi| (qi / b).exp()).sum();
    let exp_qi_over_b = (q[outcome_index] / b).exp();

    exp_qi_over_b / sum_exp_q_over_b
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::EPSILON;

    #[test]
    fn test_calculate_cost_empty_vector() {
        let q = vec![];
        let b = 50.0;
        let cost = calculate_cost(&q, b);
        assert!(cost.abs() < EPSILON, "Cost should be zero for empty input");
    }

    #[test]
    fn test_calculate_cost_equal_bets() {
        let q = vec![100.0, 100.0, 100.0];
        let b = 50.0;
        let cost = calculate_cost(&q, b);
        println!("Equal bets cost: {}", cost);
        assert!(cost > 0.0, "Cost should be positive");
    }

    #[test]
    fn test_calculate_cost_high_b_value() {
        let q = vec![100.0, 200.0, 300.0];
        let b = 1000.0;  // Large b value
        let cost = calculate_cost(&q, b);
        println!("High b value cost: {}", cost);
        assert!(cost > 0.0, "Cost should still be positive");
    }

    #[test]
    fn test_calculate_cost_low_b_value() {
        let q = vec![100.0, 200.0, 300.0];
        let b = 0.1;  // Small b value
        let cost = calculate_cost(&q, b);
        println!("Low b value cost: {}", cost);
        assert!(cost > 0.0, "Cost should still be positive");
    }

    #[test]
    fn test_calculate_cost_negative_values() {
        let q = vec![-100.0, 200.0, -300.0];
        let b = 50.0;
        let cost = calculate_cost(&q, b);
        println!("Negative values cost: {}", cost);
        assert!(cost.is_finite(), "Cost should be finite even with negative inputs");
    }

    #[test]
    fn test_calculate_price_single_outcome() {
        let q = vec![100.0];
        let b = 50.0;
        let price = calculate_price(&q, b, 0);
        println!("Price for single outcome: {}", price);
        assert!((price - 1.0).abs() < EPSILON, "Price should be 1 for a single outcome");
    }

    #[test]
    fn test_calculate_price_zero_bets() {
        let q = vec![0.0, 0.0, 0.0];
        let b = 50.0;
        let price = calculate_price(&q, b, 1);
        println!("Price for zero bets: {}", price);
        assert!(price.is_nan(), "Price should be NaN for zero bets");
    }

    #[test]
    fn test_calculate_price_high_concentration() {
        let q = vec![1000.0, 1.0, 1.0];
        let b = 50.0;
        let price = calculate_price(&q, b, 0);
        println!("Price with high concentration: {}", price);
        assert!(price > 0.9, "Price should be very high for concentrated outcome");
    }

    #[test]
    fn test_calculate_price_outcome_index_out_of_bounds() {
        let q = vec![100.0, 200.0];
        let b = 50.0;
        let result = std::panic::catch_unwind(|| {
            calculate_price(&q, b, 2);  // Invalid index
        });
        assert!(result.is_err(), "Should panic for index out of bounds");
    }
}
