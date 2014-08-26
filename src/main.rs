extern crate time;
extern crate core;

use std::collections::HashSet;

mod transaction;
mod config;

fn main() {
  let (public,secret) = config::load_key_pair();
  println("loaded keypair\nPUBLIC\n{}\nSECRET\n{}\n", public, secret);
  let mut ledger = HashSet::new();

  let first_transaction = Transaction::new("Alice","Bob",Vec::from_slice(b"Red Stamp"));
  ledger.insert(first_transaction);

  for entry in ledger.iter() {
    println!("{}", *entry);
  }
}

/*
    hash the transactionHeader
    decrypt the transaction.signature using transactionHeader.from
    if the descrypted signature does not start with 0000 FAIL:
      Bad proof of work
    if the descrypted signature does not match the transactionHeader hash FAIL:
      Transaction header hash does not match the hash found in the signature
    hash the payload
    if payload hash != transactionHeader hash FAIL:
      Payload hash does not match hash in header

    yay! everything passes! add to block!
*/
