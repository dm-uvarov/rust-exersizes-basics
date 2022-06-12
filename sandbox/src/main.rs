// importing if we need to use fucntion without  '::' scope operator 
use sandbox::greet;

// standard library functions colletcions
// rust std vectoror smthelse
// rust community crate registry crate.io
use std::collections::HashMap;


// if library name in depencies in cargo.toml we can use it ether directly or as with operator use

fn main() {
    // works directly
    sandbox::greet();

    // or with importing  of 'use sandbox::greet'
    greet();
}
