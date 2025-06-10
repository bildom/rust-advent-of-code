use crate::puzzle::{answer, puzzle_solver};
use crate::year_2015::day_07::helpers::*;

mod helpers;

puzzle_solver!(
    [2015, 7] = {
        fn solve(&mut self, input: &str) -> anyhow::Result<Answer> {
            let parser = NodeParser::default();
            let mut circuit = Circuit::default();

            for line in input.lines() {
                let (id, node) = parser.parse(line)?;
                circuit.set(id, node);
            }

            let a_id = NodeId::from("a")?;

            let first_value_of_wire_a = match circuit.get_node_value(a_id) {
                Some(value) => value,
                None => anyhow::bail!("could not calculate wire 'a' value on the first pass"),
            };

            let b_id = NodeId::from("b")?;
            circuit.clear();
            circuit.set(b_id, Node::Simple(Input::Value(first_value_of_wire_a)));

            let second_value_of_wire_a = match circuit.get_node_value(a_id) {
                Some(value) => value,
                None => anyhow::bail!("could not calculate wire 'a' value on the second pass"),
            };

            answer!(first_value_of_wire_a, second_value_of_wire_a);
        }
    }
);

#[cfg(test)]
mod tests {}
