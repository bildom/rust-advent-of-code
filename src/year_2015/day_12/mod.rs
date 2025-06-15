use crate::puzzle::{answer, puzzle_solver};
use crate::year_2015::day_12::helpers::JsonInterpreter;

mod helpers;

puzzle_solver!(
    [2015, 12] = {
        fn solve(&self, input: &str) -> anyhow::Result<crate::puzzle::Answer> {
            let json = serde_json::from_str(input)?;

            let sum = JsonInterpreter::default().add_numbers(&json);
            let sum_without_red = JsonInterpreter::without("red").add_numbers(&json);

            answer!(sum, sum_without_red);
        }
    }
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::puzzle::Solver;
    use rstest::rstest;

    #[rstest]
    #[case("[1,2,3]", 6, 6)]
    #[case(r#"{"a":2,"b":4}"#, 6, 6)]
    #[case(r#"{"a":2,"b":4}"#, 6, 6)]
    #[case("[[[3]]]", 3, 3)]
    #[case(r#"{"a":{"b":4},"c":-1}"#, 3, 3)]
    #[case(r#"{"a":[-1,1]}"#, 0, 0)]
    #[case(r#"[-1,{"a":1}]"#, 0, 0)]
    #[case("[]", 0, 0)]
    #[case("{}", 0, 0)]
    #[case(r#"[1,{"c":"red","b":2},3]"#, 6, 4)]
    #[case(r#"{"d":"red","e":[1,2,3,4],"f":5}"#, 15, 0)]
    #[case(r#"[1,"red",5]"#, 6, 6)]
    fn positive_tests(
        #[case] input: &str,
        #[case] expected_sum: i64,
        #[case] expected_sum_without_red: i64,
    ) {
        let answer = Puzzle.solve(input);

        assert!(answer.is_ok());

        let answer = answer.unwrap();

        assert_eq!(answer.results[0], expected_sum.to_string());
        assert_eq!(answer.results[1], expected_sum_without_red.to_string());
    }
}
