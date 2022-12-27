mod model;
pub use model::*;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use crate::model::{HashedDiscreteModel, Model, verify};

    #[test]
    fn create_and_check_discrete_model() {
        let model = model::DiscreteModel::new(
            vec![
                ("s0", vec!["a"]),
                ("s1", vec!["a", "b"]),
                ("s2", vec!["a"]),
                ("s3", vec!["a", "c"]),
            ],
            vec![
                ("s0", vec!["s0", "s1"]),
                ("s1", vec!["s0", "s2"]),
                ("s2", vec!["s3"]),
                ("s3", vec!["s1", "s3"]),
            ],
        );

        assert!(model.state_has(&"s0", &"a"));
        assert!(!model.state_has(&"s3", &"b"));
        assert_eq!(*model.transitions(&"s1"), vec!["s0", "s2"]);
    }

    #[test]
    fn create_and_check_hashed_discrete_model() {
        let mut labels = HashMap::new();
        labels.insert("s0", vec!["a"]);
        labels.insert("s1", vec!["a", "b"]);
        labels.insert("s2", vec!["a"]);
        labels.insert("s3", vec!["a", "c"]);

        let mut transitions = HashMap::new();
        transitions.insert("s0", vec!["s0", "s1"]);
        transitions.insert("s1", vec!["s0", "s2"]);
        transitions.insert("s2", vec!["s3"]);
        transitions.insert("s3", vec!["s1", "s3"]);

        let model = HashedDiscreteModel::new(labels, transitions);
        assert!(model.state_has(&"s0", &"a"));
        assert!(!model.state_has(&"s3", &"b"));
        assert_eq!(*model.transitions(&"s1"), vec!["s0", "s2"]);
    }
}
