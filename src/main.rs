/**
 * @file    main.rs
 * @brief   A command-line tool to convert BCut JSON data into SRT subtitle files.
 * @author  GitHub@Akiko97 <mud.miscue_0l@icloud.com>
 *
 * Copyright(C) 2024, GitHub@Akiko97. All Right Reserved
 */

mod time;
mod srt;
mod bcut;

use std::path::PathBuf;
use std::io::{self, Read, Write};
use std::fs::File;
use crate::bcut::BCutData;
use crate::srt::{SrtBlock, SrtVecToString};
use crate::time::SrtTime;

fn main() {
    let m = clap::Command::new("bcut2srt")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Convert BCut JSON to SRT subtitle file")
        .subcommand_required(false)
        .args(&[
            clap::arg!(-i --input <file> "input file")
                .required(true)
                .value_parser(clap::value_parser!(PathBuf)),
            clap::arg!(-o --output <file> "output file")
                .required(true)
                .value_parser(clap::value_parser!(PathBuf))
        ])
        .get_matches();
    let input_file = m.get_one::<PathBuf>("input")
        .expect("parameter `input` is required");
    let output_file = m.get_one::<PathBuf>("output")
        .expect("parameter `output` is required");
    match read_file_to_string(input_file) {
        Ok(contents) => {
            let data: BCutData = serde_json::from_str(contents.as_str())
                .expect("Could not parse BCut JSON");
            let mut srt_blks = vec![];
            for track in data.tracks {
                if !track.clips.is_empty() {
                    if track.clips[0].asset_info.display_name != "字幕" { break }
                }
                track.clips.iter().for_each(|clip| {
                    let blk = SrtBlock::new(
                        SrtTime::new(clip.start, clip.start + clip.duration),
                        clip.asset_info.content.clone(),
                    );
                    srt_blks.push(blk);
                });
            }
            let srt = srt_blks.to_string();
            match write_file_to_string(output_file, &srt) {
                Ok(_) => println!("Successfully wrote SRT to file {}", output_file.display()),
                Err(e) => eprintln!("Error writing output file: {}", e),
            }
        }
        Err(e) => eprintln!("Error reading input file: {}", e),
    }
}

fn read_file_to_string(path: &PathBuf) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn write_file_to_string(path: &PathBuf, contents: &String) -> io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}
