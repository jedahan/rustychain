use knuckle::sign::{Keypair, PublicKey, SecretKey, PUBKEY_BYTES, SECKEY_BYTES};

pub static POW_DIFFICULTY: uint = 2;
/*
pub fn load_key_pair() -> Keypair {
	use std::io::File;
	let publicKey = File::open(&Path::new("public.pk"));
	let secretKey = File::open(&Path::new("secret.sk"));
	let public = publicKey.read_to_end();
	let secret = secretKey.read_to_end();

    match (public, secret) {
        (Ok(public), Ok(secret)) => {
            let pk: [u8, ..PUBKEY_BYTES] = [0, ..PUBKEY_BYTES];
            for i in range(0, public.len()) {
                pk[i] = public[i];
            }
            let sk: [u8, ..SECKEY_BYTES] = [0, ..SECKEY_BYTES];
            for i in range(0, secret.len()) {
                sk[i] = secret[i];
            }
            Keypair { pk: PublicKey(pk), sk: SecretKey(sk) }
        },
        _ => {
            Keypair::new()
        }
    }
}
*/
