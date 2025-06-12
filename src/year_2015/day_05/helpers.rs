pub struct NiceStringValidator;

impl NiceStringValidator {
    pub fn validate<T>(input: &str) -> bool
    where
        T: Criteria + Default,
    {
        let criteria = T::default();
        let input = input.as_bytes();
        criteria.check_criteria(input)
    }
}

pub trait Criteria {
    fn check_criteria(self, substr: &[u8]) -> bool;
}

#[derive(Default)]
pub struct FirstYearCriteria {
    has_letter_twice_in_the_row: bool,
    vowels_count: u16,
    has_disallowed_substring: bool,
}

impl Criteria for FirstYearCriteria {
    fn check_criteria(mut self, input: &[u8]) -> bool {
        for i in 0..input.len() {
            let substr = &input[i..];
            self.check_letter_twice_in_the_row(substr);
            self.check_vowel(substr[0] as char);
            self.check_disallowed_substring(substr);
        }

        self.has_letter_twice_in_the_row && self.vowels_count >= 3 && !self.has_disallowed_substring
    }
}

impl FirstYearCriteria {
    fn check_letter_twice_in_the_row(&mut self, substr: &[u8]) {
        if self.has_letter_twice_in_the_row || substr.len() < 2 {
            return;
        }

        self.has_letter_twice_in_the_row = substr[0] == substr[1];
    }

    fn check_vowel(&mut self, c: char) {
        if ['a', 'e', 'i', 'o', 'u'].contains(&c) {
            self.vowels_count += 1;
        }
    }

    fn check_disallowed_substring(&mut self, substr: &[u8]) {
        if self.has_disallowed_substring || substr.len() < 2 {
            return;
        }

        for disallowed in ["ab", "cd", "pq", "xy"] {
            if substr.starts_with(disallowed.as_bytes()) {
                self.has_disallowed_substring = true;
                break;
            }
        }
    }
}

#[derive(Default)]
pub struct SecondYearCriteria {
    has_twice_a_pair: bool,
    has_repeating_letter_with_a_letter_between: bool,
}

impl Criteria for SecondYearCriteria {
    fn check_criteria(mut self, input: &[u8]) -> bool {
        for i in 0..input.len() {
            let substr = &input[i..];
            self.check_twice_a_pair(substr);
            self.check_repeating_letter_with_a_letter_between(substr);
        }

        self.has_twice_a_pair && self.has_repeating_letter_with_a_letter_between
    }
}

impl SecondYearCriteria {
    fn check_twice_a_pair(&mut self, substr: &[u8]) {
        if self.has_twice_a_pair || substr.len() < 4 {
            return;
        }

        let pair = &substr[..2];
        self.has_twice_a_pair = substr[2..].windows(2).any(|window| window == pair);
    }

    fn check_repeating_letter_with_a_letter_between(&mut self, substr: &[u8]) {
        if self.has_repeating_letter_with_a_letter_between || substr.len() < 3 {
            return;
        }

        self.has_repeating_letter_with_a_letter_between = substr[0] == substr[2];
    }
}
