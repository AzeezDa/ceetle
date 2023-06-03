use std::{collections::HashMap, hash::Hash};

/// # `Model`
/// Any Computional Tree Logic model can be built using two functions: a membership function and a transition function. 
/// The `Model` trait is a generalisation of the state-model, where:
/// - The `state_has` function is the membership function that takes a state in the model and an atom and returns true if the atom exists in the state
/// - The `transitions` function is the transitions function that takes a state and returns a `&Vec` of the possible transitions from that state
/// 
/// The model is parametrised by two types `S` and `A`. `S` is the type of state, usually states are be labeled by a string (`s0`, `s1`, etc.), so a `&str` could work here. 
/// The `A` type is the type of atoms used in the model, usually a string or char (`p`, `q`, etc.) is used for `A` as well.
pub trait Model<S, A> {
    fn state_has(&self, state: &S, atom: &A) -> bool;
    fn transitions(&self, state: &S) -> &Vec<S>;
}

/// # `DiscreteModel`
/// The `DiscreteModel` is identical to a finite-state automaton. It uses a `Vec` to store each state together with its atoms and transitions. 
/// Therefore, lookup is `O(n)`, where `n` is the number of states.
pub struct DiscreteModel<S, A> {
    states: Vec<(S, Vec<A>, Vec<S>)>,
}

impl<S: PartialEq, A: PartialEq> DiscreteModel<S, A> {
    /// # `new`
    /// Returns a new `DiscreteModel` with a structure given as a `Vec` of tuples `(S, Vec<A>, Vec<S>)`, where
    /// - The first value is the state label
    /// - The second value is a `Vec` of atoms that hold in that state
    /// - The last value is a `Vec` of states that can be transitioned to from this state
    /// 
    /// ## Examples
    /// ```
    /// use ceetle::DiscreteModel;
    /// 
    /// let model = DiscreteModel::new(vec![
    ///     ("s0", vec!["a"], vec!["s0", "s1"]),
    ///     ("s1", vec!["a", "b"], vec!["s0"])
    /// ]);
    /// ```
    pub fn new(states: Vec<(S, Vec<A>, Vec<S>)>) -> Self {
        Self {
            states,
        }
    }
}

impl<S: PartialEq, A: PartialEq> Model<S, A> for DiscreteModel<S, A> {

    /// # `state_has`
    /// Takes a state and an atom and returns true if the atom holds in that state. This operation is `O(n)`, where `n` is the number of states.
    fn state_has(&self, state: &S, atom: &A) -> bool {
        for s in self.states.iter() {
            if s.0 == *state {
                return s.1.contains(&atom);
            }
        }
        panic!("State is not in model!");
    }

    /// # `transitions`
    /// Takes a state and returns a `&Vec` of the states that the given state can transition to. This operation is `O(n)`, where `n` is the number of states.
    fn transitions(&self, state: &S) -> &Vec<S> {
        for s in self.states.iter() {
            if s.0 == *state {
                return &s.2;
            }
        }
        panic!("State is not in model!");
    }
}

/// # `HashedDiscreteModel`
/// The `HashedDiscreteModel` is identical to a finite-state automaton. It uses a `HashMap` to store each state together with its atoms and transitions. 
/// Therefore, lookup is `O(1)`.
pub struct HashedDiscreteModel<S, A> {
    states: HashMap<S, (Vec<A>, Vec<S>)>,
}

impl<S: PartialEq, A: PartialEq> HashedDiscreteModel<S, A> {
    /// # `new`
    /// Returns a new `HashedDiscreteModel` with a structure given as a `HashMap` of keys of type `S` and values of type `(Vec<A>, Vec<S>)`, where
    /// - The key is the state label
    /// - The first of the value's tuple is a `Vec` of atoms that hold in that state
    /// - The second of the value's tuple is a `Vec` of states that can be transitioned to from this state
    /// 
    /// ## Examples
    /// ```
    /// use std::collections::HashMap;
    /// use ceetle::HashedDiscreteModel;
    /// 
    /// let mut states = HashMap::new();
    /// states.insert("s0", (vec!["a"], vec!["s0", "s1"]));
    /// states.insert("s1", (vec!["a", "b"], vec!["s0", "s2"]));
    /// states.insert("s2", (vec!["a"], vec!["s3"]));
    /// states.insert("s3", (vec!["a", "c"], vec!["s1", "s3"]));
    /// 
    /// let model = HashedDiscreteModel::new(states);
    /// ```
    pub fn new(states: HashMap<S, (Vec<A>, Vec<S>)>) -> Self {
        Self {
            states
        }
    }
}

impl<S: Eq + Hash, A: PartialEq> Model<S, A> for HashedDiscreteModel<S, A> {

    /// # `state_has`
    /// Takes a state and an atom and returns true if the atom holds in that state. This operation is `O(1)`.
    fn state_has(&self, state: &S, atom: &A) -> bool {
        if let Some(atoms) = self.states.get(&state) {
            return atoms.0.contains(&atom);
        }
        panic!("State is not in model!");
    }

    /// # `transitions`
    /// Takes a state and returns a `&Vec` of the states that the given state can transition to. This operation is `O(1)`.
    fn transitions(&self, state: &S) -> &Vec<S> {
        if let Some(nexts) = self.states.get(&state) {
            return &(*nexts).1;
        }
        panic!("State is not in model!");
    }
}
