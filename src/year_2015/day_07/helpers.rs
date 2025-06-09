use regex::Regex;
use std::collections::HashMap;

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct NodeId(u16);

impl NodeId {
    pub fn from(name: &str) -> anyhow::Result<Self> {
        let bytes = name.as_bytes();

        let id = if bytes.len() == 1 {
            NodeId(bytes[0] as u16)
        } else if bytes.len() == 2 {
            NodeId(u16::from_le_bytes([bytes[1], bytes[0]]))
        } else {
            anyhow::bail!("invalid node name: {name}");
        };

        Ok(id)
    }
}

pub struct NodeParser {
    re_simple: Regex,
    re_gate_one_input: Regex,
    re_gate_double_input: Regex,
}

impl Default for NodeParser {
    fn default() -> Self {
        Self {
            re_simple: Regex::new(r"^(?<input>[a-z]{1,2}|[0-9]+) -> (?<name>[a-z]{1,2})$").unwrap(),
            re_gate_one_input: Regex::new(r"^(?<instruction>[A-Z]+) (?<input>[a-z]{1,2}|[0-9]+) -> (?<name>[a-z]{1,2})$").unwrap(),
            re_gate_double_input: Regex::new(r"^(?<left>[a-z]{1,2}|[0-9]+) (?<instruction>[A-Z]+) (?<right>[a-z]{1,2}|[0-9]+) -> (?<name>[a-z]{1,2})$").unwrap(),
        }
    }
}

impl NodeParser {
    const AND: &'static str = "AND";
    const OR: &'static str = "OR";
    const NOT: &'static str = "NOT";
    const RIGHT_SHIFT: &'static str = "RSHIFT";
    const LEFT_SHIFT: &'static str = "LSHIFT";

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
        let input = if let Ok(value) = input.parse::<u16>() {
            Input::Value(value)
        } else {
            Input::Node(NodeId::from(input)?)
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

        match instruction {
            NodeParser::NOT => Ok((id, Node::Negation(input))),
            other => anyhow::bail!("invalid instruction: {other}"),
        }
    }

    fn parse_gate_double_input(caps: &regex::Captures<'_>) -> anyhow::Result<(NodeId, Node)> {
        let id = NodeId::from(&caps["name"])?;
        let instruction = &caps["instruction"];
        let left = Self::parse_input(&caps["left"])?;
        let right = Self::parse_input(&caps["right"])?;

        match instruction {
            NodeParser::AND => Ok((id, Node::AndGate(left, right))),
            NodeParser::OR => Ok((id, Node::OrGate(left, right))),
            NodeParser::RIGHT_SHIFT => Ok((id, Node::RightShift(left, right))),
            NodeParser::LEFT_SHIFT => Ok((id, Node::LeftShift(left, right))),
            other => anyhow::bail!("invalid instruction: {other}"),
        }
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

    pub fn get_node_value(&mut self, id: NodeId) -> Option<u16> {
        if let Some(value) = self.values.get(&id) {
            return Some(*value);
        }

        let node = *self.nodes.get(&id)?;

        let value = match node {
            Node::Simple(input) => self.get_input_value(input),

            Node::AndGate(left, right) => {
                Some(self.get_input_value(left)? & self.get_input_value(right)?)
            }

            Node::OrGate(left, right) => {
                Some(self.get_input_value(left)? | self.get_input_value(right)?)
            }

            Node::Negation(signal) => Some(!self.get_input_value(signal)?),

            Node::RightShift(left, right) => {
                Some(self.get_input_value(left)? >> self.get_input_value(right)?)
            }

            Node::LeftShift(left, right) => {
                Some(self.get_input_value(left)? << self.get_input_value(right)?)
            }
        };

        if let Some(value) = value {
            self.values.insert(id, value);
        }

        value
    }

    fn get_input_value(&mut self, input: Input) -> Option<u16> {
        match input {
            Input::Value(value) => Some(value),
            Input::Node(id) => self.get_node_value(id),
        }
    }
}
