pub mod ledger;
use crate::ledger::init_bank;

fn main() {
    //Read in arguments
    let args: Vec<String> = std::env::args().collect();
    //Check if the argument count is 4
    if args.len() != 4 {
        //If not, print an error message and exit
        println!("Usage: {} <num of readers> <num of workers> <leader_file>", args[0]);
        std::process::exit(1);
    }
    //Parse the first argument and second argument as a i32
    let num_workers: i32 = args[2].parse().unwrap();
    let num_readers: i32 = args[1].parse().unwrap();
    init_bank(num_readers,num_workers, &args[3]);

    std::process::exit(0);
}
