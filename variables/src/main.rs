
const STARTING_MISIILES: i32 = 8;
const READY_AMMOUNT: i32 = 2;

fn main() {

    let (missiles, ready)  = (STARTING_MISIILES, READY_AMMOUNT);

    println!("Fired {} of my {} missiles.", ready,missiles);

    println!("{} missiles left", missiles - ready);
  
}
