use std::collections::BTreeSet;
use std::f64::EPSILON;

use fineng_rs::prob::spaces::{Event, SampleSpace, SigmaAlgebra};

fn setup_sample_space() -> SampleSpace<i32> {
    let mut list = SampleSpace::new();
    list.push(1);
    list.push(2);
    list.push(3);
    list.push(4);
    list
}


fn setup_sample_space_finance(start:f64, end:f64, step:f64) -> SampleSpace<f64> {
    let mut list = SampleSpace::new();
    let mut current_value = start;
    while current_value<= end + EPSILON {
        let rounded_value = (current_value*100.0).round()/100.0;
        list.push(rounded_value);
        current_value+=step;
    }
    list
}


fn setup_string_sample_space() -> SampleSpace<String> {
    let mut list = SampleSpace::new();
    list.push("Red".to_string());    // Index 0
    list.push("Green".to_string());  // Index 1
    list.push("Blue".to_string());   // Index 2
    list.push("Yellow".to_string()); // Index 3
    list
}

#[test]
fn test_sample_space() {
    let mut new_ss: SampleSpace<f64> = SampleSpace::new();
    assert!(new_ss.is_empty(), "List should be empty");
    assert_eq!(new_ss.cardinality(), 0);

    new_ss.push(10.0);
    new_ss.push(30.0);
    new_ss.push(20.0);

    assert!(!new_ss.is_empty());
    assert_eq!(new_ss.cardinality(), 3);

    let expected = vec![10.0, 30.0, 20.0];
    let mut actual = Vec::new();

    for &item in &new_ss {
        actual.push(item);
    }
    assert_eq!(actual, expected);
}
#[test]
fn test_sample_space_strings() {
    let mut list: SampleSpace<String> = SampleSpace::new();
    list.push("Rust".to_string());
    list.push("2021".to_string());

    assert_eq!(list.cardinality(), 2);

    let expected = vec!["Rust".to_string(), "2021".to_string()];

    // Iterating over owned Strings
    let actual: Vec<String> = list.into_iter().collect();
    assert_eq!(actual, expected);
}

#[test]
fn test_trivial_sigma_algebra() {
    let sample_space = setup_sample_space();
    let sigma_algebra = SigmaAlgebra::trivial(&sample_space);
    assert_eq!(sigma_algebra.events.len(), 2);

    let _empty_set = Event(BTreeSet::new());

    let omega: BTreeSet<usize> = (0..sample_space.cardinality()).collect(); // Indices {0, 1, 2, 3}
    let _omega_event = Event(omega);
}

#[test]
fn test_sigma_algebbra_add() {
    let sample_space = setup_sample_space();
    let mut sigma_algebra = SigmaAlgebra::trivial(&sample_space);
    let new_event:BTreeSet<usize> =(0..2).collect();
    let new_event_2:BTreeSet<usize> =(1..3).collect();

    let new_event = Event(new_event);
    let new_event_2 = Event(new_event_2);
    let mut new_events = vec![new_event];

    // add further events
    new_events.push(new_event_2);

    sigma_algebra.set_generator(new_events);
    println!("{:#?}", sigma_algebra);

}

#[test]
fn test_complement_operation() {
    let sample_space = setup_sample_space();
    let sigma_algebra = SigmaAlgebra::trivial(&sample_space); // Need the SS for context

    // Define an event A = {1, 3} -> Indices {0, 2}
    let event_a_indices: BTreeSet<usize> = BTreeSet::from([0, 2]);
    let event_a = Event(event_a_indices);
    println!("{:?}", &event_a);

    // Compute the complement: A^c
    let complement_event = sigma_algebra.complement(&event_a);
    println!("{:?}", &complement_event);

    // Expected complement: {2, 4} -> Indices {1, 3}
    let expected_indices: BTreeSet<usize> = BTreeSet::from([1, 3]);

    assert_eq!(
        complement_event.0, expected_indices,
        "Complement operation failed: indices do not match"
    );

    let double_complement = sigma_algebra.complement(&complement_event);
    assert_eq!(
        double_complement.0, event_a.0,
        "Double complement must return the original event"
    );
}

#[test]
fn test_string_sample_space_sigma_algebra() {
    let sample_space = setup_string_sample_space();
    let mut sigma_algebra = SigmaAlgebra::trivial(&sample_space);

    // Event A: The outcome is "Red" or "Green"
    let new_event_a: BTreeSet<usize> = (0..2).collect(); // Indices {0, 1}
    let event_a = Event(new_event_a);
    
    // Event B: The outcome is "Green" or "Blue"
    let new_event_b: BTreeSet<usize> = (1..3).collect(); // Indices {1, 2}
    let event_b = Event(new_event_b);

    let new_generators = vec![event_a, event_b];

    sigma_algebra.set_generator(new_generators);

    println!("--- Sample Space Content ---");
    println!("Indices 0..3: {:?}", sample_space.0);
    println!("--------------------------\n");

    println!("--- Sigma Algebra Generators ---");
    println!("{:#?}", sigma_algebra);

}


#[test]
fn test_sigma_algebra() {
    let sample_space = setup_sample_space_finance(80.0, 120.0, 0.1);
    let mut sigma_algebra = SigmaAlgebra::trivial(&sample_space);

    // Price values between 90 and 110
    let start_val = sample_space.find_index_of_f64(90.0);
    let end_val= sample_space.find_index_of_f64(110.0);
    let (s,e) = match (start_val, end_val) {
        (Some(s), Some(e)) => (s,e),
        _=>(0, 0)
    };


    // Create event 
    let new_event_a: BTreeSet<usize> = (s..e).collect(); // Indices {0, 1}
    let event_a = Event(new_event_a);
    let new_events = vec![event_a];
    sigma_algebra.set_generator(new_events);

    println!("Creating new Price Sigma Algebra");
    println!("start_val: {} index: {}",90.0, s);
    println!("end_val: {} index: {}",110.0, e);
    println!("{:#?}", sigma_algebra);



}
