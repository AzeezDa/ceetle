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
        CTLFormula::AG(subformula) => check_all_paths(model, state, subformula),
        CTLFormula::AF(subformula) => check_all_futures(model, state, subformula),
        CTLFormula::AX(subformula) => check_all_nexts(model, state, subformula),
        CTLFormula::AU(formula, until) => todo!(),
        CTLFormula::EG(subformula) => check_any_paths(model, state, subformula),
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
    let nexts = model.transitions(state);

    for next in nexts.iter() {
        if verify(model, next, &formula) {
            return true;
        }
    }

    return false;
}

fn check_all_paths<S: PartialEq, T: PartialEq>(
    model: &dyn Model<S, T>,
    state: &S,
    formula: &CTLFormula<T>,
) -> bool {
    if !verify(model, state, formula) {
        return false;
    }

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

        for i in model.transitions(state).iter() {
            queue.push_back(i);
        }
        visited.push(state);
    }

    return true;
}

fn check_any_paths<S: PartialEq, T: PartialEq>(
    model: &dyn Model<S, T>,
    state: &S,
    formula: &CTLFormula<T>,
) -> bool {
    if !verify(model, state, formula) {
        return false;
    }

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

    let mut visited: Vec<&S> = Vec::from([state]);
    let mut queue: VecDeque<&S> = VecDeque::new();

    for i in model.transitions(state).iter() {
        queue.push_back(i);
    }

    while let Some(state) = queue.pop_front() {
        if verify(model, state, formula) {
            return true;
        }
        for i in model.transitions(state).iter() {
            if !visited.contains(&i) {
                queue.push_back(i);
            }
        }
        visited.push(state);
    }

    return false;
}

fn check_all_futures<S: PartialEq, T: PartialEq>(
    model: &dyn Model<S, T>,
    state: &S,
    formula: &CTLFormula<T>,
) -> bool {
    if verify(model, state, formula) {
        return true;
    }

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

    let mut visited: Vec<&S> = Vec::from([state]);
    let mut queue: VecDeque<&S> = VecDeque::new();

    for i in model.transitions(state).iter() {
        if verify(model, i, until) {
            return true;
        }
        if verify(model, i, formula) {
            queue.push_back(i);
        }
    }

    while let Some(state) = queue.pop_front() {        
        for i in model.transitions(state).iter() {
            if verify(model, i, until) {
                return true;
            }
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

    let mut visited: Vec<&S> = Vec::from([state]);
    let mut queue: VecDeque<&S> = VecDeque::new();

    for i in model.transitions(state).iter() {
        let formula_false = !verify(model, i, formula);
        if !verify(model, i, until) && formula_false {
            return false;
        }
        if formula_false {
            queue.push_back(i);
        }
    }

    while let Some(state) = queue.pop_front() {
        if visited.contains(&state) {
            return false;
        }
        for i in model.transitions(state).iter() {
            let formula_false = !verify(model, i, formula);
            if !verify(model, i, until) && formula_false {
                return false;
            }
            if formula_false {
                queue.push_back(&i);
            }
        }
        visited.push(state);
    }

    return true;
}