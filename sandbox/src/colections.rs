let mut v: Vec<i32> = Vec::new();
v.push(2);
v.push(4);
let x= v.pop();
println!("{}", x);

let mut v = vec![1,2,3,4]

=== 

HashMap<K, V>

let mut h: HashMap< u8,bool > = HashMap::new();
h.insert(5,true);
h.insert(6,false);
let have_five = h.remove(&5).unwrap();

// other collections 
// VecDeque(2 ended qoueue)     HashSet     BtreeMap     LinkedList()   BinaryHeap  BTreSet  

enum Color {
  Red,
  Green,
  Black
}

enum DispenseItem {
  Empty,
  Ammo (u8),
  Things (String, i32),
  Place {x: i32, y:i32},
}

use DispencerItem::* ;

let Item = Empty;

let Item = Ammo(69);


enum Option<T> {
  Some(T),
  None,
}

if let Some(x) = my_var {
  println!("value is {}",x);
}

match my_var{
  Some(x) => {
    println!("value is {}", x);
  },
  None => {
    println!("no value");
  }, 
}

match my_var {
  _ => {
    println!("who cares");
  },
}

// if there return value {}; should be if not {}


// Option

let mut x: Option<i32> = None;
x = Some(5);

x.is_none();
x.is_some();

for i in x {
  println!("{}", i); 
}

// handle errors Result is
#[must_use]
enum Result<T, E> {
  Ok(T),
  Err(E),
}

use std::fs::File;

fn main() {
  let res = File::open("foo");
  let f = res.unwrap();
  // or let f = res.expect("error mesg");
}

fn main() {
  let res = File::open("foo");
  if res.is_ok() {
    let f = res.unwrap();
  }
}

fn main() {
  let res = File::open("foo");
  match res { 
    Ok(f) => { /* do stuff */},
    Err(e) => {/* do stuff */},
  }
}
