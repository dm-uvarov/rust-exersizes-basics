struct RedFox{
  enemy: bool,
  life: u8,
};

impl RedFox {
  fn new() -> Self {  // or RedFox
    Self {
      enemy: true,
      life: 70,
    }
  }
}
let fox = RedFox::new();
let life_left = fox.life;
fox.enemy = false;
fox.some_method();

impl RedFox {
  // associated function for
  fn function() // ................
  // methods 
  // and methods always take self as first argument
  fn move()
  fn borrow(&self)
  fn mut_borrow(&mut self)
}

// rust OOP? why rust struct doesnt have inherinatce!!!
// instead rust has a traits

struct RedFox {
  enemy: bool,
  life: u32,
}

// required behavior analog of interface
// stryct Redfox requred trait noisy
trait Noisy {
  fn get_noise(&self) -> &str;
}

impl Noisy for RedFox {
  fn get_noise(&self) -> &str {
    "Meow?"
  }
}
// why not direct implement? 
// as 
impl RedFox {
  fn get_noise(&self) -> &str {
    "Meow?"
  }
}
// answer using traits allow us 
// implement any generic functions that 
// accepting any values impletenting the f_structs_traits

fn print_noise<T: Noisy>(item: T){
  println!("{}", item.get_noise());

}

impl Noisy for u8 {
  fn get_noise(&self) -> &str {"BYTE!"}
}

fn main() {
  print_noise(5_U8); // PRINTS BYTE!


}
 

/// special copy trait!
/// copied traist of objects small and can be puted in stack
/// not in heap
/// 
/// traits have an inheritance
/// 
/// traits also has default implementations
/// 
/// 

trait Run {
  fn run (&self) {
    println!("I'm running!")
  }
}
struct Robot {}
impl Run for Robot {}  // not overrride default method

fn main() {
  let robot = Robot{};
   robot.run();
}