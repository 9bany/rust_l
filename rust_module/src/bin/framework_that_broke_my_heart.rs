
extern crate this_example; // oh right, I never named
// this crate, is the name you give in Cargo.toml
// under [package] in tha *name* field
// (don't use dashes on it)

use crate::something::a::*;
use crate::something::b::*;

fn main() {
    let first = A { a: 42, };
}
