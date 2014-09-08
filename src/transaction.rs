extern crate time;
extern crate core;

use core::cmp::{Eq,PartialEq};
use std::hash::{Hash,hash};
use std::hash::sip::SipState;

pub type User = &'static str;

#[deriving(Eq,PartialEq,Show,Hash)]
pub struct Transaction {
	header: TransactionHeader,
	payload: Vec<u8>,
	signature: u64
}

#[deriving(Eq,PartialEq,Show)]
pub struct TransactionHeader {
	from: User,
	to: User,
	time: time::Timespec,
	payload_hash: u64,
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

fn mutate_transaction_header_until_proof_of_work_is_ok(header: &mut TransactionHeader, complexity: uint) {
	let mut try = 0i;

	loop {
			try += 1;
			header.nonce = try;

			// check that the first *complexity* bytes are 0
			if hash(header) < (1 << (64-(4*complexity))) {
				println!("we found {:016x} after {} tries", hash(header), try);
				return;
			}
	}
}

impl Transaction {
    pub fn new(from: User, to: User, payload: Vec<u8>) -> Transaction {

        let mut header = TransactionHeader {
                from: from,
                to: to,
                time: time::get_time(),
                payload_hash: hash(&payload),
                payload_length: payload.len(),
                nonce: 0
        };

        mutate_transaction_header_until_proof_of_work_is_ok(&mut header, super::config::POW_DIFFICULTY);

        let hash = hash(&header);
        let signature = sign(hash);

        Transaction {
            header: header,
            payload: payload,
            signature: signature
        }
    }
}

fn sign(hash: u64) -> u64 {
	hash
}

#[test]
fn run_tests(){
    let t = Transaction::new("Alice", "Bob", Vec::from_slice(b"Red Stamp"));
    test_hash_shows_proof_of_work(&t);
}

#[cfg(test)]
fn test_hash_shows_proof_of_work(transaction: &Transaction){
    // soon, we will have to decode the signature first
    assert!(transaction.signature < (1 << (64 - 4*super::config::POW_DIFFICULTY)));
}
