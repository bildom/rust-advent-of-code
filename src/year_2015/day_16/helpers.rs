use regex::Regex;
use std::collections::HashMap;

pub struct Parser {
    re_index: Regex,
    re_param: Regex,
}

impl Default for Parser {
    fn default() -> Self {
        Self {
            re_index: Regex::new(r"^Sue (?<index>\d+)").unwrap(),
            re_param: Regex::new(r"(?<param>\w+): (?<amount>\d+)(?:, )?").unwrap(),
        }
    }
}

impl Parser {
    pub fn parse(&self, input: &str) -> anyhow::Result<Aunt> {
        let Some(caps) = self.re_index.captures(input) else {
            anyhow::bail!("could not parse index");
        };

        let index = caps["index"].parse()?;

        let mut params = HashMap::new();

        for caps in self.re_param.captures_iter(input) {
            let param = AuntParam::from(&caps["param"])?;
            let amount = caps["amount"].parse()?;

            params.insert(param, amount);
        }

        Ok(Aunt { index, params })
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum AuntParam {
    Children,
    Cats,
    Samoyeds,
    Pomeranians,
    Akitas,
    Vizslas,
    Goldfish,
    Trees,
    Cars,
    Perfumes,
}

impl AuntParam {
    pub fn from(name: &str) -> anyhow::Result<Self> {
        let result = match name {
            "children" => Self::Children,
            "cats" => Self::Cats,
            "samoyeds" => Self::Samoyeds,
            "pomeranians" => Self::Pomeranians,
            "akitas" => Self::Akitas,
            "vizslas" => Self::Vizslas,
            "goldfish" => Self::Goldfish,
            "trees" => Self::Trees,
            "cars" => Self::Cars,
            "perfumes" => Self::Perfumes,
            _ => anyhow::bail!("unknown parameter: {name}"),
        };

        Ok(result)
    }
}

type AuntParams = HashMap<AuntParam, u8>;

pub struct Aunt {
    index: usize,
    params: AuntParams,
}

impl Aunt {
    fn equals_exact(&self, other: &AuntParams) -> bool {
        let mut result = true;

        for (param, amount) in &self.params {
            if let Some(other_amount) = other.get(param) {
                result &= amount == other_amount;
            }
        }

        result
    }

    fn equals_approx(&self, other: &AuntParams) -> bool {
        let mut result = true;

        for (param, amount) in &self.params {
            if let Some(other_amount) = other.get(param) {
                result &= match param {
                    AuntParam::Cats | AuntParam::Trees => amount > other_amount,
                    AuntParam::Pomeranians | AuntParam::Goldfish => amount < other_amount,
                    _ => amount == other_amount,
                };
            }
        }

        result
    }
}

#[derive(Default)]
pub struct Analyser {
    aunts: Vec<Aunt>,
}

impl Analyser {
    pub fn add_aunt(&mut self, info: Aunt) {
        self.aunts.push(info);
    }

    pub fn solve_aunt_indices(&self, aunt_to_find: &AuntParams) -> anyhow::Result<Solution> {
        let mut index_exact = None;
        let mut index_approx = None;

        for aunt in &self.aunts {
            if aunt.equals_exact(aunt_to_find) {
                index_exact = Some(aunt.index);
            }
            if aunt.equals_approx(aunt_to_find) {
                index_approx = Some(aunt.index);
            }
        }

        let solution = Solution {
            index_exact,
            index_approx,
        };

        Ok(solution)
    }
}

pub struct Solution {
    pub index_exact: Option<usize>,
    pub index_approx: Option<usize>,
}
