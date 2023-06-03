mod model;
pub use model::*;

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, vec};

    use super::*;
    use crate::model::{verify, CTLFormula, VecDiscreteModel, HashedDiscreteModel, Model};

    #[test]
    fn create_and_check_discrete_model() {
        let model = VecDiscreteModel::new(vec![
            ("s0", vec!["a"], vec!["s0", "s1"]),
            ("s1", vec!["a", "b"], vec!["s0", "s2"]),
            ("s2", vec!["a"], vec!["s3"]),
            ("s3", vec!["a", "c"], vec!["s1", "s3"]),
        ]);

        assert!(model.state_has(&"s0", &"a"));
        assert!(!model.state_has(&"s3", &"b"));
        assert_eq!(*model.transitions(&"s1"), vec!["s0", "s2"]);
    }

    #[test]
    fn create_and_check_hashed_discrete_model() {
        let mut states = HashMap::new();
        states.insert("s0", (vec!["a"], vec!["s0", "s1"]));
        states.insert("s1", (vec!["a", "b"], vec!["s0", "s2"]));
        states.insert("s2", (vec!["a"], vec!["s3"]));
        states.insert("s3", (vec!["a", "c"], vec!["s1", "s3"]));

        let model = HashedDiscreteModel::new(states);
        assert!(model.state_has(&"s0", &"a"));
        assert!(!model.state_has(&"s3", &"b"));
        assert_eq!(*model.transitions(&"s1"), vec!["s0", "s2"]);
    }

    #[test]
    fn check_ex() {
        let model = VecDiscreteModel::new(vec![
            ("s0", vec![], vec!["s1", "s2"]),
            ("s1", vec!["p"], vec!["s1", "s2", "s3"]),
            ("s2", vec!["q"], vec!["s1"]),
            ("s3", vec!["p", "q"], vec!["s0"]),
        ]);

        assert!(verify(&model, &"s0", &ctl!(EX(Atom("p")))));
        assert!(!verify(&model, &"s0", &ctl!(EX(And(Atom("p"), Atom("q"))))));
        assert!(verify(
            &model,
            &"s0",
            &ctl!(EX(EX(And(Atom("p"), Atom("q")))))
        ));
    }

    #[test]
    fn check_ax() {
        let model = VecDiscreteModel::new(vec![
            ("s0", vec![], vec!["s1", "s2"]),
            ("s1", vec!["p"], vec!["s1", "s2", "s3"]),
            ("s2", vec!["q"], vec!["s1"]),
            ("s3", vec!["p", "q"], vec!["s0"]),
        ]);

        assert!(verify(&model, &"s0", &ctl!(AX(Or(Atom("p"), Atom("q"))))));
        assert!(!verify(
            &model,
            &"s0",
            &ctl!(AX(AX(AX(Or(Atom("p"), Atom("q"))))))
        ));
        assert!(verify(
            &model,
            &"s0",
            &ctl!(AX(Imply(Atom("p"), Not(Atom("q")))))
        ));
    }

    #[test]
    fn check_eg() {
        let model = VecDiscreteModel::new(vec![
            ("s0", vec!["p"], vec!["s1"]),
            ("s1", vec!["p"], vec!["s2", "s3"]),
            ("s2", vec![], vec!["s0", "s3"]),
            ("s3", vec!["p", "q"], vec!["s3"]),
        ]);

        assert!(verify(&model, &"s0", &ctl!(EG(Atom("p")))));
        assert!(verify(&model, &"s0", &ctl!(EG(Not(Atom("q"))))));
        assert!(!verify(&model, &"s0", &ctl!(EG(And(Atom("p"), Atom("q"))))));
    }

    #[test]
    fn check_ag() {
        let model = VecDiscreteModel::new(vec![
            ("s0", vec!["p"], vec!["s1"]),
            ("s1", vec!["p"], vec!["s2", "s3"]),
            ("s2", vec![], vec!["s0", "s3"]),
            ("s3", vec!["p", "q"], vec!["s3"]),
        ]);

        assert!(!verify(&model, &"s0", &ctl!(AG(Atom("p")))));
        assert!(verify(
            &model,
            &"s0",
            &ctl!(AG(Imply(Atom("q"), Atom("p"))))
        ));
        assert!(verify(&model, &"s0", &ctl!(AG(EX(Atom("p"))))));
    }

    #[test]
    fn check_ef() {
        let model = VecDiscreteModel::new(vec![
            ("s0", vec!["p"], vec!["s1"]),
            ("s1", vec!["q"], vec!["s0", "s3"]),
            ("s2", vec!["p", "r"], vec!["s3"]),
            ("s3", vec!["q", "r"], vec!["s2"]),
        ]);

        assert!(verify(&model, &"s0", &ctl!(EF(AG(Atom("r"))))));
        assert!(verify(&model, &"s0", &ctl!(EF(AX(Atom("r"))))));
        assert!(!verify(&model, &"s0", &ctl!(EF(And(Atom("p"), Atom("q"))))));
    }

    #[test]
    fn check_af() {
        let model = VecDiscreteModel::new(vec![
            ("s0", vec!["p"], vec!["s1"]),
            ("s1", vec!["p"], vec!["s0", "s3"]),
            ("s2", vec!["p", "r"], vec!["s3"]),
            ("s3", vec!["q", "r"], vec!["s2"]),
        ]);

        assert!(!verify(&model, &"s0", &ctl!(AF(Atom("r")))));
        assert!(verify(
            &model,
            &"s0",
            &ctl!(AF(Or(EG(Atom("p")), EG(Atom("r")))))
        ));
        assert!(verify(
            &model,
            &"s0",
            &ctl!(AF(Imply(Atom("r"), Or(Atom("p"), Atom("q")))))
        ));
    }

    #[test]
    fn check_eu() {
        let model = VecDiscreteModel::new(vec![
            ("s0", vec!["p", "r"], vec!["s1", "s2"]),
            ("s1", vec![], vec!["s3"]),
            ("s2", vec!["p"], vec!["s2", "s3"]),
            ("s3", vec!["p", "q"], vec!["s0"]),
        ]);

        assert!(verify(&model, &"s0", &ctl!(EU(Atom("p"), Atom("q")))));
        assert!(verify(&model, &"s0", &ctl!(EU(Atom("p"), EG(Atom("p"))))));
        assert!(!verify(
            &model,
            &"s0",
            &ctl!(EU(And(Atom("p"), Atom("r")), Atom("q")))
        ));
    }

    #[test]
    fn check_au() {
        let model = VecDiscreteModel::new(vec![
            ("s0", vec!["p"], vec!["s1"]),
            ("s1", vec!["p", "r"], vec!["s2"]),
            ("s2", vec!["q"], vec!["s2", "s3"]),
            ("s3", vec!["p", "q", "r"], vec!["s1"]),
        ]);

        assert!(verify(&model, &"s0", &ctl!(AU(Atom("p"), Atom("q")))));
        assert!(verify(
            &model,
            &"s0",
            &ctl!(AU(Atom("p"), AG(Or(Atom("r"), Atom("q")))))
        ));
        assert!(!verify(&model, &"s0", &ctl!(AU(Atom("p"), Atom("r")))));
    }

    #[test]
    fn readme_test() {
        use crate::{HashedDiscreteModel, ctl, CTLFormula, verify};

        let model = HashedDiscreteModel::new(HashMap::from_iter(vec![
            // (state, (atoms, transitions))
            ("s0", (vec!["p", "q"],      vec!["s1"])),
            ("s1", (vec!["p"],           vec!["s0", "s3"])),
            ("s2", (vec!["p", "q"],      vec!["s2"])),
            ("s3", (vec!["p", "r", "s"], vec!["s2"]))
        ]));

        assert!(verify(&model, &"s0", &ctl!(AG(Atom("p"))))); // Evaluates to true
        assert!(verify(&model, &"s0", &ctl!(EF(AG(And(Atom("p"), Atom("q"))))))); // Evaluates to true
    }
}
