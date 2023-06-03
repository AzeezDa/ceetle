use std::{collections::HashMap, hash::Hash};

/// # `Model`
/// Any Computional Tree Logic model can be built using two functions: a membership function and a transition function. 
/// The `Model` trait is a generalisation of the state-model, where:
/// - The `state_has` function is the membership function that takes a state in the model and an atom and returns true if the atom exists in the state
/// - The `transitions` function is the transitions function that takes a state and returns a `&Vec` of the possible transitions from that state
/// 
/// The model is parametrised by two types `S` and `L`. `S` is the type of state, usually states are be labeled by a string (`s0`, `s1`, etc.), so a `&str` could work here. 
/// The `L` type is the type of atoms used in the model, usually a string or char (`p`, `q`, etc.) is used for `L` as well.
pub trait Model<S, L> {
    fn state_has(&self, state: &S, atom: &L) -> bool;
    fn transitions(&self, state: &S) -> &Vec<S>;
}

/// # `DiscreteModel`
/// The `DiscreteModel` is identical to a finite-state automaton. It uses a `Vec` to store each state together with its atoms and transitions. 
/// Therefore, lookup is `O(n)`, where `n` is the number of states.
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

/// # `HashedDiscreteModel`
/// The `HashedDiscreteModel` is identical to a finite-state automaton. It uses a `HashMap` to store each state together with its atoms and transitions. 
/// Therefore, lookup is `O(1)`.
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
