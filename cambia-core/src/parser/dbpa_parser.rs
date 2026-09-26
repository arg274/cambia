use regex::Regex;
use simple_text_decode::DecodedText;

use crate::extract::{Extractor, Quartet, ReadMode, Ripper, TrackExtractor};
use crate::toc::{Toc, TocEntry, TocRaw};
use crate::track::{AccurateRipStatus, AccurateRipUnit, TestAndCopy, TrackEntry};
use crate::translate::TranslatorCombined;
use crate::util::Time;

use super::{IntegrityChecker, ParsedLog, ParsedLogCombined, Parser, ParserCombined, ParserTrack};

lazy_static! {
    // macOS builds omit the "Release x.y" entirely
    static ref RIPPER_VERSION: Regex = Regex::new(r"dBpoweramp Release (\S+)").unwrap();
    static ref DRIVE: Regex = Regex::new(r"Ripping with drive '[^\[]*\[\s*(?P<vendor>.+?)\s+-\s+(?P<model>.+?)\s*\]'").unwrap();
    static ref READ_OFFSET: Regex = Regex::new(r"Drive offset:\s*([+-]?\d+)").unwrap();
    static ref OVERREAD: Regex = Regex::new(r"Overread Lead-in/out:\s*(Yes|No)").unwrap();
    static ref USE_C2: Regex = Regex::new(r"Using C2:\s*(Yes|No)").unwrap();
    static ref ENCODER: Regex = Regex::new(r"(?m)^Encoder:\s*(.+?)\s*$").unwrap();
    static ref SECOND_PASS: Regex = Regex::new(r"Pass 2 Drive Speed:").unwrap();
    static ref TWO_PASS_TRACK: Regex = Regex::new(r"\[Pass \d+ & \d+").unwrap();

    static ref TRACK_SPLIT: Regex = Regex::new(r"(?m)^Track (\d+):").unwrap();
    static ref T_RIPPED: Regex = Regex::new(
        r"Ripped LBA (?P<start>\d+) to (?P<end>\d+) \((?P<len>[0-9:]+)\) in (?P<took>[0-9:]+)\."
    ).unwrap();
    static ref T_FILENAME: Regex = Regex::new(r"(?m)Filename: (.+?)\s*$").unwrap();
    static ref T_CRC: Regex = Regex::new(r"CRC32: ([0-9A-Fa-f]+)").unwrap();
    // the AccurateRip CRC here is zero-padded, the one on the Verified line is not
    static ref T_AR_CRC: Regex = Regex::new(r"AccurateRip CRC: ([0-9A-Fa-f]+)(?: \(CRCv(?P<ver>\d+)\))?").unwrap();
    static ref T_AR_STATUS: Regex =
        Regex::new(r"AccurateRip: (?P<verdict>Accurate|Inaccurate) \(confidence (?P<conf>\d+)\)").unwrap();
    static ref T_AR_VERIFIED: Regex = Regex::new(
        r"AccurateRip Verified Confidence (?P<conf>\d+) \[CRCv(?P<ver>\d+) (?P<crc>[0-9A-Fa-f]+)\](?:, Using Pressing Offset (?P<offset>[+-]\d+))?"
    ).unwrap();
    static ref T_SECURE: Regex = Regex::new(r"Secure(?P<warn> \(Warning\))?\s+\[Pass").unwrap();
    static ref T_RERIP: Regex = Regex::new(r"Re-rip Frame: \d+").unwrap();
}

fn frames_to_time(frames: u32) -> Time {
    Time::from_ss(&(f64::from(frames) / 75.0).to_string())
}

fn mm_ss_to_secs(v: &str) -> Option<f64> {
    let (m, s) = v.split_once(':')?;
    Some(m.parse::<f64>().ok()? * 60.0 + s.parse::<f64>().ok()?)
}

pub struct DbpaParser {
    encoded_log: DecodedText,
}

pub struct DbpaParserSingle {
    log: String,
}

pub struct DbpaParserTrack {
    num: u8,
    block: String,
}

impl DbpaParser {
    pub fn new(encoded_log: DecodedText) -> Self {
        DbpaParser { encoded_log }
    }
}

impl ParserCombined for DbpaParser {
    fn parse_combined(&self) -> ParsedLogCombined {
        let parsed_logs: Vec<ParsedLog> =
            vec![DbpaParserSingle::new(self.encoded_log.text.trim().to_owned()).parse()];

        ParsedLogCombined {
            parsed_logs,
            encoding: self.encoded_log.orig_encoding.to_string(),
        }
    }
}

impl TranslatorCombined for DbpaParser {
    fn translate_combined(&self) -> String {
        self.encoded_log.text.clone()
    }
}

impl DbpaParserSingle {
    pub fn new(log: String) -> Self {
        Self { log }
    }

    fn capture(&self, re: &Regex, group: usize) -> Option<String> {
        re.captures(&self.log)
            .and_then(|c| c.get(group))
            .map(|m| m.as_str().trim().to_owned())
    }

    fn yes_no(&self, re: &Regex) -> Quartet {
        match self.capture(re, 1).as_deref() {
            Some("Yes") => Quartet::True,
            Some("No") => Quartet::False,
            _ => Quartet::Unknown,
        }
    }

    fn track_blocks(&self) -> Vec<DbpaParserTrack> {
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
                DbpaParserTrack {
                    num,
                    block: self.log[start..end].to_owned(),
                }
            })
            .collect()
    }
}

impl Parser for DbpaParserSingle {}

