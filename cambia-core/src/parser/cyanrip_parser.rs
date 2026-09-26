use regex::Regex;
use simple_text_decode::DecodedText;

use crate::extract::{Extractor, Gap, Quartet, ReadMode, ReleaseInfo, Ripper, TrackExtractor};
use crate::toc::{Toc, TocEntry, TocRaw};
use crate::track::{AccurateRipConfidence, AccurateRipOffset, AccurateRipStatus, AccurateRipUnit, TestAndCopy, TrackEntry};
use crate::translate::TranslatorCombined;
use crate::util::Time;

use super::{IntegrityChecker, ParsedLog, ParsedLogCombined, Parser, ParserCombined, ParserTrack};

mod fun512;

lazy_static! {
    static ref RIPPER_VERSION: Regex = Regex::new(r"^cyanrip ([^\s(]+)").unwrap();
    // Drive used: is newer than any release; every shipping version emits only
    // Device model:, and only the former is shaped like the AccurateRip entries
    static ref DRIVE_USED: Regex = Regex::new(r"Drive used:\s+(.+)").unwrap();
    static ref DEVICE_MODEL: Regex = Regex::new(r"Device model:\s+(.+)").unwrap();
    static ref OFFSET: Regex = Regex::new(r"Offset:\s+([+-]?\d+) samples?").unwrap();
    // the label itself flips to Underread when the value is negative
    static ref OVERREAD_MODE: Regex = Regex::new(r"(?:Over|Under)read mode:\s+(.+)").unwrap();
    static ref PARANOIA: Regex = Regex::new(r"Paranoia level:\s+(\S+)").unwrap();
    static ref OUTPUTS: Regex = Regex::new(r"Outputs:\s+(.+)").unwrap();
    static ref ALBUM: Regex = Regex::new(r"(?m)^Album:\s+(.+?)\s*$").unwrap();
    static ref ALBUM_ARTIST: Regex = Regex::new(r"(?m)^Album artist:\s+(.+?)\s*$").unwrap();

    static ref TRACK_SPLIT: Regex = Regex::new(r"(?m)^Track (\d+) ripped").unwrap();
    static ref TRACK_OK: Regex = Regex::new(r"^Track \d+ ripped and encoded successfully").unwrap();
    static ref T_DURATION: Regex = Regex::new(r"Duration:\s+([0-9:.]+)").unwrap();
    static ref T_START_LSN: Regex = Regex::new(r"Start LSN:\s+(\d+)").unwrap();
    static ref T_END_LSN: Regex = Regex::new(r"End LSN:\s+(\d+)").unwrap();
    static ref T_PREGAP: Regex = Regex::new(r"Pregap LSN:\s+\d+ \(duration: ([0-9:.]+)\)").unwrap();
    static ref T_PREEMPHASIS: Regex = Regex::new(r"(?m)^\s*Preemphasis:\s+(.+?)\s*$").unwrap();
    static ref T_CRC: Regex = Regex::new(r"EAC CRC32:\s+([0-9A-F]{8})").unwrap();
    static ref T_FILES_START: Regex = Regex::new(r"(?m)^\s*File\(s\):\s*$").unwrap();
    static ref LABEL: Regex = Regex::new(r"^[A-Za-z][A-Za-z0-9 ()]*:").unwrap();
    static ref T_GAIN: Regex = Regex::new(r"REPLAYGAIN_TRACK_GAIN:\s+([+-]?[0-9.]+) dB").unwrap();
    static ref T_PEAK: Regex = Regex::new(r"REPLAYGAIN_TRACK_PEAK:\s+([0-9.]+)").unwrap();

    // v0.5.x printed 0x-prefixed lowercase and had no status line at all;
    // the 450 checksum is skipped here, its label has no colon after the digit
    static ref T_AR_STATUS: Regex = Regex::new(r"(?m)^\s*Accurip:\s+(?P<state>.+?)\s*$").unwrap();
    static ref T_AR_SIGN: Regex = Regex::new(r"(?m)^\s*Accurip v(?P<ver>[12]):\s+(?:0x)?(?P<sign>[0-9a-fA-F]{8})(?P<rest>.*)$").unwrap();
    static ref T_AR_STATUS_CONF: Regex = Regex::new(r"\((?:max )?confidence: (\d+)\)").unwrap();
    static ref T_AR_MATCH: Regex = Regex::new(r"accurately ripped(?:, confidence (\d+))?").unwrap();
    static ref T_AR_DB_SIGN: Regex = Regex::new(r"Accurip DB of 0x([0-9a-fA-F]+)").unwrap();
}

