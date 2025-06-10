pub trait Criteria {
    fn check_criteria(&mut self, substr: &[u8]);
    fn is_nice(&self) -> bool;
}

#[derive(Default)]
pub struct FirstYearCriteria {
    has_letter_twice_in_the_row: bool,
    vowels_count: u16,
    has_disallowed_substring: bool,
}

impl Criteria for FirstYearCriteria {
    fn check_criteria(&mut self, substr: &[u8]) {
        self.check_letter_twice_in_the_row(substr);
        self.check_vowel(substr[0] as char);
        self.check_disallowed_substring(substr);
    }

    fn is_nice(&self) -> bool {
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
        const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

        if VOWELS.contains(&c) {
            self.vowels_count += 1;
        }
    }

    fn check_disallowed_substring(&mut self, substr: &[u8]) {
        const DISALLOWED_SUBSTRINGS: [&str; 4] = ["ab", "cd", "pq", "xy"];

        if self.has_disallowed_substring || substr.len() < 2 {
            return;
        }

        for disallowed in DISALLOWED_SUBSTRINGS {
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
    fn check_criteria(&mut self, substr: &[u8]) {
        self.check_twice_a_pair(substr);
        self.check_repeating_letter_with_a_letter_between(substr);
    }

    fn is_nice(&self) -> bool {
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