impl Extractor for DbpaParserSingle {
    fn extract_ripper(&self) -> Ripper {
        Ripper::DBPA
    }

    fn extract_ripper_version(&self) -> String {
        self.capture(&RIPPER_VERSION, 1).unwrap_or(String::from("Unknown"))
    }

    fn extract_language(&self) -> String {
        String::from("English")
    }

    fn extract_drive(&self) -> String {
        match (self.capture(&DRIVE, 1), self.capture(&DRIVE, 2)) {
            (Some(vendor), Some(model)) => format!("{vendor} {model}"),
            _ => String::default(),
        }
    }

    fn extract_read_offset(&self) -> Option<i16> {
        self.capture(&READ_OFFSET, 1).and_then(|v| v.parse::<i16>().ok())
    }

    fn extract_overread(&self) -> Quartet {
        self.yes_no(&OVERREAD)
    }

    fn extract_use_c2(&self) -> Quartet {
        self.yes_no(&USE_C2)
    }

    /// Two configured passes is what dBpoweramp calls secure ripping; a log
    /// without one says nothing either way.
    fn extract_read_mode(&self) -> ReadMode {
        match SECOND_PASS.is_match(&self.log) {
            true => ReadMode::Secure,
            false => ReadMode::Unknown,
        }
    }

    /// A track only reaches a second pass when the first was not verified, so
    /// the comparison is evidence of a test pass having happened.
    fn extract_test_and_copy(&self) -> Quartet {
        match TWO_PASS_TRACK.is_match(&self.log) {
            true => Quartet::True,
            false => Quartet::False,
        }
    }

    fn extract_audio_encoder(&self) -> Vec<String> {
        match self.capture(&ENCODER, 1) {
            Some(enc) => vec![enc.split_whitespace().next().unwrap_or_default().to_owned()],
            None => Vec::new(),
        }
    }

    fn extract_toc(&self) -> Toc {
        let entries: Vec<TocEntry> = self
            .track_blocks()
            .iter()
            .filter_map(|t| {
                let c = T_RIPPED.captures(&t.block)?;
                let start = c["start"].parse::<u32>().ok()?;
                let end = c["end"].parse::<u32>().ok()?;
                Some(TocEntry::new(
                    u32::from(t.num),
                    frames_to_time(start),
                    frames_to_time(end.saturating_sub(start)),
                    start,
                    end.saturating_sub(1),
                ))
            })
            .collect();

        Toc::new(TocRaw::new(entries))
    }

    fn extract_tracks(&self) -> Vec<TrackEntry> {
        self.track_blocks().iter().map(|t| t.parse_track()).collect()
    }
}

impl IntegrityChecker for DbpaParserSingle {}

impl DbpaParserTrack {
    fn field(&self, re: &Regex, group: usize) -> Option<String> {
        re.captures(&self.block)
            .and_then(|c| c.get(group))
            .map(|m| m.as_str().trim().to_owned())
    }
}

impl ParserTrack for DbpaParserTrack {}

impl TrackExtractor for DbpaParserTrack {
    fn extract_num(&self) -> u8 {
        self.num
    }

    fn extract_is_range(&self) -> bool {
        false
    }

    /// Every completed track prints a CRC32. A log cut short by "User Stopped
    /// Ripping" leaves its final track without one.
    fn extract_is_aborted(&self) -> bool {
        !T_CRC.is_match(&self.block)
    }

    fn extract_filenames(&self) -> Vec<String> {
        match self.field(&T_FILENAME, 1) {
            Some(f) => vec![f],
            None => Vec::new(),
        }
    }

    /// Not reported, but the track length and the time it took both are.
    fn extract_extraction_speed(&self) -> Option<f64> {
        let c = T_RIPPED.captures(&self.block)?;
        let len = mm_ss_to_secs(&c["len"])?;
        let took = mm_ss_to_secs(&c["took"])?;
        match took > 0.0 {
            true => Some(len / took),
            false => None,
        }
    }

    /// One CRC is printed however many passes ran, so there is no second hash
    /// to compare it against.
    fn extract_test_and_copy(&self) -> TestAndCopy {
        TestAndCopy::new(
            String::new(),
            self.field(&T_CRC, 1).unwrap_or_default().to_uppercase(),
            String::new(),
            String::new(),
        )
    }

    fn extract_ar_info(&self) -> Vec<AccurateRipUnit> {
        let version = T_AR_VERIFIED
            .captures(&self.block)
            .and_then(|c| c.name("ver"))
            .or_else(|| T_AR_CRC.captures(&self.block).and_then(|c| c.name("ver")))
            .and_then(|m| m.as_str().parse::<u8>().ok())
            .unwrap_or(1);

        let crc = self
            .field(&T_AR_CRC, 1)
            .unwrap_or_default()
            .to_uppercase();

        match T_AR_STATUS.captures(&self.block) {
            Some(c) => {
                let conf = c["conf"].parse::<u32>().unwrap_or_default();
                match &c["verdict"] {
                    "Accurate" => vec![AccurateRipUnit::new_eac(version, crc, conf)],
                    _ => vec![AccurateRipUnit::new_eac_mismatch(
                        version,
                        crc.clone(),
                        crc,
                        conf,
                    )],
                }
            }
            // No verdict means the disc was not in the database, but the CRC
            // was still computed and is worth keeping.
            None if !crc.is_empty() => vec![AccurateRipUnit::new(
                Some(version),
                crc.clone(),
                crc,
                None,
                AccurateRipStatus::NotFound,
            )],
            None => Vec::new(),
        }
    }
}
