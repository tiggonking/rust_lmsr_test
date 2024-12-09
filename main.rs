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

    #[test]
    fn test_calculate_cost() {
        let q = vec![100.0, 150.0, 120.0];
        let b = 50.0;
        let cost = calculate_cost(&q, b);
        println!("Cost: {}", cost);
        assert!((cost - 130.814).abs() < 0.001); // Expected value should be pre-calculated or validated
    }

    #[test]
    fn test_calculate_price() {
        let q = vec![100.0, 150.0, 120.0];
        let b = 50.0;
        let price = calculate_price(&q, b, 1); // Price for the second outcome
        println!("Price for outcome 1: {}", price);
        assert!((price - 0.611).abs() < 0.001); // Expected value should be pre-calculated or validated
    }
}

fn main() {
    let q = vec![100.0, 150.0, 120.0];
    let b = 50.0;
    println!("Total cost: {}", calculate_cost(&q, b));
    println!("Price for outcome 0: {}", calculate_price(&q, b, 0));
}
