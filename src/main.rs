mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

use clap::derive::Clap;
use crate::args::PngMeArgs;


fn main() -> Result<()> {
    let png_args: PngMeArgs = PngMeArgs::parse();

    match png_args {
        PngMeArgs::Encode(x) => {
            commands::encode(x)
        },
        PngMeArgs::Decode(x) => {
            commands::decode(x)
        },
        PngMeArgs::Remove(x) => {
            commands::remove(x)
        },
        PngMeArgs::Print(x) => {
            commands::print_chunks(x)
        }
    }
}
