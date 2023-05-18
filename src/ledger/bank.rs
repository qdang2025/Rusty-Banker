use std::sync::{Mutex, Arc};

pub struct Account {
    balance: i32,
}

pub struct Bank {
    num_succ: Arc<Mutex<i32>>,
    num_fail: Arc<Mutex<i32>>,
    pub accounts: Arc<Mutex<Vec<Account>>>,
}

impl Bank {
    // Prints account information
    pub fn print_account(&self) {
        //For each account, print out the account id and balance
        for i in 0..10 {
            let guard = self.accounts.lock().unwrap();
            println!("Account #{}: {}", i, guard[i as usize].balance);
        }
        println!("Successes: {} Fails: {}", self.num_succ.lock().unwrap(), self.num_fail.lock().unwrap());
    }

    // Helper function to increment the bank variable `num_fail` and log message
    fn record_fail(&self, message: &str) {
        let mut _guard = self.num_fail.lock().unwrap();
        println!("{}", message);
        *_guard += 1;
    }

    // Helper function to increment the bank variable `num_succ` and log message
    fn record_succ(&self, message: &str) {
        let mut _guard = self.num_succ.lock().unwrap();
        println!("{}", message);
        *_guard += 1;
    }

    // Constructor to create a new Bank struct
    pub fn new() -> Self {    
        let mut accounts = Vec::new();
        for _i in 0..10{
            accounts.push(Account {
                balance: 0,
            });
        }
        Bank {
            num_succ: Arc::new(Mutex::new(0)),
            num_fail: Arc::new(Mutex::new(0)),
            accounts: Arc::new(Mutex::new(accounts)),
        }
    }

    // Adds money to an account
    pub fn deposit(&self, worker_id: i32, ledger_id: i32, account_id: i32, amount: i32) -> i32 {
        let mut _guard = self.accounts.lock().unwrap();
        _guard[account_id as usize].balance += amount;
        let message = format!("Worker {} completed ledger {}: deposit {} into account {}", worker_id, ledger_id, amount, account_id);
        drop(_guard);
        self.record_succ(&message);
        0
    }

    // Withdraws money from an account
    pub fn withdraw(&self, worker_id: i32, ledger_id: i32, account_id: i32, amount: i32) -> i32 {
        let mut _guard = self.accounts.lock().unwrap();
        let mut res = 0;
        if _guard[account_id as usize].balance >= amount {
            _guard[account_id as usize].balance -= amount;
            let message = format!("Worker {} completed ledger {}: withdraw {} from account {}", worker_id, ledger_id, amount, account_id);
            self.record_succ(&message);
        }
        else {
            let message = format!("Worker {} failed to complete ledger {}: withdraw {} from account {}", worker_id, ledger_id, amount, account_id);
            self.record_fail(&message);
            res = -1;
        }
        drop(_guard);
        res
    }

    // Transfers money from one account to another
    pub fn transfer (&self, worker_id: i32, ledger_id: i32, from_id: i32, to_id: i32, amount: i32) -> i32 {
        //Check if both accounts are the same
        if from_id == to_id {
            let message = format!("Worker {} failed to complete ledger {}: transfer {} from account {} to account {}", worker_id, ledger_id, amount, from_id, to_id);
            self.record_fail(&message);
            return -1;
        }
        
        //Lock the smaller account first
        let (from, to) = if from_id < to_id {
            (from_id, to_id)
        }
        else {
            (to_id, from_id)
        };
        
        let mut _guard = self.accounts.lock().unwrap();

        let mut res = 0;
        if _guard[from as usize].balance >= amount {
            _guard[from as usize].balance -= amount;
            _guard[to as usize].balance += amount;
            let message = format!("Worker {} completed ledger {}: transfer {} from account {} to account {}", worker_id, ledger_id, amount, from_id, to_id);
            self.record_succ(&message);
        }
        else {
            let message = format!("Worker {} failed to complete ledger {}: transfer {} from account {} to account {}", worker_id, ledger_id, amount, from_id, to_id);
            self.record_fail(&message);
            res = -1;
        }
        drop(_guard);
        res
    }
}

