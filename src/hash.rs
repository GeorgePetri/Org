use std::io;
use std::io::Read;

use ring::digest::{Context, Digest, SHA256};

pub fn sha256_digest<R: Read>(mut reader: R) -> Result<Digest, io::Error> {
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    Ok(context.finish())
}


//todo add tests for this
pub fn digest_to_upper_hex(digest: Digest) -> String {
    digest
        .as_ref()
        .iter()
        .map(|e| format!("{:02X}", e))
        .fold(String::new(), |string, current| string + &current)
}
