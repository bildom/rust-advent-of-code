type ValidPassword = bool;

pub struct PasswordGenerator;

impl PasswordGenerator {
    pub fn find_next_valid_password(password: &str) -> anyhow::Result<String> {
        let mut password: Vec<char> = password.chars().collect();

        match Self::increment_password(&mut password, 0, false) {
            Ok(true) => Ok(password.into_iter().collect()),
            Ok(false) => anyhow::bail!("no correct password found"),
            Err(e) => Err(e),
        }
    }

    fn increment_password(
        password: &mut [char],
        idx: usize,
        skip: bool,
    ) -> anyhow::Result<ValidPassword> {
        loop {
            if idx < password.len() - 1 {
                let skip = skip || !Self::check_allowed_char(password[idx]);

                match Self::increment_password(password, idx + 1, skip) {
                    Ok(false) => (),
                    other => return other,
                }
            }

            let next_c = if skip {
                'a'
            } else {
                Self::get_next_char(password[idx])?
            };

            password[idx] = next_c;

            if next_c == 'a' {
                break Ok(false);
            }

            if Self::validate(password) {
                return Ok(true);
            }
        }
    }

    fn check_allowed_char(c: char) -> bool {
        const FORBIDDEN_CHARS: [char; 3] = ['i', 'o', 'l'];

        !FORBIDDEN_CHARS.contains(&c)
    }

    fn get_next_char(c: char) -> anyhow::Result<char> {
        let mut c = c;

        loop {
            c = match c {
                'a'..='y' => char::from(c as u8 + 1),
                'z' => 'a',
                _ => anyhow::bail!("invalid character: {}", c),
            };
            if Self::check_allowed_char(c) {
                break Ok(c);
            }
        }
    }

    fn validate(password: &[char]) -> bool {
        Self::check_three_increasing_letters(password)
            && Self::check_non_overlapping_pairs(password) > 1
    }

    fn check_three_increasing_letters(password: &[char]) -> bool {
        password
            .windows(3)
            .any(|w| w[2] as u32 == (w[1] as u32 + 1) && w[1] as u32 == (w[0] as u32 + 1))
    }

    fn check_non_overlapping_pairs(password: &[char]) -> u8 {
        let mut pair_last_window = false;
        let mut count = 0u8;

        for pair in password.windows(2) {
            if pair[0] == pair[1] && !pair_last_window {
                pair_last_window = true;
                count += 1;
            } else {
                pair_last_window = false;
            }
        }

        count
    }
}
