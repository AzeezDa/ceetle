use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parenthesized, parse::Parse, parse_macro_input, Expr, Token};

macro_rules! process {
    ($keyword:tt in $input:tt as $out:ty) => {
        {
            $input.parse::<kw::$keyword>()?;
            let content;
            let _ = parenthesized!(content in $input);
            let val: $out = content.parse()?;
            Ok(CTLFormula::$keyword(val))
        }
    };
    ($keyword:tt in $input:tt as boxed $out:ty) => {
        {
            $input.parse::<kw::$keyword>()?;
            let content;
            let _ = parenthesized!(content in $input);
            let val: $out = content.parse()?;
            Ok(CTLFormula::$keyword(Box::new(val)))
        }
    };
    ($keyword:tt 2 in $input:tt as boxed $out:ty) => {
        {
            $input.parse::<kw::$keyword>()?;
            let content;
            let _ = parenthesized!(content in $input);
            let left: $out = content.parse()?;
            content.parse::<Token![,]>()?;
            let right: $out = content.parse()?;
            Ok(CTLFormula::$keyword(Box::new(left), Box::new(right)))
        }
    };
}

mod kw {
    use syn::custom_keyword;
    custom_keyword!(True);
    custom_keyword!(False);
    custom_keyword!(Atom);
    custom_keyword!(And);
    custom_keyword!(Or);
    custom_keyword!(Not);
    custom_keyword!(Imply);
    custom_keyword!(AG);
    custom_keyword!(AF);
    custom_keyword!(AX);
    custom_keyword!(AU);
    custom_keyword!(EG);
    custom_keyword!(EF);
    custom_keyword!(EX);
    custom_keyword!(EU);
}

enum CTLFormula {
    True,
    False,
    Atom(Expr),
    And(Box<CTLFormula>, Box<CTLFormula>),
    Or(Box<CTLFormula>, Box<CTLFormula>),
    Not(Box<CTLFormula>),
    Imply(Box<CTLFormula>, Box<CTLFormula>),
    AG(Box<CTLFormula>),
    AF(Box<CTLFormula>),
    AX(Box<CTLFormula>),
    AU(Box<CTLFormula>, Box<CTLFormula>),
    EG(Box<CTLFormula>),
    EF(Box<CTLFormula>),
    EX(Box<CTLFormula>),
    EU(Box<CTLFormula>, Box<CTLFormula>),
}

impl Parse for CTLFormula {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(kw::True) {
            input.parse::<kw::True>()?;
            return Ok(CTLFormula::True);
        } else if lookahead.peek(kw::False) {
            input.parse::<kw::True>()?;
            return Ok(CTLFormula::False);
        } else if lookahead.peek(kw::Atom) {
            return process!(Atom in input as Expr);
        } else if lookahead.peek(kw::Not) {
            return process!(Not in input as boxed CTLFormula);
        } else if lookahead.peek(kw::AG) {
            return process!(AG in input as boxed CTLFormula);
        } else if lookahead.peek(kw::AF) {
            return process!(AF in input as boxed CTLFormula);
        } else if lookahead.peek(kw::AX) {
            return process!(AX in input as boxed CTLFormula);
        } else if lookahead.peek(kw::EG) {
            return process!(EG in input as boxed CTLFormula);
        } else if lookahead.peek(kw::EF) {
            return process!(EF in input as boxed CTLFormula);
        } else if lookahead.peek(kw::EX) {
            return process!(EX in input as boxed CTLFormula);
        } else if lookahead.peek(kw::And) {
            return process!(And 2 in input as boxed CTLFormula);
        } else if lookahead.peek(kw::Or) {
            return process!(Or 2 in input as boxed CTLFormula);
        } else if lookahead.peek(kw::AU) {
            return process!(AU 2 in input as boxed CTLFormula);
        } else if lookahead.peek(kw::Imply) {
            return process!(Imply 2 in input as boxed CTLFormula);
        } else if lookahead.peek(kw::EU) {
            return process!(EU 2 in input as boxed CTLFormula);
        }

        Err(lookahead.error())
    }
}

/// # `ctl`
/// The `ctl` macro helps generating CTL formulas much easier than using the `CTLFormula`.
/// This macro uses the following syntax:
/// 
/// ```f = True | False | Atom(p) | Not(f) | AG(f) | AF(f) | AX(f) | EG(f) | EF(f) | EX(f) | And(f,f) | Or(f,f) | Imply(f,f) | AU(f,f) | EU(f,f)```
/// where `p` is any value.
/// 
/// ## Examples
/// ```
/// let f = ctl!(AX(Atom(5))); // Translates to "AX(5)"
/// let g = ctl!(And(Atom(1), AG(Atom(3)))); // Translates to "1 ∧ AG(3)"
/// let h = ctl!(Or(Imply(Atom(1), Atom(3)), EU(Atom(1), Atom(2)))); // Translates to "(1 → 3) ∨ E[1 U 2]""
/// ```
#[proc_macro]
pub fn ctl(_input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(_input as CTLFormula);
    get_ctl(input).into()
}

fn get_ctl(input: CTLFormula) -> TokenStream2 {
    let mut result = quote!();
    result = match input {
        CTLFormula::True => quote!(#result CTLFormula::True),
        CTLFormula::False => quote!(#result CTLFormula::False),
        CTLFormula::Atom(atom) => quote!(#result CTLFormula::Atom(#atom)),
        CTLFormula::Not(inner) => {
            let inner = get_ctl(*inner);
            quote!(#result CTLFormula::Not(Box::new(#inner)))
        }
        CTLFormula::AG(inner) => {
            let inner = get_ctl(*inner);
            quote!(#result CTLFormula::AG(Box::new(#inner)))
        }
        CTLFormula::AF(inner) => {
            let inner = get_ctl(*inner);
            quote!(#result CTLFormula::AF(Box::new(#inner)))
        }
        CTLFormula::AX(inner) => {
            let inner = get_ctl(*inner);
            quote!(#result CTLFormula::AX(Box::new(#inner)))
        }
        CTLFormula::EG(inner) => {
            let inner = get_ctl(*inner);
            quote!(#result CTLFormula::EG(Box::new(#inner)))
        }
        CTLFormula::EF(inner) => {
            let inner = get_ctl(*inner);
            quote!(#result CTLFormula::EF(Box::new(#inner)))
        }
        CTLFormula::EX(inner) => {
            let inner = get_ctl(*inner);
            quote!(#result CTLFormula::EX(Box::new(#inner)))
        }
        CTLFormula::And(left, right) => {
            let left = get_ctl(*left);
            let right = get_ctl(*right);
            quote!(#result CTLFormula::And(Box::new(#left), Box::new(#right)))
        }
        CTLFormula::Or(left, right) => {
            let left = get_ctl(*left);
            let right = get_ctl(*right);
            quote!(#result CTLFormula::Or(Box::new(#left), Box::new(#right)))
        }
        CTLFormula::AU(left, right) => {
            let left = get_ctl(*left);
            let right = get_ctl(*right);
            quote!(#result CTLFormula::AU(Box::new(#left), Box::new(#right)))
        }
        CTLFormula::Imply(left, right) => {
            let left = get_ctl(*left);
            let right = get_ctl(*right);
            quote!(#result CTLFormula::Imply(Box::new(#left), Box::new(#right)))
        }
        CTLFormula::EU(left, right) => {
            let left = get_ctl(*left);
            let right = get_ctl(*right);
            quote!(#result CTLFormula::EU(Box::new(#left), Box::new(#right)))
        }
    };
    result
}
