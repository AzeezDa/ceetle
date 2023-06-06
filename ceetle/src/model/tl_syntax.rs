use std::fmt;

/// # `CTLFormula`
/// `CTLFormula` represents a Computional Tree Logic formula that is used to verify models. 
/// It will almost always be easier to generate `CTLFormula`s using the `ctl` macro.
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

// Formatting for println!("{}")
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
