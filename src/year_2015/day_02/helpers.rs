use anyhow::Context;

pub struct Present {
    wrapping_paper_area: u32,
    ribbon_length: u32,
}

impl Present {
    const DIMENSION_SEPARATOR: char = 'x';

    pub fn get_wrapping_paper_area(&self) -> u32 {
        self.wrapping_paper_area
    }

    pub fn get_ribbon_length(&self) -> u32 {
        self.ribbon_length
    }

    pub fn new(text: &str) -> anyhow::Result<Self> {
        let dimensions: Vec<&str> = text.split(Present::DIMENSION_SEPARATOR).collect();

        let dimensions = dimensions.iter()
            .map(|d| {
                d.parse::<u32>()
                    .with_context(|| format!("could not parse '{d}' as number"))
            })
            .collect::<anyhow::Result<Vec<_>>>()
            .with_context(|| format!("could not parse dimensions: '{text}' - use L{x}W{x}H pattern, where L = length, W = width, H = height", x = Present::DIMENSION_SEPARATOR))?;

        if dimensions.len() != 3 {
            anyhow::bail!("expected 3 dimensions in '{text}'");
        }

        let length = dimensions[0];
        let width = dimensions[1];
        let height = dimensions[2];

        let wrapping_paper_area = Self::calculate_wrapping_paper_area(length, width, height);
        let ribbon_length = Self::calculate_ribbon_length(length, width, height);

        let present = Self {
            wrapping_paper_area,
            ribbon_length,
        };

        Ok(present)
    }

    fn calculate_wrapping_paper_area(l: u32, w: u32, h: u32) -> u32 {
        let side_areas = [l * w, w * h, h * l];

        let full_area: u32 = side_areas.iter().map(|a| 2 * a).sum();
        let smallest_area = side_areas.iter().min().unwrap();

        full_area + smallest_area
    }

    fn calculate_ribbon_length(l: u32, w: u32, h: u32) -> u32 {
        let perimeters = [2 * (l + w), 2 * (w + h), 2 * (h + l)];

        let smallest_perimeter = perimeters.iter().min().unwrap();
        let volume = l * w * h;

        smallest_perimeter + volume
    }
}
