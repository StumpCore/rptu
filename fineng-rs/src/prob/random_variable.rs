use std::collections::BTreeSet;
use std::fmt::Debug;
use super::spaces::{Event, SampleSpace};

type RandomVariableFunction<T, S>=Box<dyn Fn(&T)->S>;
pub struct RandomVariable<T,S> {
    mapping:RandomVariableFunction<T,S>,
}

impl<T,S> RandomVariable<T,S>
where
    T:Debug,
    S:Debug,
{
    pub fn new(mapping:impl Fn(&T)->S+'static)->Self{
        RandomVariable{
            mapping: Box::new(mapping),
        }
    }

    pub fn image(&self, outcome:&T) ->S{
        (self.mapping)(outcome)
    }


    pub fn create_image(&self, sample_space:&SampleSpace<T>)->SampleSpace<S>{
        let map_values = sample_space
            .into_iter()
            .map(|outcome| self.image(outcome))
            .collect();
        SampleSpace(map_values)
    }

    pub fn invert_image(&self, 
        sample_space:&SampleSpace<T>,
        predicate: impl Fn(S) -> bool,
        )->Event{
        let idx = sample_space
            .0
            .iter()
            .enumerate()
            .filter_map(|(index,outcome)|{
                let map_value = self.image(outcome);
                if predicate(map_value) {
                    Some(index)
                } else {
                    None
                }
        })
        .collect::<BTreeSet<usize>>();
        Event(idx)

    }
}
