/**
 * @file    srt.rs
 * @brief   Provides functionality for representing and formatting SRT subtitle blocks.
 * @author  GitHub@Akiko97 <mud.miscue_0l@icloud.com>
 *
 * Copyright(C) 2024, GitHub@Akiko97. All Right Reserved
 */

use crate::time::SrtTime;
use std::fmt::Write;

pub struct SrtBlock {
    time: SrtTime,
    text: String,
}

impl SrtBlock {
    pub fn new(time: SrtTime, text: String) -> SrtBlock {
        Self {time, text}
    }
    pub fn to_string(&self) -> String {
        let mut srt_block = String::new();
        writeln!(&mut srt_block, "{}", self.time.to_string()).unwrap();
        writeln!(&mut srt_block, "{}", self.text).unwrap();
        srt_block
    }
}

pub trait SrtVecToString {
    fn to_string(&self) -> String;
}

impl SrtVecToString for Vec<SrtBlock> {
    fn to_string(&self) -> String {
        let mut srt = String::new();
        self.iter().enumerate().for_each(|(index, srt_block)| {
            writeln!(&mut srt, "{}", index).unwrap();
            writeln!(&mut srt, "{}", srt_block.to_string()).unwrap();
            srt.push_str("\n");
        });
        srt
    }
}
