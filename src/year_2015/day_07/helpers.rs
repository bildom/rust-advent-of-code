use crate::dictionary::{Dictionary, DictionaryIdx};
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
    pub fn parse(&self, input: &str) -> anyhow::Result<(String, Node)> {
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
            Err(_) => Input::Node(String::from(input)),
        };

        Ok(input)
    }

    fn parse_simple_node(caps: &regex::Captures) -> anyhow::Result<(String, Node)> {
        let name = caps["name"].to_string();
        let input = Self::parse_input(&caps["input"])?;

        Ok((name, Node::Simple(input)))
    }

    fn parse_gate_one_input(caps: &regex::Captures) -> anyhow::Result<(String, Node)> {
        let name = caps["name"].to_string();
        let instruction = &caps["instruction"];
        let input = Self::parse_input(&caps["input"])?;

        let result = match instruction {
            "NOT" => (name, Node::Negation(input)),
            other => anyhow::bail!("invalid instruction: {other}"),
        };

        Ok(result)
    }

    fn parse_gate_double_input(caps: &regex::Captures) -> anyhow::Result<(String, Node)> {
        let name = caps["name"].to_string();
        let instruction = &caps["instruction"];
        let left = Self::parse_input(&caps["left"])?;
        let right = Self::parse_input(&caps["right"])?;

        let result = match instruction {
            "AND" => (name, Node::AndGate(left, right)),
            "OR" => (name, Node::OrGate(left, right)),
            "RSHIFT" => (name, Node::RightShift(left, right)),
            "LSHIFT" => (name, Node::LeftShift(left, right)),
            other => anyhow::bail!("invalid instruction: {other}"),
        };

        Ok(result)
    }
}

pub enum Node {
    Simple(Input),
    AndGate(Input, Input),
    OrGate(Input, Input),
    Negation(Input),
    RightShift(Input, Input),
    LeftShift(Input, Input),
}

pub enum Input {
    Value(u16),
    Node(String),
}

type NodeIdx = DictionaryIdx;
type NodeCache = HashMap<NodeIdx, u16>;

#[derive(Default)]
pub struct Circuit {
    node_dict: Dictionary,
    nodes: HashMap<NodeIdx, Node>,
}

impl Circuit {
    pub fn set(&mut self, name: &str, node: Node) {
        let idx = self.node_dict.put(name);
        self.nodes.insert(idx, node);
    }

    fn get_node_index(&self, name: &str) -> anyhow::Result<NodeIdx> {
        self.node_dict
            .map_to_idx(name)
            .ok_or_else(|| anyhow::anyhow!("invalid node name: {name}"))
    }

    pub fn get_node_value(&self, name: &str) -> anyhow::Result<u16> {
        let mut cache = NodeCache::new();
        self.solve_node_value(&mut cache, name)
    }

    pub fn solve_node_value(&self, cache: &mut NodeCache, name: &str) -> anyhow::Result<u16> {
        let idx = self.get_node_index(name)?;

        if let Some(value) = cache.get(&idx) {
            return Ok(*value);
        }

        let node = self
            .nodes
            .get(&idx)
            .ok_or_else(|| anyhow::anyhow!("invalid circuit definition (no '{name}')"))?;

        let value = match node {
            Node::Simple(input) => self.solve_input_value(cache, input)?,

            Node::AndGate(left, right) => {
                self.solve_input_value(cache, left)? & self.solve_input_value(cache, right)?
            }

            Node::OrGate(left, right) => {
                self.solve_input_value(cache, left)? | self.solve_input_value(cache, right)?
            }

            Node::Negation(signal) => !self.solve_input_value(cache, signal)?,

            Node::RightShift(left, right) => {
                self.solve_input_value(cache, left)? >> self.solve_input_value(cache, right)?
            }

            Node::LeftShift(left, right) => {
                self.solve_input_value(cache, left)? << self.solve_input_value(cache, right)?
            }
        };

        cache.insert(idx, value);

        Ok(value)
    }

    fn solve_input_value(&self, cache: &mut NodeCache, input: &Input) -> anyhow::Result<u16> {
        match input {
            Input::Value(value) => Ok(*value),
            Input::Node(name) => self.solve_node_value(cache, name),
        }
    }
}
