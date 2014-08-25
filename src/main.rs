extern crate time;
extern crate core;
extern crate crypto = "rust-crypto";

use std::collections::HashSet;

use core::cmp::{Eq,PartialEq};
use std::hash::Hash;
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
  payload_length: Vec<u8>,
  nonce: int
}

impl Hash for TransactionHeader {

  fn hash(&self, state: &mut SipState) {

    let mut digest = Sha256::new();

      let transactionHeader = format!("{}{}{}{}{}{}{}",
        self.from,
        self.to,
        self.time.sec,
        self.time.nsec,
        self.payload_hash,
        self.payload_length,
        self.nonce);

    digest.input(transactionHeader.as_bytes());
    digest.result_str();
  }
}

#[deriving(Eq,PartialEq,Show)]
struct Transaction {
  header: TransactionHeader,
  payload: Vec<u8>,
  signature: Vec<u8>
}

impl Hash for Transaction {

  fn hash(&self, state: &mut SipState) {

    let mut digest = Sha256::new();

    let transaction = format!("{}{}",
      self.header,
      self.payload);

    digest.input(transaction.as_bytes());
    digest.result_str();
  }
}


fn mutateTransactionHeaderUntilProofOfWorkIsOk(header: TransactionHeader, complexity: uint) {
  let mut try = 0i;

  loop {
      try += 1;
      header.nonce = try;

      let s = String::from_char(complexity*2,'0');
      if header.hash().as_bytes().slice(0,complexity*2) == s.as_bytes().slice(0,complexity*2) {
        println!("we found {} after {} tries", header.hash(), try);
        return;
      }
  }
}


fn makeTransaction(from: User, to: User, payload: Vec<u8>) -> Transaction {
  let mut digest = Sha256::new();
  digest.input(payload);

  let mut header = TransactionHeader {
      from: from,
      to: to,
      time: time::get_time(),
      payload_hash: digest.result_str(),
      payload_length: payload.len(),
      nonce: 0
  };

  mutateTransactionHeaderUntilProofOfWorkIsOk(header, 2);

  let signature = sign(header.hash());

  Transaction {
    header: header,
    payload: payload,
    signature: signature
  }
}

fn main() {
  let mut ledger = HashSet::new();

  let first_transaction = makeTransaction("Alice","Bob","Red Stamp");
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
