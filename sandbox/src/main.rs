// importing if we need to use fucntion without  '::' scope operator 
use sandbox::greet;

// standard library functions colletcions
use std::collections::HashMap;


fn main() {
    // works directly
    sandbox::greet();

    // or with importing  of 'use sandbox::greet'
    greet();
}
