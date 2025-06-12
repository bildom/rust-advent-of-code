use crate::puzzle::{answer, puzzle_solver};
use crate::year_2015::day_08::helpers::StringParser;

mod helpers;

puzzle_solver!(
    [2015, 8] = {
        fn solve(&mut self, input: &str) -> anyhow::Result<Answer> {
            let mut raw_length = 0;
            let mut parsed_length = 0;
            let mut encoded_length = 0;

            for line in input.lines() {
                raw_length += line.len();
                parsed_length += StringParser::get_unescaped_string_as_u8(line)?.len();
                encoded_length += format!("\"{}\"", line.escape_default()).len();
            }

            let raw_vs_parsed_diff = raw_length - parsed_length;
            let encoded_vs_raw_diff = encoded_length - raw_length;

            answer!(raw_vs_parsed_diff, encoded_vs_raw_diff);
        }
    }
);

#[cfg(test)]
mod tests {
    use crate::puzzle::Solver;
    use crate::year_2015::day_08::Puzzle;
    use rstest::rstest;

    #[rstest]
    #[case(stringify!(""), 2, 4)]
    #[case(stringify!("abc"), 2, 4)]
    #[case(stringify!("aaa\"aaa"), 3, 6)]
    #[case(stringify!("\x27"), 5, 5)]
    fn positive_tests(
        #[case] input: &str,
        #[case] expected_first_year: u32,
        #[case] expected_second_year: u32,
    ) {
        let answer = Puzzle.solve(input).unwrap();

        assert_eq!(answer.results[0], expected_first_year.to_string());
        assert_eq!(answer.results[1], expected_second_year.to_string());
    }
}
