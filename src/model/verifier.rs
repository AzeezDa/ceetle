use std::collections::VecDeque;

use super::{CTLFormula, Model};

pub fn verify<S: PartialEq, T: PartialEq>(
    model: &dyn Model<S, T>,
    state: &S,
    formula: &CTLFormula<T>,
) -> bool {
    match formula {
        CTLFormula::True => true,
        CTLFormula::False => false,
        CTLFormula::Atom(atom) => model.state_has(&state, &atom),
        CTLFormula::And(formula1, formula2) => {
            verify(model, state, formula1) && verify(model, state, formula2)
        }
        CTLFormula::Or(formula1, formula2) => {
            verify(model, state, formula1) || verify(model, state, formula2)
        }
        CTLFormula::Not(formula) => !verify(model, state, formula),
        CTLFormula::Imply(formula1, formula2) => {
            !verify(model, state, formula1) || verify(model, state, formula2)
        }
        CTLFormula::AG(subformula) => check_all_paths_global(model, state, subformula),
        CTLFormula::AF(subformula) => check_all_future(model, state, subformula),
        CTLFormula::AX(subformula) => check_all_nexts(model, state, subformula),
        CTLFormula::AU(formula, until) => check_all_until(model, state, formula, until),
        CTLFormula::EG(subformula) => check_any_path_global(model, state, subformula),
        CTLFormula::EF(subformula) => check_any_future(model, state, subformula),
        CTLFormula::EX(subformula) => check_any_next(model, state, subformula),
        CTLFormula::EU(formula, until) => check_any_until(model, state, formula, until),
    }
}

fn check_all_nexts<S: PartialEq, T: PartialEq>(
    model: &dyn Model<S, T>,
    state: &S,
    formula: &CTLFormula<T>,
) -> bool {
    // Iter of next states
    let nexts = model.transitions(state);

    for next in nexts.iter() {
        if !verify(model, next, &formula) {
            return false;
        }
    }

    return true;
}

fn check_any_next<S: PartialEq, T: PartialEq>(
    model: &dyn Model<S, T>,
    state: &S,
    formula: &CTLFormula<T>,
) -> bool {
    // Iter of next states
    let nexts = model.transitions(state);

    for next in nexts.iter() {
        if verify(model, next, &formula) {
            return true;
        }
    }

    return false;
}

fn check_all_paths_global<S: PartialEq, T: PartialEq>(
    model: &dyn Model<S, T>,
    state: &S,
    formula: &CTLFormula<T>,
) -> bool {
    if !verify(model, state, formula) {
        return false;
    }

    // ============================================================
    //                        Algorithm
    // BFS that returns false if the formula is not valid for some
    // state
    // ============================================================
    let mut visited: Vec<&S> = Vec::from([state]);
    let mut queue: VecDeque<&S> = VecDeque::new();

    for i in model.transitions(state).iter() {
        queue.push_back(i);
    }

    while let Some(state) = queue.pop_front() {
        if visited.contains(&state) {
            continue;
        }

        if !verify(model, state, formula) {
            return false;
        }

        // Add next states of current state
        for i in model.transitions(state).iter() {
            queue.push_back(i);
        }

        visited.push(state);
    }

    return true;
}

fn check_any_path_global<S: PartialEq, T: PartialEq>(
    model: &dyn Model<S, T>,
    state: &S,
    formula: &CTLFormula<T>,
) -> bool {
    if !verify(model, state, formula) {
        return false;
    }

    // ============================================================
    //                        Algorithm
    // BFS that returns true if it finds a loop where the formula
    // is valid for all states along that path
    // ============================================================
    let mut visited: Vec<&S> = Vec::from([state]);
    let mut queue: VecDeque<&S> = VecDeque::new();

    for i in model.transitions(state).iter() {
        if verify(model, i, formula) {
            queue.push_back(i);
        }
    }

    while let Some(state) = queue.pop_front() {
        if visited.contains(&state) {
            return true;
        }

        // Add next states of current state
        for i in model.transitions(state).iter() {
            if verify(model, i, formula) {
                queue.push_back(i);
            }
        }

        visited.push(state);
    }

    return true;
}

