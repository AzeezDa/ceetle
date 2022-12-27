use ceetle::ctl;

pub fn main() {
    let x = ctl!(AX(AG(2)));

    println!("{}", x);
}