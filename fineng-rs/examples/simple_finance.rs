use std::collections::BTreeSet;
use std::f64::EPSILON;

use fineng_rs::prob::random_variable::RandomVariable;
use fineng_rs::prob::spaces::{Event, SampleSpace, SigmaAlgebra};
use statrs::distribution::{Continuous, ContinuousCDF, Uniform};

fn setup_sample_space_finance(start:f64, end:f64, step:f64) -> SampleSpace<f64> {
    let mut list = SampleSpace::new();
    let mut current_value = start;
    while current_value<= end + EPSILON{
        let rounded_value = (current_value*100.0).round()/100.0;
        list.push(rounded_value);
        current_value+=step;
    }
    list
}

/// Simple Random Function
/// Just mapping the variable to itself
fn random_variable(x:&f64)->f64{
    *x 
}


fn main() {
    let sample_space = setup_sample_space_finance(80.0, 120.0, 0.1);
    let mut sigma_algebra = SigmaAlgebra::trivial(&sample_space);

    // Creating an event where the prices are between 90.0 and 110.0
    let lower_bound = 90.0;
    let upper_bound = 110.0;

    let start_val_index = sample_space.find_index_of_f64(lower_bound).unwrap_or(0);
    let end_val_index = sample_space.find_index_of_f64(upper_bound).unwrap_or(sample_space.cardinality());

    let new_event_a: BTreeSet<usize> = (start_val_index..=end_val_index).collect(); 
    let event_a = Event(new_event_a);
    sigma_algebra.set_generator(vec![event_a]);

    println!("Creating new Price Sigma Algebra");
    println!("start_val: {} index: {}", lower_bound, start_val_index);
    println!("end_val: {} index: {}", upper_bound, end_val_index);
    println!("{:#?}", sigma_algebra);

    // Create the Uniform distribution object
    let stock_dist = Uniform::new(lower_bound, upper_bound)
        .expect("Invalid bounds for Uniform distribution");
    let pdf = |x: f64| stock_dist.pdf(x);

    // Demonstration of the pdf and probability of a value between a rand of values
    let x_test = 99.0;
    let density_at_x = pdf(x_test);
    let prob_95_to_100 = stock_dist.cdf(100.0) - stock_dist.cdf(x_test);

    println!("\n--- Theoretical PDF using statrs ---");
    println!("Theoretical density f({}) = {}", x_test, density_at_x);
    println!("Theoretical Probability P({} <= X <= 100) = {}",x_test, prob_95_to_100);

    let simpla_map = RandomVariable::new(random_variable);
    let _sigma_hat = simpla_map.create_image(&sample_space);

}
