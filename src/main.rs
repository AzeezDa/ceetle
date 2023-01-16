use ceetle::ctl;

pub fn main() {
    let x = ctl!(AX, ctl!(Atom, 2));

    println!("{}", x);
}