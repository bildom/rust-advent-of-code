use anyhow::Context;
use md5::digest::Output;
use md5::{Digest, Md5};
use std::io::Write;

#[derive(Default)]
pub struct Hasher {
    hasher: Md5,
}

impl Hasher {
    pub fn calculate_suffix<F>(
        &mut self,
        text: &str,
        starts_with: u32,
        hash_checker: F,
    ) -> anyhow::Result<u32>
    where
        F: Fn(&[u8]) -> bool,
    {
        let mut result = None;
        let mut buffer = Vec::new();
        let mut hash = Output::<Md5>::default();

        for number in starts_with..=u32::MAX {
            buffer.clear();

            write!(&mut buffer, "{text}{number}").with_context(|| "could not write hash")?;

            self.hasher.update(&buffer);
            self.hasher.finalize_into_reset(&mut hash);

            if hash_checker(&hash) {
                result = Some(number);
                break;
            }
        }

        if let Some(number) = result {
            Ok(number)
        } else {
            anyhow::bail!("could not calculate the suffix for {text}");
        }
    }

    pub fn starts_with_5_zeros(hash: &[u8]) -> bool {
        hash[0] == 0x00 && hash[1] == 0x00 && hash[2] <= 0x0F
    }

    pub fn starts_with_6_zeros(hash: &[u8]) -> bool {
        hash[0] == 0x00 && hash[1] == 0x00 && hash[2] == 0x00
    }
}
