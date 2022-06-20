closure is anononym fn 

| x, y| { x + y};

||{}

let s = "🐙".to_string();
let f = || {
  println!( "{}", s );
};


f(); // prints 🐙


let s = "🐙".to_string();
let f = move || {
  println!( "{}", s );
};


f(); // prints 🐙


let mut v = vec![2,4,6];

v.iter()
  .map( | x | x*3)
  .filter( |x| *x > 10)
  .fold(0, |acc, x| acc + x); // reduce?