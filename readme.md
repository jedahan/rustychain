blockchain client in rust

inspired by [minimum viable blockchain](https://www.igvita.com/2014/05/05/minimum-viable-block-chain/)

### TODO

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

### notes

timespec will probably be replaces soon, see https://github.com/lifthrasiir/rust-chrono maybe
if not, hash as a pull request might be a nice contribution
implementing traits from different crates (monkey-patching) may be allowed sometime soon
