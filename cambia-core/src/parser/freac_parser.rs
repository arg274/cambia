use regex::Regex;
use simple_text_decode::DecodedText;

use crate::extract::{Extractor, ReleaseInfo, Ripper, TrackExtractor};
use crate::toc::{Toc, TocEntry, TocRaw};
use crate::track::{AccurateRipConfidence, AccurateRipOffset, AccurateRipStatus, AccurateRipUnit, TestAndCopy, TrackEntry};
use crate::translate::TranslatorCombined;
use crate::util::Time;

use super::{IntegrityChecker, ParsedLog, ParsedLogCombined, Parser, ParserCombined, ParserTrack};

lazy_static! {
    static ref RIPPER_VERSION: Regex = Regex::new(r"Version:\s+v?(\S+)").unwrap();
    static ref DRIVE: Regex = Regex::new(r"(?m)Drive model:\s+(.+?)\s*$").unwrap();
    static ref READ_OFFSET: Regex = Regex::new(r"Read offset:\s+([+-]?\d+) samples").unwrap();
    static ref ENCODER: Regex = Regex::new(r"(?m)Selected encoder:\s+(.+?)\s*$").unwrap();
    static ref ARTIST: Regex = Regex::new(r"(?m)Artist:\s+(.+?)\s*$").unwrap();
    static ref ALBUM: Regex = Regex::new(r"(?m)Album:\s+(.+?)\s*$").unwrap();
    static ref TOC_ROW: Regex = Regex::new(
        r"(?m)^\S+\s+(?P<track>\d+) \| (?P<start>[\d:.]+) \| (?P<length>[\d:.]+) \|\s+(?P<first>\d+) \|\s+(?P<last>\d+)"
    ).unwrap();

    static ref TRACK_SPLIT: Regex = Regex::new(r"Ripping: device://cdda:\d+/(\d+)").unwrap();
    static ref T_FILENAME: Regex = Regex::new(r"(?m)^\S+\s+to: (.+?)\s*$").unwrap();
    static ref T_CRC: Regex = Regex::new(r"CRC checksum: ([0-9a-fA-F]+)").unwrap();
    static ref T_AR: Regex = Regex::new(
        r"Checksum \(AccurateRip v(?P<ver>\d+)\): (?P<crc>[0-9a-fA-F]+), Confidence: (?P<conf>\d+)"
    ).unwrap();
    static ref T_ACCURATE: Regex = Regex::new(r"Track has been accurately ripped").unwrap();
    static ref T_SPEED: Regex = Regex::new(r"Duration: [\d:.]+ \(([\d.]+)x speed\)").unwrap();
}

/// MM:SS.FF, where FF counts CD frames.
fn msf_to_time(v: &str) -> Time {
    let secs = (|| {
        let (mm, rest) = v.split_once(':')?;
        let (ss, ff) = rest.split_once('.')?;
        Some(mm.parse::<f64>().ok()? * 60.0 + ss.parse::<f64>().ok()? + ff.parse::<f64>().ok()? / 75.0)
    })()
    .unwrap_or(0.0);
    Time::from_ss(&secs.to_string())
}

pub struct FreacParser {
    encoded_log: DecodedText,
}

pub struct FreacParserSingle {
    log: String,
}

pub struct FreacParserTrack {
    num: u8,
    block: String,
}

impl FreacParser {
    pub fn new(encoded_log: DecodedText) -> Self {
        FreacParser { encoded_log }
    }
}

impl ParserCombined for FreacParser {
    fn parse_combined(&self) -> ParsedLogCombined {
        let parsed_logs: Vec<ParsedLog> =
            vec![FreacParserSingle::new(self.encoded_log.text.trim().to_owned()).parse()];

        ParsedLogCombined {
            parsed_logs,
            encoding: self.encoded_log.orig_encoding.to_string(),
        }
    }
}

impl TranslatorCombined for FreacParser {
    fn translate_combined(&self) -> String {
        self.encoded_log.text.clone()
    }
}

impl FreacParserSingle {
    pub fn new(log: String) -> Self {
        Self { log }
    }

    fn capture(&self, re: &Regex, group: usize) -> Option<String> {
        re.captures(&self.log)
            .and_then(|c| c.get(group))
            .map(|m| m.as_str().trim().to_owned())
    }

