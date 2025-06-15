use regex::Regex;
use std::cmp::max;

pub struct Parser {
    re: Regex,
}

impl Default for Parser {
    fn default() -> Self {
        Self {
            re: Regex::new(r"^\w+: capacity (?<capacity>-?\d+), durability (?<durability>-?\d+), flavor (?<flavor>-?\d+), texture (?<texture>-?\d+), calories (?<calories>-?\d+)").unwrap(),
        }
    }
}

impl Parser {
    pub fn parse(&self, input: &str) -> anyhow::Result<Ingredient> {
        let result = match self.re.captures(input) {
            Some(caps) => {
                let capacity = caps["capacity"].parse()?;
                let durability = caps["durability"].parse()?;
                let flavor = caps["flavor"].parse()?;
                let texture = caps["texture"].parse()?;
                let calories = caps["calories"].parse()?;

                Ingredient {
                    capacity,
                    durability,
                    flavor,
                    texture,
                    calories,
                }
            }
            None => anyhow::bail!("could not parse input '{input}' as an ingredient"),
        };

        Ok(result)
    }
}

pub struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

#[derive(Default)]
pub struct RecipeCalculator {
    ingredients: Vec<Ingredient>,
}

impl RecipeCalculator {
    pub fn add_ingredient(&mut self, ingredient: Ingredient) {
        self.ingredients.push(ingredient);
    }

    pub fn solve_recipe(&mut self, spoon_amount: u32) -> anyhow::Result<Solution> {
        let mut amounts = vec![0; self.ingredients.len()];
        let mut solution = Solution::default();

        self.calculate_score(&mut solution, &mut amounts, 0, spoon_amount);

        Ok(solution)
    }

    fn calculate_score(
        &mut self,
        solution: &mut Solution,
        amounts: &mut Vec<u32>,
        index: usize,
        spoons_left: u32,
    ) {
        if index < self.ingredients.len() - 1 {
            for spoons in 0..=spoons_left {
                amounts[index] = spoons;
                self.calculate_score(solution, amounts, index + 1, spoons_left - spoons);
                amounts[index] = 0;
            }
        } else {
            amounts[index] = spoons_left;

            let mut capacity = 0;
            let mut durability = 0;
            let mut flavor = 0;
            let mut texture = 0;
            let mut calories = 0;

            for (idx, amount) in amounts.iter().enumerate() {
                let ingredient = &self.ingredients[idx];

                capacity += *amount as i32 * ingredient.capacity;
                durability += *amount as i32 * ingredient.durability;
                flavor += *amount as i32 * ingredient.flavor;
                texture += *amount as i32 * ingredient.texture;
                calories += *amount as i32 * ingredient.calories;
            }

            let capacity = max(0, capacity) as u32;
            let durability = max(0, durability) as u32;
            let flavor = max(0, flavor) as u32;
            let texture = max(0, texture) as u32;

            let score = capacity * durability * flavor * texture;

            solution.solve_best_score(score);

            if calories == 500 {
                solution.solve_best_score_500_cal(score);
            }

            amounts[index] = 0;
        }
    }
}

#[derive(Default)]
pub struct Solution {
    pub best_score: Option<u32>,
    pub best_score_500_cal: Option<u32>,
}

impl Solution {
    fn solve_best_score(&mut self, score: u32) {
        self.best_score = match self.best_score {
            Some(best) => Some(max(best, score)),
            None => Some(score),
        };
    }

    fn solve_best_score_500_cal(&mut self, score: u32) {
        self.best_score_500_cal = match self.best_score_500_cal {
            Some(best) => Some(max(best, score)),
            None => Some(score),
        };
    }
}
