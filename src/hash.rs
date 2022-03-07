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

pub fn digest_to_upper_hex(digest: Digest) -> String {
    bytes_to_upper_hex(digest.as_ref())
}

fn bytes_to_upper_hex(data: &[u8]) -> String {
    data.iter()
        .map(|e| format!("{e:02X}"))
        .fold(String::new(), |string, current| string + &current)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bytes_to_upper_hex_return_correct_values() {
        let data = vec![0x0f, 0x00, 0x05, 0xff];

        let result = bytes_to_upper_hex(&data);

        //adds leading 0
        assert_eq!("0F0005FF", result);
    }
}
