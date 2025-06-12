use regex::Regex;

pub struct Parser {
    re: Regex,
}

impl Default for Parser {
    fn default() -> Self {
        Self {
            re: Regex::new(r"^(?<length>\d+)x(?<width>\d+)x(?<height>\d+)$").unwrap(),
        }
    }
}

impl Parser {
    pub fn parse(&self, text: &str) -> anyhow::Result<Present> {
        if let Some(caps) = self.re.captures(text) {
            let length = caps["length"].parse::<u32>()?;
            let width = caps["width"].parse::<u32>()?;
            let height = caps["height"].parse::<u32>()?;

            Ok(Present {
                length,
                width,
                height,
            })
        } else {
            anyhow::bail!("could not parse dimensions: '{text}'")
        }
    }
}

pub struct Present {
    length: u32,
    width: u32,
    height: u32,
}

impl Present {
    pub fn get_wrapping_paper_area(&self) -> u32 {
        let side_areas = [
            self.length * self.width,
            self.width * self.height,
            self.height * self.length,
        ];

        let full_area: u32 = side_areas.iter().map(|a| 2 * a).sum();
        let smallest_area = side_areas.iter().min().unwrap();

        full_area + smallest_area
    }

    pub fn get_ribbon_length(&self) -> u32 {
        let perimeters = [
            2 * (self.length + self.width),
            2 * (self.width + self.height),
            2 * (self.height + self.length),
        ];

        let smallest_perimeter = perimeters.iter().min().unwrap();
        let volume = self.length * self.width * self.height;

        smallest_perimeter + volume
    }
}
