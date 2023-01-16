use std::fmt;

#[macro_export]
macro_rules! ctl {
    (True) => {ceetle::CTLFormula::True};
    (False) => {ceetle::CTLFormula::False};
    (Atom, $e1:expr) => {ceetle::CTLFormula::Atom($e1)};
    ($t:tt, $e1:expr) => {ceetle::CTLFormula::$t(Box::new($e1))};
    ($t:tt, $e1:expr, $e2:expr) => {ceetle::CTLFormula::$t(Box::new($e1), Box::new($e2))};
}

// Used for testing and debugging
#[macro_export]
macro_rules! _ctlb {
    (True) => {CTLFormula::True};
    (False) => {CTLFormula::False};
    (Atom, $e1:expr) => {CTLFormula::Atom($e1)};
    ($t:tt, $e1:expr) => {CTLFormula::$t(Box::new($e1))};
    ($t:tt, $e1:expr, $e2:expr) => {CTLFormula::$t(Box::new($e1), Box::new($e2))};
}

pub enum CTLFormula<T: PartialEq> {
    True,
    False,
    Atom(T),
    And(Box<CTLFormula<T>>, Box<CTLFormula<T>>),
    Or(Box<CTLFormula<T>>, Box<CTLFormula<T>>),
    Not(Box<CTLFormula<T>>),
    Imply(Box<CTLFormula<T>>, Box<CTLFormula<T>>),
    AG(Box<CTLFormula<T>>),
    AF(Box<CTLFormula<T>>),
    AX(Box<CTLFormula<T>>),
    AU(Box<CTLFormula<T>>, Box<CTLFormula<T>>),
    EG(Box<CTLFormula<T>>),
    EF(Box<CTLFormula<T>>),
    EX(Box<CTLFormula<T>>),
    EU(Box<CTLFormula<T>>, Box<CTLFormula<T>>),
}

impl<T: fmt::Display + PartialEq> fmt::Display for CTLFormula<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::True => {
                write!(f, "⊤")
            }
            Self::False => {
                write!(f, "⊥")
            }
            Self::Atom(value) => {
                write!(f, "{}", value)
            }
            Self::And(value, value2) => {
                write!(f, "({}∧{})", value, value2)
            }
            Self::Or(value, value2) => {
                write!(f, "({}∨{})", value, value2)
            }
            Self::Not(value) => {
                write!(f, "¬{}", value)
            }
            Self::Imply(value, value2) => {
                write!(f, "({}→{})", value, value2)
            }
            Self::AG(value) => {
                write!(f, "AG{}", value)
            }
            Self::AF(value) => {
                write!(f, "AF{}", value)
            }
            Self::AX(value) => {
                write!(f, "AX{}", value)
            }
            Self::AU(value, value2) => {
                write!(f, "A[{} U {}]", value, value2)
            }
            Self::EG(value) => {
                write!(f, "EG{}", value)
            }
            Self::EF(value) => {
                write!(f, "EF{}", value)
            }
            Self::EX(value) => {
                write!(f, "EX{}", value)
            }
            Self::EU(value, value2) => {
                write!(f, "E[{} U {}]", value, value2)
            }
        }
    }
}
