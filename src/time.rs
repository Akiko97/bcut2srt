/**
 * @file    time.rs
 * @brief   Implements time management for SRT subtitles.
 * @author  GitHub@Akiko97 <mud.miscue_0l@icloud.com>
 *
 * Copyright(C) 2024, GitHub@Akiko97. All Right Reserved
 */

struct Time {
    ms: u64,
}

impl Time {
    pub fn new(ms: u64) -> Self {
        Self { ms }
    }
    pub fn to_srt_format(&self) -> String {
        let hours = self.ms / 3_600_000;
        let minutes = (self.ms % 3_600_000) / 60_000;
        let seconds = (self.ms % 60_000) / 1_000;
        let milliseconds = self.ms % 1_000;

        format!("{:02}:{:02}:{:02},{:03}", hours, minutes, seconds, milliseconds)
    }
}

pub struct SrtTime {
    start: Time,
    end: Time,
}

impl SrtTime {
    pub fn new(start: u64, end: u64) -> Self {
        Self {
            start: Time::new(start),
            end: Time::new(end),
        }
    }

    pub fn to_string(&self) -> String {
        format!("{} --> {}", self.start.to_srt_format(), self.end.to_srt_format())
    }
}
