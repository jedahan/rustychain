extern crate time;
extern crate core;

use std::collections::HashSet;

mod transaction;
mod config;

fn main() {
  let (public,secret) = config::load_key_pair();
  let mut ledger = HashSet::new();

  let first_transaction = transaction::new("Alice","Bob",Vec::from_slice(b"Red Stamp"), &secret);
  ledger.insert(first_transaction);

  for entry in ledger.iter() {
    println!("{}", *entry);
  }
}
