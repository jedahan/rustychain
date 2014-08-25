extern crate time;
extern crate core;
extern crate crypto = "rust-crypto";

use std::collections::HashSet;

use core::cmp::{Eq,PartialEq};
use std::hash::{Hash,hash};
use std::hash::sip::SipState;

use crypto::sha2::Sha256;
use crypto::digest::Digest;

type User = &'static str;

#[deriving(Eq,PartialEq,Show)]
struct TransactionHeader {
  from: User,
  to: User,
  time: time::Timespec,
  payload_hash: Vec<u8>,
  payload_length: uint,
  nonce: int
}

impl Hash for TransactionHeader {

  fn hash(&self, state: &mut SipState) {
    self.from.hash(state);
    self.to.hash(state);
    self.time.sec.hash(state);
    self.time.nsec.hash(state);
    self.payload_hash.hash(state);
    self.payload_length.hash(state);
    self.nonce.hash(state);
  }
}

#[deriving(Eq,PartialEq,Show,Hash)]
struct Transaction {
  header: TransactionHeader,
  payload: Vec<u8>,
  signature: Vec<u8>
}

fn mutate_transaction_header_until_proof_of_work_is_ok(header: &mut TransactionHeader, complexity: uint) {
  let mut try = 0i;

  loop {
      try += 1;
      header.nonce = try;

      // check that the first *complexity* bytes are 0
      if hash(header) < (1 << (64-(8*complexity))) {
        println!("we found {} after {} tries", hash(header), try);
        return;
      }
  }
}


fn makeTransaction(from: User, to: User, payload: Vec<u8>) -> TransactionHeader {
  /*let mut digest = Sha256::new();
  digest.input(payload);
*/
  let mut header = TransactionHeader {
      from: from,
      to: to,
      time: time::get_time(),
      payload_hash: vec![0],
      payload_length: payload.len(),
      nonce: 0
  };

  mutate_transaction_header_until_proof_of_work_is_ok(&mut header, 3);

  header
/*
  let signature = sign(header.hash());

  Transaction {
    header: header,
    payload: payload,
    signature: signature
  }
  */
}

fn main() {
  let mut ledger = HashSet::new();

  let first_transaction = makeTransaction("Alice","Bob",Vec::from_slice(b"Red Stamp"));
  ledger.insert(first_transaction);

  for entry in ledger.iter() {
    println!("{}", *entry);
  }
}


fn sign(hash: String) -> String {
  hash
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
