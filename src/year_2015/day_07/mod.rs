use crate::puzzle::{answer, puzzle_solver};
use crate::year_2015::day_07::helpers::{Circuit, Input, Node, NodeId, Parser};

mod helpers;

puzzle_solver!(
    [2015, 7] = {
        fn solve(&self, input: &str) -> anyhow::Result<Answer> {
            let parser = Parser::default();
            let mut circuit = Circuit::default();

            for line in input.lines() {
                let (id, node) = parser.parse(line)?;
                circuit.set(id, node);
            }

            let a_id = NodeId::from("a")?;

            let first_value_of_wire_a = circuit.get_node_value(a_id)?;

            let b_id = NodeId::from("b")?;
            circuit.clear();
            circuit.set(b_id, Node::Simple(Input::Value(first_value_of_wire_a)));

            let second_value_of_wire_a = circuit.get_node_value(a_id)?;

            answer!(first_value_of_wire_a, second_value_of_wire_a);
        }
    }
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::puzzle::Solver;
    use indoc::indoc;
    use rstest::rstest;

    const INPUT_AND: &str = indoc! {
        "b AND c -> a
        123 -> b
        456 -> c"
    };

    const INPUT_OR: &str = indoc! {
        "b OR c -> a
        123 -> b
        456 -> c"
    };

    const INPUT_RSHIFT: &str = indoc! {
        "b RSHIFT 2 -> a
        123 -> b"
    };

    const INPUT_LSHIFT: &str = indoc! {
        "b LSHIFT 2 -> a
        123 -> b"
    };

    const INPUT_NOT: &str = indoc! {
        "NOT b -> a
        123 -> b"
    };

    #[rstest]
    #[case(INPUT_AND, 72, 72)]
    #[case(INPUT_OR, 507, 507)]
    #[case(INPUT_RSHIFT, 30, 7)]
    #[case(INPUT_LSHIFT, 492, 1968)]
    #[case(INPUT_NOT, 65412, 123)]
    fn positive_tests(
        #[case] input: &str,
        #[case] expected_first_value_of_wire_a: u16,
        #[case] expected_second_value_of_wire_a: u16,
    ) {
        let solution = Puzzle.solve(input).unwrap();

        assert_eq!(
            solution.results[0],
            expected_first_value_of_wire_a.to_string()
        );
        assert_eq!(
            solution.results[1],
            expected_second_value_of_wire_a.to_string()
        );
    }
}