pub struct CyanRipParser {
    encoded_log: DecodedText,
}

pub struct CyanRipParserSingle {
    log: String,
}

pub struct CyanRipParserTrack {
    num: u8,
    block: String,
}

impl CyanRipParser {
    pub fn new(encoded_log: DecodedText) -> Self {
        CyanRipParser { encoded_log }
    }
}

impl ParserCombined for CyanRipParser {
    fn parse_combined(&self) -> ParsedLogCombined {
        let parsed_logs: Vec<ParsedLog> =
            vec![CyanRipParserSingle::new(self.encoded_log.text.trim().to_owned()).parse()];

        ParsedLogCombined {
            parsed_logs,
            encoding: self.encoded_log.orig_encoding.to_string(),
        }
    }
}

impl TranslatorCombined for CyanRipParser {
    fn translate_combined(&self) -> String {
        self.encoded_log.text.clone()
    }
}

impl CyanRipParserSingle {
    pub fn new(log: String) -> Self {
        Self { log }
    }

    fn capture(&self, re: &Regex) -> Option<String> {
        re.captures(&self.log)
            .map(|c| c.get(1).unwrap().as_str().trim().to_owned())
    }

    /// The per-track sections, sliced so field order and indentation, both of
    /// which moved between versions, do not matter.
    fn track_blocks(&self) -> Vec<CyanRipParserTrack> {
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
                CyanRipParserTrack {
                    num,
                    block: self.log[start..end].to_owned(),
                }
            })
            .collect()
    }
}

impl Parser for CyanRipParserSingle {}

impl Extractor for CyanRipParserSingle {
    fn extract_ripper(&self) -> Ripper {
        Ripper::CyanRip
    }

    fn extract_ripper_version(&self) -> String {
        self.capture(&RIPPER_VERSION).unwrap_or(String::from("Unknown"))
    }

    fn extract_language(&self) -> String {
        String::from("English")
    }

    fn extract_release_info(&self) -> ReleaseInfo {
        match (self.capture(&ALBUM_ARTIST), self.capture(&ALBUM)) {
            (Some(artist), Some(album)) => ReleaseInfo::new(artist, album),
            _ => ReleaseInfo::default(),
        }
    }

    fn extract_drive(&self) -> String {
        self.capture(&DRIVE_USED)
            .or_else(|| self.capture(&DEVICE_MODEL))
            .unwrap_or_default()
    }

    fn extract_read_offset(&self) -> Option<i16> {
        self.capture(&OFFSET).and_then(|v| v.parse::<i16>().ok())
    }

    fn extract_overread(&self) -> Quartet {
        match self.capture(&OVERREAD_MODE) {
            Some(mode) if mode.starts_with("read in") => Quartet::True,
            Some(_) => Quartet::False,
            None => Quartet::Unknown,
        }
    }

    fn extract_fill_silence(&self) -> Quartet {
        match self.capture(&OVERREAD_MODE) {
            Some(mode) if mode.starts_with("fill with silence") => Quartet::True,
            Some(_) => Quartet::False,
            None => Quartet::Unknown,
        }
    }

    fn extract_read_mode(&self) -> ReadMode {
        match self.capture(&PARANOIA).as_deref() {
            Some("max") => ReadMode::Paranoid,
            Some("none") => ReadMode::Burst,
            Some(_) => ReadMode::Secure,
            None => ReadMode::Unknown,
        }
    }

    fn extract_gap_handling(&self) -> Gap {
        Gap::AppendNoHtoa
    }

    fn extract_audio_encoder(&self) -> Vec<String> {
        match self.capture(&OUTPUTS) {
            Some(outputs) => outputs.split(',').map(|o| o.trim().to_owned()).collect(),
            None => Vec::new(),
        }
    }

    fn extract_toc(&self) -> Toc {
        let entries: Vec<TocEntry> = self
            .track_blocks()
            .iter()
            .filter_map(|t| {
                let start = t.field(&T_START_LSN)?.parse::<u32>().ok()?;
                let end = t.field(&T_END_LSN)?.parse::<u32>().ok()?;
                let zero = Time::from_ss("0");
                let length = t
                    .field(&T_DURATION)
                    .map_or(zero, |d| Time::from_h_mm_ss(&d));
                Some(TocEntry::new(
                    t.num as u32,
                    Time::from_ss(&(f64::from(start) / 75.0).to_string()),
                    length,
                    start,
                    end,
                ))
            })
            .collect();

        Toc::new(TocRaw::new(entries))
    }

