use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::{Arc, Mutex};
use std::thread;

pub mod bank;
struct Ledger {
    from: i32,
    to: i32,
    amount: i32,
    mode: Mode,
    ledger_id: i32,
}

enum Mode {
    D,
    W,
    T,
}

struct LedgerList {
    ledgers: Arc<Mutex<Vec<Ledger>>>,
    bank: Arc<bank::Bank>,
}

pub fn init_bank(num_readers: i32,num_workers: i32, ledger_file: &str) {
    //Load the ledger file
    let ledger_list = load_ledger(num_readers,ledger_file);
    //Print the initial account balances
    ledger_list.bank.print_account();
    //Set up a vector/list of threads
    let mut threads = Vec::new();
    //Execute the tasks for each worker by spawning threads
    for i in 0..num_workers{
        let ledger_list_clone = Arc::clone(&ledger_list);
        threads.push(thread::spawn(move || {
            worker(i as i32, ledger_list_clone);
        }));
    }
    //Join all the threads
    for thread in threads {
        thread.join().unwrap();
    }
    ledger_list.bank.print_account();
}

fn load_ledger(num_threads: i32, filename: &str) -> Arc<LedgerList> { 
    //Start with a new ledger list
    let ledger_list = Arc::new(LedgerList {
        ledgers: Arc::new(Mutex::new(Vec::new())),
        bank: Arc::new(bank::Bank::new()),
    });
    
    let lines = Arc::new(Mutex::new(Vec::new()));
    //Open the file and handle invalid file name
    let file = match File::open(filename){
        Ok(file) => file,
        Err(err) => {
            println!("My error message: {}", err);
            std::process::exit(1);
        }
    };
    //A buffered reader to read the file line by line
    let reader = BufReader::new(file);
    //Push the lines into the vector
    let mut index = 0;
    for line in reader.lines() {
        lines.lock().unwrap().push((line.unwrap(), Arc::new(Mutex::new(index))));
        index += 1;
    }

    //Set up a vector/list of threads
    let mut threads = Vec::new();
    
    //Execute the tasks for each worker by spawning threads
    for i in 0..num_threads {
        let lines_clone = Arc::clone(&lines);
        let ledger_list_clone = Arc::clone(&ledger_list);
        threads.push(thread::spawn(move || {
            read_file(i,lines_clone, ledger_list_clone);
        }));
    }
    //Join all the reader threads
    for thread in threads {
        thread.join().unwrap();
    }
    ledger_list
}

//Reader thread helper function
fn read_file (thread_num: i32, lines : Arc<Mutex<Vec<(String,Arc<Mutex<i32>>)>>>, ledger_list: Arc<LedgerList>) {
    while lines.lock().unwrap().len() > 0 {
        let mut guard = lines.lock().unwrap();
        if guard.len() != 0 {
            let line = guard.remove(0);
            let num_line = line.1.lock().unwrap().clone();
            drop(guard);
            let mut split = line.0.split_whitespace();
            let from = split.next().unwrap().parse().unwrap();
            let to = split.next().unwrap().parse().unwrap();
            let amount = split.next().unwrap().parse().unwrap();
            let mode = match split.next().unwrap().parse().unwrap() {
                0 => Mode::D,
                1 => Mode::W,
                2 => Mode::T,
                _ => panic!("Invalid mode"),
            };
            let ledger = Ledger {
                from: from,
                to: to,
                amount: amount,
                mode: mode,
                ledger_id: num_line,
            };
            ledger_list.ledgers.lock().unwrap().push(ledger);
            let guard = line.1.lock().unwrap();
            print!("Reader {} has finished processing line {}\n", thread_num, guard.clone());
            print!("");
            drop(guard);
            //Note: This print!("") macro somehow stalls the program. I have no idea why.
            //If you comment them out, the program will run fine, but the output will sometimes 
            //look like a thread not letting other threads take the next line and process it.
            //The print macro kinda stalls the current thread so that other threads can take over.
            //It could be that I'm locking incorrectly, but I'm not sure.
        } else {
            drop(guard);
        }
    }
}

fn worker(workerid: i32, ledger_list: Arc<LedgerList>){
    while ledger_list.ledgers.lock().unwrap().len() > 0 {
        let mut guard = ledger_list.ledgers.lock().unwrap();
        //Check if the ledger is empty because another worker took over. If so, just skip.
        if guard.len() != 0{
            let ledger = guard.remove(0);
            drop(guard);
            //Perform the transaction
            match ledger.mode {
                Mode::D => { 
                    ledger_list.bank.deposit(workerid, ledger.ledger_id, ledger.from, ledger.amount);
                },
                Mode::W => {
                    ledger_list.bank.withdraw(workerid, ledger.ledger_id, ledger.from, ledger.amount);
                },
                Mode::T => {
                    ledger_list.bank.transfer(workerid, ledger.ledger_id, ledger.from, ledger.to, ledger.amount);
                }
            }
        } else {
            drop(guard);
        }
    }  
}
