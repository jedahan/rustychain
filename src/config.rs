extern crate knuckle;

use knuckle::sign::Keypair;

pub fn load_key_pair() -> (uint, uint) {
	use std::io::File;
	let publicKey = File::open(&Path::new("public.pk"));
	let secretKey = File::open(&Path::new("secret.sk"));
	let public = try!(publicKey.read_to_end());
	let secret = try!(secretKey.read_to_end());

	if public.len() < 1 || secret.len() < 1 {
		println!("keypair not found, generating a new one");
		let k {public: pk, secret: sk} = Keypair::new();
		println!("public {}\nsecret {}\n", public, secret);
	}

	(public, secret)
}
