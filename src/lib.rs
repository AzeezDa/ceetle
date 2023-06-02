mod model;
pub use model::*;

use ctl_proc_macro::ctl;

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, vec};

    use super::*;
    use crate::model::{HashedDiscreteModel, Model, DiscreteModel, verify, CTLFormula};

    #[test]
    fn create_and_check_discrete_model() {
        let model = DiscreteModel::new(
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

    #[test]
    fn check_ex() {
        let model = DiscreteModel::new(
            vec![
                ("s0", vec![]),
                ("s1", vec!["p"]),
                ("s2", vec!["q"]),
                ("s3", vec!["p", "q"]),
            ],
            vec![
                ("s0", vec!["s1", "s2"]),
                ("s1", vec!["s1", "s2", "s3"]),
                ("s2", vec!["s1"]),
                ("s3", vec!["s0"]),
            ]
        );

        assert!(verify(&model, &"s0", &_ctlb!(EX, _ctlb!(Atom, "p"))));
        assert!(!verify(&model, &"s0", &_ctlb!(EX, _ctlb!(And, _ctlb!(Atom, "p"), _ctlb!(Atom, "q")))));
        assert!(verify(&model, &"s0", &_ctlb!(EX, _ctlb!(EX, _ctlb!(And, _ctlb!(Atom, "p"), _ctlb!(Atom, "q"))))));
    }

    #[test]
    fn check_ax() {
        let model = DiscreteModel::new(
            vec![
                ("s0", vec![]),
                ("s1", vec!["p"]),
                ("s2", vec!["q"]),
                ("s3", vec!["p", "q"]),
            ],
            vec![
                ("s0", vec!["s1", "s2"]),
                ("s1", vec!["s1", "s2", "s3"]),
                ("s2", vec!["s1"]),
                ("s3", vec!["s0"]),
            ]
        );

        assert!(verify(&model, &"s0", &_ctlb!(AX, _ctlb!(Or, _ctlb!(Atom, "p"), _ctlb!(Atom, "q")))));
        assert!(!verify(&model, &"s0", &_ctlb!(AX, _ctlb!(AX, _ctlb!(AX, _ctlb!(Or, _ctlb!(Atom, "p"), _ctlb!(Atom, "q")))))));
        assert!(verify(&model, &"s0", &_ctlb!(AX, _ctlb!(Imply, _ctlb!(Atom, "p"), _ctlb!(Not, _ctlb!(Atom, "q"))))));
    }

    #[test]
    fn check_eg() {
        let model = DiscreteModel::new(
            vec![
                ("s0", vec!["p"]),
                ("s1", vec!["p"]),
                ("s2", vec![]),
                ("s3", vec!["p", "q"]),
            ],
            vec![
                ("s0", vec!["s1"]),
                ("s1", vec!["s2", "s3"]),
                ("s2", vec!["s0", "s3"]),
                ("s3", vec!["s3"]),
            ]
        );

        assert!(verify(&model, &"s0", &_ctlb!(EG, _ctlb!(Atom, "p"))));
        assert!(verify(&model, &"s0", &_ctlb!(EG, _ctlb!(Not, _ctlb!(Atom, "q")))));
        assert!(!verify(&model, &"s0", &_ctlb!(EG, _ctlb!(And, _ctlb!(Atom, "p"), _ctlb!(Atom, "q")))));
    }

    #[test]
    fn check_ag() {
        let model = DiscreteModel::new(
            vec![
                ("s0", vec!["p"]),
                ("s1", vec!["p"]),
                ("s2", vec![]),
                ("s3", vec!["p", "q"]),
            ],
            vec![
                ("s0", vec!["s1"]),
                ("s1", vec!["s2", "s3"]),
                ("s2", vec!["s0", "s3"]),
                ("s3", vec!["s3"]),
            ]
        );

        assert!(!verify(&model, &"s0", &_ctlb!(AG, _ctlb!(Atom, "p"))));
        assert!(verify(&model, &"s0", &_ctlb!(AG, _ctlb!(Imply, _ctlb!(Atom, "q"), _ctlb!(Atom, "p")))));
        assert!(verify(&model, &"s0", &_ctlb!(AG, _ctlb!(EX, _ctlb!(Atom, "p")))));
    }

    #[test]
    fn check_ef() {
        let model = DiscreteModel::new(
            vec![
                ("s0", vec!["p"]),
                ("s1", vec!["q"]),
                ("s2", vec!["p", "r"]),
                ("s3", vec!["q", "r"]),
            ],
            vec![
                ("s0", vec!["s1"]),
                ("s1", vec!["s0", "s3"]),
                ("s2", vec!["s3"]),
                ("s3", vec!["s2"]),
            ]
        );

        assert!(verify(&model, &"s0", &_ctlb!(EF, _ctlb!(AG, _ctlb!(Atom, "r")))));
        assert!(verify(&model, &"s0", &_ctlb!(EF, _ctlb!(AX, _ctlb!(Atom, "r")))));
        assert!(!verify(&model, &"s0", &_ctlb!(EF, _ctlb!(And, _ctlb!(Atom, "p"), _ctlb!(Atom, "q")))));
    }

    #[test]
    fn check_af() {
        let model = DiscreteModel::new(
            vec![
                ("s0", vec!["p"]),
                ("s1", vec!["p"]),
                ("s2", vec!["p", "r"]),
                ("s3", vec!["q", "r"]),
            ],
            vec![
                ("s0", vec!["s1"]),
                ("s1", vec!["s0", "s3"]),
                ("s2", vec!["s3"]),
                ("s3", vec!["s2"]),
            ]
        );

        assert!(!verify(&model, &"s0", &_ctlb!(AF, _ctlb!(Atom, "r"))));
        assert!(verify(&model, &"s0", &_ctlb!(AF, _ctlb!(Or, _ctlb!(EG, _ctlb!(Atom, "p")), _ctlb!(EG, _ctlb!(Atom, "r"))))));
        assert!(verify(&model, &"s0", &_ctlb!(AF, _ctlb!(Imply, _ctlb!(Atom, "r"), _ctlb!(Or, _ctlb!(Atom, "p"), _ctlb!(Atom, "q"))))));
    }

    #[test]
    fn check_eu() {
        let model = DiscreteModel::new(
            vec![
                ("s0", vec!["p", "r"]),
                ("s1", vec![]),
                ("s2", vec!["p"]),
                ("s3", vec!["p", "q"]),
            ],
            vec![
                ("s0", vec!["s1", "s2"]),
                ("s1", vec!["s3"]),
                ("s2", vec!["s2", "s3"]),
                ("s3", vec!["s0"]),
            ]
        );

        assert!(verify(&model, &"s0", &_ctlb!(EU, _ctlb!(Atom, "p"), _ctlb!(Atom, "q"))));
        assert!(verify(&model, &"s0", &_ctlb!(EU, _ctlb!(Atom, "p"), _ctlb!(EG, _ctlb!(Atom, "p")))));
        assert!(!verify(&model, &"s0", &_ctlb!(EU, _ctlb!(And, _ctlb!(Atom, "p"), _ctlb!(Atom, "r")), _ctlb!(Atom, "q"))));
    }

    #[test]
    fn check_au() {
        let model = DiscreteModel::new(
            vec![
                ("s0", vec!["p"]),
                ("s1", vec!["p", "r"]),
                ("s2", vec!["q"]),
                ("s3", vec!["p", "q", "r"]),
            ],
            vec![
                ("s0", vec!["s1"]),
                ("s1", vec!["s2"]),
                ("s2", vec!["s2", "s3"]),
                ("s3", vec!["s1"]),
            ]
        );

        assert!(verify(&model, &"s0", &_ctlb!(AU, _ctlb!(Atom, "p"), _ctlb!(Atom, "q"))));
        assert!(verify(&model, &"s0", &_ctlb!(AU, _ctlb!(Atom, "p"), _ctlb!(AG, _ctlb!(Or, _ctlb!(Atom, "r"), _ctlb!(Atom, "q"))))));
        assert!(!verify(&model, &"s0", &_ctlb!(AU, _ctlb!(Atom, "p"), _ctlb!(Atom, "r"))));
    }

    #[test]
    fn testing() {
        let x : CTLFormula<i32> = ctl!(And(Atom(5), AX(Atom(3))));

        println!("{}", x);
    }

}
