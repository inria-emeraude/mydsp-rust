# mydsp-rust
Rust version of mydsp library for basic audio processing/synthesis

This library is developed with the aim of helping to set up a [5th year course option](https://5tc-rust-cc1d61.gitlabpages.inria.fr/) 
on introduction to  Rust 
([@INSA Lyon - Département Télécommunications, Services et Usages](https://telecom.insa-lyon.fr/) 
&& [@Emeraude Research Team](https://team.inria.fr/emeraude/)).

The [crate `mydsp-jack`](https://github.com/Longrui-Ma/mydsp-jack) was first developped by Longrui Ma adapting the functionalities found in 
[the original C++ `mydsp` library](https://github.com/grame-cncm/embaudio/tree/master/examples/teensy/libraries/mydsp) to Jack and Rust.

# How to use 
[`mydsp-rust-example` crate]{https://github.com/inria-emeraude/mydsp-rust-example} show examples of using the 'mydsp-rust' crate with jack


# Notes (j'ai gardé ca mais il faudra l'enlever je pense)
## `impl ProcessHandler for Patch` in user's binary crate
Because we want the modules to be connected in a sequential manner rather than [nested](https://github.com/Longrui-Ma/mydsp-jack-old), 
`impl ProcessHandler for Patch` (`ProcessHandler` trait defined in `jack-rust`) 
cannot be implemented in the `mydsp-jack` library crate, 
it can only be implemented in the user's binary crate by defining a new struct `Patch`. (**orphan rule**) 
So the final result is that `mydsp-jack` is completely independent of `jack-rust`.