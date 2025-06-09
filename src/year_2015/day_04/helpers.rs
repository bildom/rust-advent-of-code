use md5::digest::core_api::CoreWrapper;
use md5::{Digest, Md5Core};

#[derive(Default)]
pub struct Hasher {
    hasher: CoreWrapper<Md5Core>,
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
        let mut number = starts_with;

        loop {
            self.hasher.update(format!("{text}{number}"));
            let hash = &self.hasher.finalize_reset()[..];

            if hash_checker(hash) {
                break Ok(number);
            }

            number = match number.checked_add_signed(1) {
                Some(number) => number,
                None => anyhow::bail!("number overflow!"),
            };
        }
    }

    pub fn starts_with_5_zeros(hash: &[u8]) -> bool {
        hash[0] == 0x00 && hash[1] == 0x00 && hash[2] <= 0x0F
    }

    pub fn starts_with_6_zeros(hash: &[u8]) -> bool {
        hash[0] == 0x00 && hash[1] == 0x00 && hash[2] == 0x00
    }
}
