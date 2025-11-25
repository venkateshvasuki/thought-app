use clap::{Parser, ValueEnum};
use strum_macros::{Display, EnumString};

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(long, value_enum)]
    pub thought_type: ThoughtType,
    #[arg(short = 'c', long)]
    pub content: String,
}

#[derive(Display, Debug, Clone, ValueEnum, EnumString)]
pub enum ThoughtType {
    Notes,
    Project,
    Misc,
    Todo,
    Question,
}
