use regex::Regex;
use std::collections::HashMap;

pub struct Parser {
    re_simple: Regex,
    re_gate_one_input: Regex,
    re_gate_double_input: Regex,
}

impl Default for Parser {
    fn default() -> Self {
        Self {
            re_simple: Regex::new(r"^(?<input>[a-z]{1,2}|[0-9]+) -> (?<name>[a-z]{1,2})$").unwrap(),
            re_gate_one_input: Regex::new(r"^(?<instruction>[A-Z]+) (?<input>[a-z]{1,2}|[0-9]+) -> (?<name>[a-z]{1,2})$").unwrap(),
            re_gate_double_input: Regex::new(r"^(?<left>[a-z]{1,2}|[0-9]+) (?<instruction>[A-Z]+) (?<right>[a-z]{1,2}|[0-9]+) -> (?<name>[a-z]{1,2})$").unwrap(),
        }
    }
}

impl Parser {
    pub fn parse(&self, input: &str) -> anyhow::Result<(NodeId, Node)> {
        if let Some(caps) = self.re_simple.captures(input) {
            Self::parse_simple_node(&caps)
        } else if let Some(caps) = self.re_gate_one_input.captures(input) {
            Self::parse_gate_one_input(&caps)
        } else if let Some(caps) = self.re_gate_double_input.captures(input) {
            Self::parse_gate_double_input(&caps)
        } else {
            anyhow::bail!("invalid input: {}", input);
        }
    }

    fn parse_input(input: &str) -> anyhow::Result<Input> {
        let input = match input.parse() {
            Ok(value) => Input::Value(value),
            Err(_) => Input::Node(NodeId::from(input)?),
        };

        Ok(input)
    }

    fn parse_simple_node(caps: &regex::Captures<'_>) -> anyhow::Result<(NodeId, Node)> {
        let id = NodeId::from(&caps["name"])?;
        let input = Self::parse_input(&caps["input"])?;

        Ok((id, Node::Simple(input)))
    }

    fn parse_gate_one_input(caps: &regex::Captures<'_>) -> anyhow::Result<(NodeId, Node)> {
        let id = NodeId::from(&caps["name"])?;
        let instruction = &caps["instruction"];
        let input = Self::parse_input(&caps["input"])?;

        let result = match instruction {
            "NOT" => (id, Node::Negation(input)),
            other => anyhow::bail!("invalid instruction: {other}"),
        };

        Ok(result)
    }

    fn parse_gate_double_input(caps: &regex::Captures<'_>) -> anyhow::Result<(NodeId, Node)> {
        let id = NodeId::from(&caps["name"])?;
        let instruction = &caps["instruction"];
        let left = Self::parse_input(&caps["left"])?;
        let right = Self::parse_input(&caps["right"])?;

        let result = match instruction {
            "AND" => (id, Node::AndGate(left, right)),
            "OR" => (id, Node::OrGate(left, right)),
            "RSHIFT" => (id, Node::RightShift(left, right)),
            "LSHIFT" => (id, Node::LeftShift(left, right)),
            other => anyhow::bail!("invalid instruction: {other}"),
        };

        Ok(result)
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct NodeId(u64);

impl NodeId {
    pub fn from(name: &str) -> anyhow::Result<Self> {
        let id = u64::from_str_radix(name, 36)?;
        Ok(NodeId(id))
    }
}

#[derive(Clone, Copy)]
pub enum Node {
    Simple(Input),
    AndGate(Input, Input),
    OrGate(Input, Input),
    Negation(Input),
    RightShift(Input, Input),
    LeftShift(Input, Input),
}

#[derive(Clone, Copy)]
pub enum Input {
    Value(u16),
    Node(NodeId),
}

#[derive(Default)]
pub struct Circuit {
    values: HashMap<NodeId, u16>,
    nodes: HashMap<NodeId, Node>,
}

impl Circuit {
    pub fn clear(&mut self) {
        self.values.clear();
    }

    pub fn set(&mut self, id: NodeId, node: Node) {
        self.nodes.insert(id, node);
    }

    pub fn get_node_value(&mut self, id: NodeId) -> anyhow::Result<u16> {
        if let Some(value) = self.values.get(&id) {
            return Ok(*value);
        }

        let node = *self
            .nodes
            .get(&id)
            .ok_or_else(|| anyhow::anyhow!("invalid node id"))?;

        let value = match node {
            Node::Simple(input) => self.get_input_value(input)?,

            Node::AndGate(left, right) => {
                self.get_input_value(left)? & self.get_input_value(right)?
            }

            Node::OrGate(left, right) => {
                self.get_input_value(left)? | self.get_input_value(right)?
            }

            Node::Negation(signal) => !self.get_input_value(signal)?,

            Node::RightShift(left, right) => {
                self.get_input_value(left)? >> self.get_input_value(right)?
            }

            Node::LeftShift(left, right) => {
                self.get_input_value(left)? << self.get_input_value(right)?
            }
        };

        self.values.insert(id, value);

        Ok(value)
    }

    fn get_input_value(&mut self, input: Input) -> anyhow::Result<u16> {
        match input {
            Input::Value(value) => Ok(value),
            Input::Node(id) => self.get_node_value(id),
        }
    }
}
