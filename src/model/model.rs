use std::{collections::HashMap, hash::Hash};

pub trait Model<S, L> {
    fn state_has(&self, state: &S, atom: &L) -> bool;
    fn transitions(&self, state: &S) -> &Vec<S>;
}

pub struct DiscreteModel<S, L> {
    labels: Vec<(S, Vec<L>)>,
    transitions: Vec<(S, Vec<S>)>,
}

impl<S: PartialEq, L: PartialEq> DiscreteModel<S, L> {
    pub fn new(labels: Vec<(S, Vec<L>)>, transitions: Vec<(S, Vec<S>)>) -> Self {
        Self {
            labels,
            transitions,
        }
    }
}

impl<S: PartialEq, L: PartialEq> Model<S, L> for DiscreteModel<S, L> {
    fn state_has(&self, state: &S, atom: &L) -> bool {
        for s in self.labels.iter() {
            if s.0 == *state {
                return s.1.contains(&atom);
            }
        }
        panic!("State is not in model!");
    }

    fn transitions(&self, state: &S) -> &Vec<S> {
        for s in self.transitions.iter() {
            if s.0 == *state {
                return &s.1;
            }
        }
        panic!("State is not in model!");
    }
}

pub struct HashedDiscreteModel<S, L> {
    labels: HashMap<S, Vec<L>>,
    transitions: HashMap<S, Vec<S>>,
}

impl<S: PartialEq, L: PartialEq> HashedDiscreteModel<S, L> {
    pub fn new(labels: HashMap<S, Vec<L>>, transitions: HashMap<S, Vec<S>>) -> Self {
        Self {
            labels,
            transitions,
        }
    }
}

impl<S: Eq + Hash, L: PartialEq> Model<S, L> for HashedDiscreteModel<S, L> {
    fn state_has(&self, state: &S, atom: &L) -> bool {
        if let Some(atoms) = self.labels.get(&state) {
            return atoms.contains(&atom);
        }
        panic!("State is not in model!");
    }

    fn transitions(&self, state: &S) -> &Vec<S> {
        if let Some(nexts) = self.transitions.get(&state) {
            return nexts;
        }
        panic!("State is not in model!");
    }
}