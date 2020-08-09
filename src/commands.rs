use std::str::FromStr;
use crate::{Result};
use crate::args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs};
use crate::png::{Chunk, ChunkType, Png};

/// Encodes a message into a PNG file and saves the result
pub fn encode(args: EncodeArgs) -> Result<()> {
    
    let mut png = Png::from_file(args.file_path.as_path()).expect("error reading png file");

    let msg = Chunk::new(ChunkType::from_str(&args.chunk_type)?, args.message.bytes().collect());

    png.append_chunk(msg);

    let output_path = if args.output_file == None {
        args.file_path
    }else {
        args.output_file.unwrap()
    };
    
    png.save_file(output_path.as_path()).expect("could not write");

    Ok(())
}

/// Searches for a message hidden in a PNG file and prints the message if one is found
pub fn decode(args: DecodeArgs) -> Result<()> {
    let png = Png::from_file(args.file_path.as_path()).expect("error reading png file");

    match png.chunk_by_type(&args.chunk_type){
        None => println!("Message not found"),
        Some(c) => println!("Message found: {}", c.data_as_string()?),
    }

    Ok(())
}

/// Removes a chunk from a PNG file and saves the result
pub fn remove(args: RemoveArgs) -> Result<()> {
    let mut png = Png::from_file(args.file_path.as_path()).expect("error reading png file");

    png.remove_chunk(&args.chunk_type).expect("chunk not found");

    png.save_file(args.file_path.as_path()).expect("could not write");

    Ok(())
}

/// Prints all of the chunks in a PNG file
pub fn print_chunks(args: PrintArgs) -> Result<()> {
    let png = Png::from_file(args.file_path.as_path()).expect("error reading png file");

    for c in png.chunks().iter() {
        if !c.chunk_type().is_critical() {
            println!("{}", c.data_as_string()?);
        }
    }

    Ok(())
}
