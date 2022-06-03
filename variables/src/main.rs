
const STARTING_MISIILES: i32 = 8;
const READY_AMMOUNT: i32 = 2;

fn main() {

    let mut missiles = STARTING_MISIILES; 
    let ready = READY_AMMOUNT;


    println!("Fired {} of my {} missiles.", ready,missiles);

    missiles = missiles - ready;

    println!("{} missiles left", missiles);
}
