use anyhow::Context;
use std::fs;
use std::path::PathBuf;

#[derive(clap::Parser)]
pub struct Args {
    #[clap(short)]
    pub year: u16,
    #[clap(short)]
    pub day: u16,
    #[clap(flatten)]
    pub input: InputArgs,
}

#[derive(clap::Args)]
#[group(required = true, multiple = false)]
pub struct InputArgs {
    #[clap(short)]
    pub input: Option<String>,
    #[clap(short)]
    pub file_path: Option<String>,
}

impl InputArgs {
    pub fn extract(self) -> anyhow::Result<String> {
        let input = match (&self.file_path, self.input) {
            (None, Some(input)) => input,

            (Some(file_path), None) => {
                let path = PathBuf::from(file_path);

                fs::read_to_string(&path)
                    .with_context(|| format!("unable to read {file_path} input file"))?
            }

            _ => anyhow::bail!("invalid input parameters"),
        };

        Ok(input)
    }
}
