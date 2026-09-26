use regex::Regex;
use simple_text_decode::DecodedText;

use crate::extract::{Extractor, Gap, Quartet, ReadMode, ReleaseInfo, Ripper, TrackExtractor};
use crate::toc::{Toc, TocEntry, TocRaw};
use crate::track::{AccurateRipUnit, TestAndCopy, TrackEntry};
use crate::translate::TranslatorCombined;
use crate::util::Time;

use super::{IntegrityChecker, ParsedLog, ParsedLogCombined, Parser, ParserCombined, ParserTrack};

lazy_static! {
    static ref RIPPER_VERSION: Regex = Regex::new(r"Logfile created by: morituri (\S+)").unwrap();
    static ref ALBUM: Regex = Regex::new(r"(?m)^Album: (.+?) - (.+?)\s*$").unwrap();
    static ref DRIVE: Regex = Regex::new(r"(?m)^Drive: vendor (.+?), model (.+?)\s*$").unwrap();
    static ref READ_OFFSET: Regex = Regex::new(r"Read offset correction: ([+-]?\d+)").unwrap();
    static ref TOC_ENTRY: Regex =
        Regex::new(r"(?m)^\s+(?P<track>\d+)\s+\|\s+(?P<start>\d+) - \S+ \|\s+(?P<length>\d+) - \S+")
            .unwrap();
    static ref NO_CRC_CHECK: Regex = Regex::new(r"WARNING: no CRC check done").unwrap();

    static ref TRACK_SPLIT: Regex = Regex::new(r"(?m)^Track\s+(\d+)\s*$").unwrap();
    static ref T_FILENAME: Regex = Regex::new(r"(?m)^\s+Filename (.+?)\s*$").unwrap();
    static ref T_PREGAP: Regex = Regex::new(r"Pre-gap: (\S+)").unwrap();
    static ref T_PEAK: Regex = Regex::new(r"Peak level ([0-9.]+) %").unwrap();
    static ref T_COPY_SPEED: Regex = Regex::new(r"Extraction Speed \(Copy\) ([0-9.]+) X").unwrap();
    static ref T_COPY_CRC: Regex = Regex::new(r"Copy CRC ([0-9A-F]{8})").unwrap();
    static ref T_TEST_CRC: Regex = Regex::new(r"Test CRC ([0-9A-F]{8})").unwrap();
    static ref T_AR_OK: Regex =
        Regex::new(r"Accurately ripped \(confidence (?P<conf>\d+)\) \[(?P<crc>[0-9A-F]{8})\]").unwrap();
    static ref T_AR_BAD: Regex = Regex::new(
        r"Cannot be verified as accurate \[(?P<crc>[0-9A-F]{8})\], AccurateRip returned \[(?P<db>[0-9A-F]{8})\]"
    )
    .unwrap();
    static ref T_AR_ABSENT: Regex = Regex::new(r"Track not present in AccurateRip database").unwrap();
}

/// morituri counts in CD frames; 75 of them make a second.
fn frames_to_time(frames: u32) -> Time {
    Time::from_ss(&(f64::from(frames) / 75.0).to_string())
}

pub struct MorituriParser {
    encoded_log: DecodedText,
}

pub struct MorituriParserSingle {
    log: String,
}

pub struct MorituriParserTrack {
    num: u8,
    block: String,
}

impl MorituriParser {
    pub fn new(encoded_log: DecodedText) -> Self {
        MorituriParser { encoded_log }
    }
}

impl ParserCombined for MorituriParser {
    fn parse_combined(&self) -> ParsedLogCombined {
        let parsed_logs: Vec<ParsedLog> =
            vec![MorituriParserSingle::new(self.encoded_log.text.trim().to_owned()).parse()];

        ParsedLogCombined {
            parsed_logs,
            encoding: self.encoded_log.orig_encoding.to_string(),
        }
    }
}

impl TranslatorCombined for MorituriParser {
    fn translate_combined(&self) -> String {
        self.encoded_log.text.clone()
    }
}

impl MorituriParserSingle {
    pub fn new(log: String) -> Self {
        Self { log }
    }

    fn capture(&self, re: &Regex, group: usize) -> Option<String> {
        re.captures(&self.log)
            .and_then(|c| c.get(group))
            .map(|m| m.as_str().trim().to_owned())
    }

    fn track_blocks(&self) -> Vec<MorituriParserTrack> {
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
                MorituriParserTrack {
                    num,
                    block: self.log[start..end].to_owned(),
                }
            })
            .collect()
    }
}

impl Parser for MorituriParserSingle {}