fn check_any_future<S: PartialEq, T: PartialEq>(
    model: &dyn Model<S, T>,
    state: &S,
    formula: &CTLFormula<T>,
) -> bool {
    if verify(model, state, formula) {
        return true;
    }

    // ============================================================
    //                        Algorithm
    // BFS that returns true if the formula is valid for some state
    // ============================================================
    let mut visited: Vec<&S> = Vec::from([state]);
    let mut queue: VecDeque<&S> = VecDeque::new();

    for i in model.transitions(state).iter() {
        queue.push_back(i);
    }

    while let Some(state) = queue.pop_front() {
        if verify(model, state, formula) {
            return true;
        }

        // Add next states of current state
        for i in model.transitions(state).iter() {
            if !visited.contains(&i) {
                queue.push_back(i);
            }
        }

        visited.push(state);
    }

    return false;
}

fn check_all_future<S: PartialEq, T: PartialEq>(
    model: &dyn Model<S, T>,
    state: &S,
    formula: &CTLFormula<T>,
) -> bool {
    if verify(model, state, formula) {
        return true;
    }

    // ============================================================
    //                        Algorithm
    // BFS that returns false if there is a loop where the formula
    // is false in all states along side that path
    // ============================================================
    let mut visited: Vec<&S> = Vec::from([state]);
    let mut queue: VecDeque<&S> = VecDeque::new();

    for i in model.transitions(state).iter() {
        if !verify(model, i, formula) {
            queue.push_back(i);
        }
    }

    while let Some(state) = queue.pop_front() {
        if visited.contains(&state) {
            return false;
        }

        // Add next states of current state where the formula is invalid
        for i in model.transitions(state).iter() {
            if !verify(model, i, formula) {
                queue.push_back(&i);
            }
        }

        visited.push(state);
    }

    return true;
}

fn check_any_until<S: PartialEq, T: PartialEq>(
    model: &dyn Model<S, T>,
    state: &S,
    formula: &CTLFormula<T>,
    until: &CTLFormula<T>,
) -> bool {
    if !verify(model, state, formula) {
        return false;
    }
    if verify(model, state, until) {
        return true;
    }

    // ============================================================
    //                        Algorithm
    // BFS that returns true if it finds a state where the until
    // formula is valid and where the first formula holds for all
    // states along that path until the last
    // ============================================================
    let mut visited: Vec<&S> = Vec::from([state]);
    let mut queue: VecDeque<&S> = VecDeque::new();

    for i in model.transitions(state).iter() {
        // We found a path already!
        if verify(model, i, until) {
            return true;
        }
        if verify(model, i, formula) {
            queue.push_back(i);
        }
    }

    while let Some(state) = queue.pop_front() {
        if visited.contains(&state) {
            continue;
        }

        for i in model.transitions(state).iter() {
            // We found a path where until holds
            if verify(model, i, until) {
                return true;
            }

            // Add next states of current state
            if verify(model, i, formula) {
                queue.push_back(i);
            }
        }

        visited.push(state);
    }

    return false;
}

fn check_all_until<S: PartialEq, T: PartialEq>(
    model: &dyn Model<S, T>,
    state: &S,
    formula: &CTLFormula<T>,
    until: &CTLFormula<T>,
) -> bool {
    if !verify(model, state, formula) {
        return false;
    }
    if verify(model, state, until) {
        return true;
    }

    // ============================================================
    //                        Algorithm
    // BFS that returns false if:
    //      1. It finds a loop where the first formula only holds
    //         along that loop
    //      2. It finds a state where the first and until formula
    //         do not hold
    // ============================================================
    let mut visited: Vec<&S> = Vec::from([state]);
    let mut queue: VecDeque<&S> = VecDeque::new();

    for i in model.transitions(state).iter() {
        let first_formula_holds = verify(model, i, formula);

        // If neither until or first formulas are true then return false
        if !verify(model, i, until) && !first_formula_holds {
            return false;
        }

        if first_formula_holds {
            queue.push_back(i);
        }
    }

    while let Some(state) = queue.pop_front() {
        if visited.contains(&state) {
            return false;
        }

        for i in model.transitions(state).iter() {
            let first_formula_holds = verify(model, i, formula);

            if !verify(model, i, until) && !first_formula_holds {
                return false;
            }

            if first_formula_holds {
                queue.push_back(&i);
            }
        }

        visited.push(state);
    }

    return true;
}
