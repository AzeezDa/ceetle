# `ceetle` - A Computional Tree Logic Verifier
A Rust Library for defining models in Computational Tree Logic and verifying their semantics. See [Wikipedia](https://en.wikipedia.org/wiki/Computation_tree_logic) to learn more.

The library is **passively-maintained**, which means there will be no other features added however issues on the GitHub will be answered and solved.
Contributions and feedback to this library are more than welcome! 

## Examples
Consider the figure below. 

![Finite State Machine](/images/fsm.png)

To build it as a model in the library we do this:

```rust
use ceetle::{HashedDiscreteModel, ctl, CTLFormula, verify};

let model = HashedDiscreteModel::new(HashMap::from_iter(vec![
    // (state, (atoms, transitions))
    ("s0", (vec!["p", "q"],      vec!["s1"])),
    ("s1", (vec!["p"],           vec!["s0", "s3"])),
    ("s2", (vec!["p", "q"],      vec!["s2"])),
    ("s3", (vec!["p", "r", "s"], vec!["s2"]))
]));
```

To verify the formula $S_0\models \text{AG}(p)$, we do:
```rust
verify(&model, &"s0", &ctl!(AG(Atom("p")))); // Evaluates to true
```

To verify the formula $S_0 \models \text{EF(AG}(p \land q))$, we do:

```rust
verify(&model, &"s0", &ctl!(EF(AG(And(Atom("p"), Atom("q")))))); // Evaluates to true
```