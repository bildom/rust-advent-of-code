use regex::Regex;
use std::cmp::max;

pub struct Parser {
    re: Regex,
}

impl Default for Parser {
    fn default() -> Self {
        Self {
            re: Regex::new(r"^\S+: capacity (?<capacity>-?\d+), durability (?<durability>-?\d+), flavor (?<flavor>-?\d+), texture (?<texture>-?\d+), calories (?<calories>-?\d+)").unwrap(),
        }
    }
}

impl Parser {
    pub fn parse(&self, input: &str) -> anyhow::Result<Ingredient> {
        if let Some(caps) = self.re.captures(input) {
            let capacity = caps["capacity"].parse()?;
            let durability = caps["durability"].parse()?;
            let flavor = caps["flavor"].parse()?;
            let texture = caps["texture"].parse()?;
            let calories = caps["calories"].parse()?;

            Ok(Ingredient {
                capacity,
                durability,
                flavor,
                texture,
                calories,
            })
        } else {
            anyhow::bail!("could not parse input '{input}' as an ingredient");
        }
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
    best_score: Option<u32>,
    best_score_500_cal: Option<u32>,
}

impl RecipeCalculator {
    pub fn get_best_score(&self) -> Option<u32> {
        self.best_score
    }

    pub fn get_best_score_500_cal(&self) -> Option<u32> {
        self.best_score_500_cal
    }

    pub fn add_ingredient(&mut self, ingredient: Ingredient) {
        self.ingredients.push(ingredient);
    }

    pub fn solve_recipe(&mut self, spoon_amount: u32) {
        let mut amounts = vec![0; self.ingredients.len()];

        self.calculate_score(&mut amounts, 0, spoon_amount);
    }

    fn calculate_score(&mut self, amounts: &mut Vec<u32>, index: usize, spoons_left: u32) {
        if index < self.ingredients.len() - 1 {
            for spoons in 0..=spoons_left {
                amounts[index] = spoons;
                self.calculate_score(amounts, index + 1, spoons_left - spoons);
                amounts[index] = 0;
            }
        } else {
            amounts[index] = spoons_left;

            let mut capacity_score = 0;
            let mut durability_score = 0;
            let mut flavor_score = 0;
            let mut texture_score = 0;
            let mut calories = 0;

            for (idx, amount) in amounts.iter().enumerate() {
                let ingredient = &self.ingredients[idx];
                capacity_score += *amount as i32 * ingredient.capacity;
                durability_score += *amount as i32 * ingredient.durability;
                flavor_score += *amount as i32 * ingredient.flavor;
                texture_score += *amount as i32 * ingredient.texture;
                calories += *amount as i32 * ingredient.calories;
            }

            let score = (max(0, capacity_score)
                * max(0, durability_score)
                * max(0, flavor_score)
                * max(0, texture_score)) as u32;

            self.best_score = match self.best_score {
                Some(best) => Some(max(best, score)),
                None => Some(score),
            };

            if calories == 500 {
                self.best_score_500_cal = match self.best_score_500_cal {
                    Some(best) => Some(max(best, score)),
                    None => Some(score),
                };
            }

            amounts[index] = 0;
        }
    }
}
