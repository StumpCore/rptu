use core::fmt;
use std::collections::BTreeSet;
use std::f64::EPSILON;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Event(pub BTreeSet<usize>);

/// SampleSpace
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SampleSpace<T>(pub Vec<T>);

impl<T> SampleSpace<T> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push(&mut self, item: T) {
        self.0.push(item);
    }

    pub fn cardinality(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl SampleSpace<f64>{
    pub fn find_index_of_f64(&self, target:f64)->Option<usize> {
        self.0
            .iter()
            .enumerate()
            .find(|&value|{
                (value.1 - target).abs() < EPSILON
            })
        .map(|(index, _value)|index)
    }
}

/// Implementing consuming iterator
impl<T> IntoIterator for SampleSpace<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

/// Implementing non consuming iterator
impl<'a, T> IntoIterator for &'a SampleSpace<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

/// Sigma-Algebra implementation
#[derive(Clone, PartialEq, Eq)]
pub struct SigmaAlgebra<'a, T> {
    pub sample_space: &'a SampleSpace<T>,
    pub events: Vec<Event>,
}

impl<'a, T> SigmaAlgebra<'a, T> {

    pub fn trivial(sample_space: &'a SampleSpace<T>) -> Self {
        let omega_indices: BTreeSet<usize> = (0..sample_space.cardinality()).collect();
        let omega_event = Event(omega_indices);
        let empty_set = Event(BTreeSet::new());
        Self {
            sample_space,
            events: vec![empty_set, omega_event],
        }
    }

    pub fn complement(&self, event: &Event) -> Event {
        let omega_indices: BTreeSet<usize> = (0..self.sample_space.cardinality()).collect();
        let complement_indices: BTreeSet<usize> = omega_indices.difference(&event.0)
            .cloned()
            .collect();

        Event(complement_indices)
    }

    /// Setting the generators of the Sigma Algebra
    pub fn set_generator(&mut self, new_events: Vec<Event>) {

        // Validation: Ensure all indices in the new_event are valid for the Sample Space.
        let omega_len = self.sample_space.cardinality();
        for event in new_events.iter(){
            for &index in event.0.iter() {
                if index >= omega_len {
                    panic!("Cannot add Event: Index {} is outside the bounds of the Sample Space", index);
                }
            }
        }

        self.events.clear();

        let omega_indices: BTreeSet<usize> = (0..self.sample_space.cardinality()).collect();
        let omega_event = Event(omega_indices);
        let empty_set = Event(BTreeSet::new());
        let mut unique_events_data = BTreeSet::new();

        unique_events_data.insert(empty_set.0);
        unique_events_data.insert(omega_event.0);

        for event in new_events.into_iter(){
            let event_a_complement = self.complement(&event);
            unique_events_data.insert(event.0);
            unique_events_data.insert(event_a_complement.0);
        }

        self.events = unique_events_data.into_iter().map(Event).collect();
    }
}

impl <'a,T> fmt::Debug for SigmaAlgebra<'a, T>
where
    T: fmt::Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Reference to the sample space with all string values
        let data_vec = &self.sample_space.0;

        // Mapping the generator sets to the underlying string values
        let trans_events:Vec<Vec<&T>> = self.events.iter().map(|event|{
            event.0.iter()
                .filter_map(|&index|{
                    data_vec.get(index)
                })
                .collect()
        }).collect();

        f.debug_struct("SigmaAlgebra")
            .field("sample_space", &self.sample_space)
            .field("events_indices", &self.events) // Keep the raw index view for clarity
            .field("events_transformed", &trans_events) // Print the temporary, transformed data
            .finish()
    }

}
