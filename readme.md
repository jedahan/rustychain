## blockchain client in rust

inspired by [minimum viable blockchain](https://www.igvita.com/2014/05/05/minimum-viable-block-chain/)

### TODO

* hash the transactionHeader
* decrypt the transaction.signature using transactionHeader.from
* if the descrypted signature does not start with 0000 *FAIL: Bad proof of work*
* if the descrypted signature does not match the transactionHeader hash *FAIL: Transaction header has does not match signature*
* hash the payload
* if payload hash does not match transactionHeader hash *FAIL: Payload hash does not match transaction header hash*

yay! everything passes! add to block!

### notes

* timespec may be replaced with [rust-chrono](https://github.com/lifthrasiir/rust-chrono)
* if not, maybe implement hash for timespec as a pull request
* implementing traits from different crates (monkey-patching) may be allowed sometime soon