impl Extractor for MorituriParserSingle {
    fn extract_ripper(&self) -> Ripper {
        Ripper::Morituri
    }

    fn extract_ripper_version(&self) -> String {
        self.capture(&RIPPER_VERSION, 1).unwrap_or(String::from("Unknown"))
    }

    fn extract_language(&self) -> String {
        String::from("English")
    }

    fn extract_release_info(&self) -> ReleaseInfo {
        match (self.capture(&ALBUM, 1), self.capture(&ALBUM, 2)) {
            (Some(artist), Some(title)) => ReleaseInfo::new(artist, title),
            _ => ReleaseInfo::default(),
        }
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

    /// cdparanoia is invoked with no paranoia-disabling flags, so full
    /// paranoia applies, the same as its descendant whipper.
    fn extract_read_mode(&self) -> ReadMode {
        ReadMode::Secure
    }

    fn extract_accurate_stream(&self) -> Quartet {
        Quartet::True
    }

    fn extract_defeat_audio_cache(&self) -> Quartet {
        Quartet::Unknown
    }

    fn extract_use_null_samples(&self) -> Quartet {
        Quartet::True
    }

    fn extract_gap_handling(&self) -> Gap {
        Gap::Append
    }

    fn extract_test_and_copy(&self) -> Quartet {
        match NO_CRC_CHECK.is_match(&self.log) {
            true => Quartet::False,
            false => match T_TEST_CRC.is_match(&self.log) {
                true => Quartet::True,
                false => Quartet::Unknown,
            },
        }
    }

    fn extract_toc(&self) -> Toc {
        let mut entries: Vec<TocEntry> = Vec::new();

        for c in TOC_ENTRY.captures_iter(&self.log) {
            let Ok(track) = c["track"].parse::<u32>() else { continue };
            let Ok(start) = c["start"].parse::<u32>() else { continue };
            let Ok(length) = c["length"].parse::<u32>() else { continue };

            entries.push(TocEntry::new(
                track,
                frames_to_time(start),
                frames_to_time(length),
                start,
                start + length - 1,
            ));
        }

        Toc::new(TocRaw::new(entries))
    }

    fn extract_tracks(&self) -> Vec<TrackEntry> {
        self.track_blocks().iter().map(|t| t.parse_track()).collect()
    }
}

impl IntegrityChecker for MorituriParserSingle {}

impl MorituriParserTrack {
    fn field(&self, re: &Regex) -> Option<String> {
        re.captures(&self.block)
            .and_then(|c| c.get(1))
            .map(|m| m.as_str().trim().to_owned())
    }
}

impl ParserTrack for MorituriParserTrack {}

impl TrackExtractor for MorituriParserTrack {
    fn extract_num(&self) -> u8 {
        self.num
    }

    fn extract_is_range(&self) -> bool {
        false
    }

    fn extract_filenames(&self) -> Vec<String> {
        match self.field(&T_FILENAME) {
            Some(f) => vec![f],
            None => Vec::new(),
        }
    }

    fn extract_peak_level(&self) -> Option<f64> {
        self.field(&T_PEAK)
            .and_then(|v| v.parse::<f64>().ok())
            .map(|v| v / 100.0)
    }

    fn extract_pregap_length(&self) -> Option<Time> {
        self.field(&T_PREGAP).map(|v| Time::from_mm_ss_cs(&v))
    }

    fn extract_extraction_speed(&self) -> Option<f64> {
        self.field(&T_COPY_SPEED).and_then(|v| v.parse::<f64>().ok())
    }

    fn extract_test_and_copy(&self) -> TestAndCopy {
        let copy = self.field(&T_COPY_CRC).unwrap_or_default();
        let test = self.field(&T_TEST_CRC).unwrap_or_default();
        TestAndCopy::new(test, copy, String::new(), String::new())
    }

    fn extract_ar_info(&self) -> Vec<AccurateRipUnit> {
        if let Some(c) = T_AR_OK.captures(&self.block) {
            let conf = c["conf"].parse::<u32>().unwrap_or_default();
            return vec![AccurateRipUnit::new_eac(1, c["crc"].to_owned(), conf)];
        }
        if let Some(c) = T_AR_BAD.captures(&self.block) {
            return vec![AccurateRipUnit::new_eac_mismatch(
                1,
                c["crc"].to_owned(),
                c["db"].to_owned(),
                0,
            )];
        }
        if T_AR_ABSENT.is_match(&self.block) {
            return vec![AccurateRipUnit::new_eac_notfound()];
        }
        Vec::new()
    }
}