    /// Each track spans a Ripping / verified / Finished run of entries.
    fn track_blocks(&self) -> Vec<FreacParserTrack> {
        let starts: Vec<(usize, u8)> = TRACK_SPLIT
            .captures_iter(&self.log)
            .map(|c| {
                (
                    c.get(0).unwrap().start(),
                    c.get(1).unwrap().as_str().parse::<u8>().unwrap_or_default(),
                )
            })
            .collect();

        starts
            .iter()
            .enumerate()
            .map(|(i, &(start, num))| {
                let end = starts.get(i + 1).map_or(self.log.len(), |&(s, _)| s);
                FreacParserTrack {
                    num,
                    block: self.log[start..end].to_owned(),
                }
            })
            .collect()
    }
}

impl Parser for FreacParserSingle {}

impl Extractor for FreacParserSingle {
    fn extract_ripper(&self) -> Ripper {
        Ripper::FreAc
    }

    fn extract_ripper_version(&self) -> String {
        self.capture(&RIPPER_VERSION, 1).unwrap_or(String::from("Unknown"))
    }

    fn extract_language(&self) -> String {
        String::from("English")
    }

    fn extract_release_info(&self) -> ReleaseInfo {
        match (self.capture(&ARTIST, 1), self.capture(&ALBUM, 1)) {
            (Some(artist), Some(album)) => ReleaseInfo::new(artist, album),
            _ => ReleaseInfo::default(),
        }
    }

    fn extract_drive(&self) -> String {
        self.capture(&DRIVE, 1).unwrap_or_default()
    }

    fn extract_read_offset(&self) -> Option<i16> {
        self.capture(&READ_OFFSET, 1).and_then(|v| v.parse::<i16>().ok())
    }

    fn extract_audio_encoder(&self) -> Vec<String> {
        match self.capture(&ENCODER, 1) {
            Some(e) => vec![e],
            None => Vec::new(),
        }
    }

    fn extract_toc(&self) -> Toc {
        let entries: Vec<TocEntry> = TOC_ROW
            .captures_iter(&self.log)
            .filter_map(|c| {
                Some(TocEntry::new(
                    c["track"].parse::<u32>().ok()?,
                    msf_to_time(&c["start"]),
                    msf_to_time(&c["length"]),
                    c["first"].parse::<u32>().ok()?,
                    c["last"].parse::<u32>().ok()?,
                ))
            })
            .collect();

        Toc::new(TocRaw::new(entries))
    }

    fn extract_tracks(&self) -> Vec<TrackEntry> {
        self.track_blocks().iter().map(|t| t.parse_track()).collect()
    }
}

impl IntegrityChecker for FreacParserSingle {}

impl FreacParserTrack {
    fn field(&self, re: &Regex, group: usize) -> Option<String> {
        re.captures(&self.block)
            .and_then(|c| c.get(group))
            .map(|m| m.as_str().trim().to_owned())
    }
}

impl ParserTrack for FreacParserTrack {}

impl TrackExtractor for FreacParserTrack {
    fn extract_num(&self) -> u8 {
        self.num
    }

    fn extract_is_range(&self) -> bool {
        false
    }

    /// A track that never reaches its "Finished ripping" entry has no CRC.
    fn extract_is_aborted(&self) -> bool {
        !T_CRC.is_match(&self.block)
    }

    fn extract_filenames(&self) -> Vec<String> {
        match self.field(&T_FILENAME, 1) {
            Some(f) => vec![f],
            None => Vec::new(),
        }
    }

    fn extract_extraction_speed(&self) -> Option<f64> {
        self.field(&T_SPEED, 1).and_then(|v| v.parse::<f64>().ok())
    }

    /// One read, so the CRC stands alone with nothing to compare against.
    fn extract_test_and_copy(&self) -> TestAndCopy {
        TestAndCopy::new(
            String::new(),
            self.field(&T_CRC, 1).unwrap_or_default().to_uppercase(),
            String::new(),
            String::new(),
        )
    }

    fn extract_ar_info(&self) -> Vec<AccurateRipUnit> {
        let Some(c) = T_AR.captures(&self.block) else {
            return Vec::new();
        };

        let crc = c["crc"].to_uppercase();
        let status = match T_ACCURATE.is_match(&self.block) {
            true => AccurateRipStatus::Match,
            false => AccurateRipStatus::Mismatch,
        };

        vec![AccurateRipUnit::new(
            c["ver"].parse::<u8>().ok(),
            crc.clone(),
            crc,
            c["conf"].parse::<u32>().ok().map(|n| {
                AccurateRipConfidence::new(Some(n), None, AccurateRipOffset::Same)
            }),
            status,
        )]
    }
}