    fn extract_tracks(&self) -> Vec<TrackEntry> {
        self.track_blocks().iter().map(|t| t.parse_track()).collect()
    }
}

impl IntegrityChecker for CyanRipParserSingle {
    fn extract_checksum(&self) -> String {
        fun512::extract(&self.log).unwrap_or_default().to_owned()
    }

    fn calculate_checksum(&self) -> String {
        fun512::calculate(&self.log)
    }
}

impl CyanRipParserTrack {
    fn field(&self, re: &Regex) -> Option<String> {
        re.captures(&self.block)
            .map(|c| c.get(1).unwrap().as_str().trim().to_owned())
    }
}

impl ParserTrack for CyanRipParserTrack {}

impl TrackExtractor for CyanRipParserTrack {
    fn extract_num(&self) -> u8 {
        self.num
    }

    fn extract_is_range(&self) -> bool {
        false
    }

    fn extract_is_aborted(&self) -> bool {
        !TRACK_OK.is_match(&self.block)
    }

    fn extract_filenames(&self) -> Vec<String> {
        let Some(m) = T_FILES_START.find(&self.block) else {
            return Vec::new();
        };

        self.block[m.end()..]
            .lines()
            .skip(1)
            .map(str::trim)
            .take_while(|l| !l.is_empty() && !LABEL.is_match(l))
            .map(str::to_owned)
            .collect()
    }

    fn extract_peak_level(&self) -> Option<f64> {
        self.field(&T_PEAK).and_then(|v| v.parse::<f64>().ok())
    }

    fn extract_pregap_length(&self) -> Option<Time> {
        self.field(&T_PREGAP).map(|v| Time::from_h_mm_ss(&v))
    }

    fn extract_gain(&self) -> Option<f64> {
        self.field(&T_GAIN).and_then(|v| v.parse::<f64>().ok())
    }

    fn extract_preemphasis(&self) -> Option<bool> {
        self.field(&T_PREEMPHASIS).map(|v| !v.starts_with("none"))
    }

    /// Only one read happens, so the copy hash stands alone; claiming it as a
    /// test hash too would report a match that was never verified.
    fn extract_test_and_copy(&self) -> TestAndCopy {
        match self.field(&T_CRC) {
            Some(crc) => TestAndCopy::new(String::new(), crc, String::new(), String::new()),
            None => TestAndCopy::new(String::new(), String::new(), String::new(), String::new()),
        }
    }

    fn extract_ar_info(&self) -> Vec<AccurateRipUnit> {
        let state = self.field(&T_AR_STATUS).unwrap_or_default();
        let disabled = state.starts_with("disabled");
        let found = state.starts_with("found") || state.starts_with("disc found");
        let status_conf = T_AR_STATUS_CONF
            .captures(&state)
            .and_then(|c| c[1].parse::<u32>().ok());

        T_AR_SIGN
            .captures_iter(&self.block)
            .map(|c| {
                let sign = c["sign"].to_uppercase();
                let rest = &c["rest"];

                let (status, matching) = match T_AR_MATCH.captures(rest) {
                    // v0.6.0 reported confidence once on the status line instead
                    Some(m) => (
                        AccurateRipStatus::Match,
                        m.get(1).and_then(|v| v.as_str().parse::<u32>().ok()).or(status_conf),
                    ),
                    None if disabled => (AccurateRipStatus::Disabled, None),
                    // an unannotated checksum means the other version matched
                    None if found => (AccurateRipStatus::Mismatch, None),
                    None => (AccurateRipStatus::NotFound, None),
                };

                let offset_sign = match status {
                    AccurateRipStatus::Match => sign.clone(),
                    // v0.6.0 printed the database checksum unpadded
                    _ => T_AR_DB_SIGN
                        .captures(rest)
                        .map(|d| format!("{:0>8}", d[1].to_uppercase()))
                        .unwrap_or_default(),
                };

                AccurateRipUnit::new(
                    c["ver"].parse::<u8>().ok(),
                    sign,
                    offset_sign,
                    matching.map(|n| {
                        AccurateRipConfidence::new(Some(n), None, AccurateRipOffset::Same)
                    }),
                    status,
                )
            })
            .collect()
    }
}
