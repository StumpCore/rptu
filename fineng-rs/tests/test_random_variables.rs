use std::collections::BTreeSet;

use fineng_rs::prob::random_variable::{RandomVariable};
use fineng_rs::prob::spaces::{SampleSpace};
fn setup_vowel_rv() -> (SampleSpace<String>, RandomVariable<String, f64>) {
    let mut outcomes = SampleSpace::new();
    outcomes.push("alpha".to_string()); // Index 0: Value 2.0
    outcomes.push("rust".to_string());  // Index 1: Value 1.0
    outcomes.push("aeiou".to_string()); // Index 2: Value 5.0
    outcomes.push("xyz".to_string());   // Index 3: Value 0.0
    outcomes.push("beta".to_string());  // Index 4: Value 2.0
    
    let vowel_counter = RandomVariable::new(|outcome: &String| -> f64 {
        outcome.chars().filter(|c| "aeiouAEIOU".contains(*c)).count() as f64
    });
    (outcomes, vowel_counter)
}

#[test]
fn test_vowel_count_random_variable() {
    fn mapping_fn(outcome:&String)->f64{
        outcome
            .chars()
            .filter(|c| "aeiouAEIOU".contains(*c))
            .count() as f64
    }
    // The Sample Space (Omega) of string outcomes
    let mut outcomes = SampleSpace::new();
    outcomes.push("alpha".to_string()); 
    outcomes.push("rust".to_string());  
    outcomes.push("aeiou".to_string()); 
    outcomes.push("xyz".to_string());   

    println!("Sample Space:{:?}", &outcomes);

    let vowel_counter = RandomVariable::new(mapping_fn);
    let mapped_values_list: SampleSpace<f64> = vowel_counter.create_image(&outcomes);

    // Extract the underlying vector for easy comparison
    let actual_values = mapped_values_list.0;

    // Expected values: [2.0, 1.0, 5.0, 0.0]
    let expected_values = vec![2.0, 1.0, 5.0, 0.0];

    assert_eq!(actual_values, expected_values, "The Random Variable mapping failed to produce the correct distribution of values.");
}

#[test]
fn test_pre_image_exact_value() {
    let (outcomes, rv) = setup_vowel_rv();

    // Condition: Event $A$ such that $X=2$ (two vowels)
    let two_vowel_event = rv.invert_image(&outcomes, |value| (value - 2.0).abs() < f64::EPSILON);

    // Expected outcomes: "alpha" (Index 0), "beta" (Index 4)
    let expected_indices = BTreeSet::from([0, 4]);

    assert_eq!(two_vowel_event.0, expected_indices, "Pre-image for X=2 failed");
}

#[test]
fn test_pre_image_inequality() {
    let (outcomes, rv) = setup_vowel_rv();

    // Condition: Event $B$ such that $X > 1.0$
    let event_b = rv.invert_image(&outcomes, |value| value > 1.0);

    // Expected outcomes: "alpha" (0, 2.0), "aeiou" (2, 5.0), "beta" (4, 2.0) -> Indices {0, 2, 4}
    let expected_indices = BTreeSet::from([0, 2, 4]);

    assert_eq!(event_b.0, expected_indices, "Pre-image for X > 1.0 failed");
}
