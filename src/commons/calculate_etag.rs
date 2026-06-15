use anyhow::Result;
use md5::{Digest, Md5};
use std::fs::File;
use std::io::prelude::*;

fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

#[allow(dead_code)]
fn calculate_etag_from_read(f: &mut dyn Read, chunk_size: usize) -> Result<String> {
    let mut concat_hasher = Md5::new();
    let mut input_buffer = vec![0u8; chunk_size];
    let mut chunk_count = 0;
    let mut first_hash = [0u8; 16];

    loop {
        let amount_read = f.read(&mut input_buffer)?;
        if amount_read == 0 {
            break;
        }

        let hash = Md5::digest(&input_buffer[0..amount_read]);
        concat_hasher.update(&hash);

        if chunk_count == 0 {
            first_hash.copy_from_slice(&hash);
        }
        chunk_count += 1;
    }

    Ok(if chunk_count > 1 {
        format!("{}-{}", hex_encode(&concat_hasher.finalize()), chunk_count)
    } else if chunk_count == 1 {
        hex_encode(&first_hash)
    } else {
        String::new()
    })
}

#[allow(dead_code)]
fn calculate_etag(file: &str, chunk_size: usize) -> Result<String> {
    let mut f = File::open(file)?;
    calculate_etag_from_read(&mut f, chunk_size)
}

#[cfg(test)]
mod test {

    use super::calculate_etag;

    //cargo test commons::calculate_etag::test::test_calculate_etag -- --nocapture
    #[test]
    fn test_calculate_etag() {
        let r = calculate_etag("/tmp/gen/tmp", 8);
        println!("test scan result {:?}", r);
    }
}
