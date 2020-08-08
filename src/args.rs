use std::path::PathBuf;
use clap::Clap;

#[derive(Clap)]
pub enum PngMeArgs {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

#[derive(Debug, Clap)]
pub struct EncodeArgs {
    file_path: PathBuf,
    chunk_type: String,
    message: String,
    output_file: Option<PathBuf>,
}

#[derive(Debug, Clap)]
pub struct DecodeArgs {
    file_path: PathBuf,
    chunk_type: String,
}

#[derive(Debug, Clap)]
pub struct RemoveArgs {
    file_path: PathBuf,
    chunk_type: String,
}

#[derive(Debug, Clap)]
pub struct PrintArgs {
    file_path: PathBuf,
}