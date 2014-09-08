extern crate time;
extern crate core;
extern crate knuckle;

use std::collections::HashSet;

mod transaction;
mod config;

fn main() {
//  let kp = config::load_key_pair();
  let mut ledger = HashSet::new();

  let first_transaction = transaction::Transaction::new("Alice","Bob",Vec::from_slice(b"Red Stamp"));
  ledger.insert(first_transaction);

  for entry in ledger.iter() {
    println!("{}", *entry);
  }
}
