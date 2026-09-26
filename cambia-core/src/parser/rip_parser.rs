use regex::Regex;
use simple_text_decode::DecodedText;

use crate::extract::{Extractor, Quartet, ReleaseInfo, Ripper, TrackExtractor};
use crate::toc::{Toc, TocEntry, TocRaw};
use crate::track::{AccurateRipConfidence, AccurateRipOffset, AccurateRipStatus, AccurateRipUnit, TrackEntry};
use crate::translate::TranslatorCombined;
use crate::util::Time;

use super::{IntegrityChecker, ParsedLog, ParsedLogCombined, Parser, ParserCombined, ParserTrack};

lazy_static! {
    static ref RIPPER_VERSION: Regex = Regex::new(r"^Rip (.+?) Audio Extraction Log").unwrap();
    static ref DRIVE: Regex = Regex::new(r"(?m)^Drive used:\s+(.+?)\s*$").unwrap();
    // the parenthesised firmware revision is not part of the AccurateRip name
    static ref DRIVE_REVISION: Regex = Regex::new(r"\s*\([^)]*\)\s*$").unwrap();
    static ref READ_OFFSET: Regex = Regex::new(r"(?m)^Read offset:\s+([+-]?\d+)").unwrap();
    static ref STREAM_ACCURATE: Regex = Regex::new(r"(?m)^Stream accurate:\s+(Yes|No)").unwrap();
    static ref NAME: Regex = Regex::new(r"(?m)^Name:\s+(.+?) - (.+?)\s*$").unwrap();
    static ref COPY_VERIFIED: Regex = Regex::new(r"Copy verified").unwrap();
    static ref TOC_ROW: Regex = Regex::new(
        r"(?m)^\s*(?P<track>\d+)\s*\|\s*(?P<start>[0-9:.]+)\s*\|\s*[0-9:.]+\s*\|\s*(?P<dur>[0-9:.]+)\s*\|\s*(?P<first>\d+)\s*\|\s*(?P<last>\d+)\s*\|"
    ).unwrap();

    static ref TRACK_SPLIT: Regex = Regex::new(r"(?m)^Track (\d+) saved to (.+?)\s*$").unwrap();
    static ref T_AR_CRC: Regex = Regex::new(r"AccurateRip checksum:\s+([0-9a-fA-F]+)").unwrap();
    static ref T_ACCURATE: Regex = Regex::new(r"Accurately ripped:\s+(Yes|No)").unwrap();
    static ref T_CONFIDENCE: Regex = Regex::new(r"Confidence level:\s+(\d+)").unwrap();
    static ref T_AR_FAILED: Regex = Regex::new(r"AccurateRip verification failed").unwrap();
    static ref T_PEAK: Regex = Regex::new(r"(?m)^\s*Peak level:\s+([0-9.]+)").unwrap();
    static ref T_GAIN: Regex = Regex::new(r"(?m)^\s*Replay gain:\s+([+-]?[0-9.]+) dB").unwrap();
}

/// Rip prints AccurateRip checksums sign-extended to 64 bits, so
/// ffffffffdf016c86 is really df016c86.
fn low32(hex: &str) -> String {
    let s = hex.trim_start_matches('0');
    let take = s.len().min(8);
    hex[hex.len() - take.max(1).min(hex.len())..].to_uppercase()
}

/// MM:SS.FF, where FF counts CD frames rather than hundredths.
fn msf_to_time(v: &str) -> Time {
    let secs = (|| {
        let (mm, rest) = v.split_once(':')?;
        let (ss, ff) = rest.split_once('.')?;
        Some(mm.parse::<f64>().ok()? * 60.0 + ss.parse::<f64>().ok()? + ff.parse::<f64>().ok()? / 75.0)
    })()
    .unwrap_or(0.0);
    Time::from_ss(&secs.to_string())
}

pub struct RipParser {
    encoded_log: DecodedText,
}

pub struct RipParserSingle {
    log: String,
}

pub struct RipParserTrack {
    num: u8,
    filename: String,
    block: String,
}

impl RipParser {
    pub fn new(encoded_log: DecodedText) -> Self {
        RipParser { encoded_log }
    }
}

impl ParserCombined for RipParser {
    fn parse_combined(&self) -> ParsedLogCombined {
        let parsed_logs: Vec<ParsedLog> =
            vec![RipParserSingle::new(self.encoded_log.text.trim().to_owned()).parse()];

        ParsedLogCombined {
            parsed_logs,
            encoding: self.encoded_log.orig_encoding.to_string(),
        }
    }
}

impl TranslatorCombined for RipParser {
    fn translate_combined(&self) -> String {
        self.encoded_log.text.clone()
    }
}

impl RipParserSingle {
    pub fn new(log: String) -> Self {
        Self { log }
    }

    fn capture(&self, re: &Regex, group: usize) -> Option<String> {
        re.captures(&self.log)
            .and_then(|c| c.get(group))
            .map(|m| m.as_str().trim().to_owned())
    }

    fn track_blocks(&self) -> Vec<RipParserTrack> {
        let marks: Vec<(usize, usize, u8, String)> = TRACK_SPLIT
            .captures_iter(&self.log)
            .map(|c| {
                let m = c.get(0).unwrap();
                (
                    m.start(),
                    m.end(),
                    c.get(1).unwrap().as_str().parse::<u8>().unwrap_or_default(),
                    c.get(2).unwrap().as_str().trim().to_owned(),
                )
            })
            .collect();

        marks
            .iter()
            .enumerate()
            .map(|(i, (_, body, num, filename))| {
                let end = marks.get(i + 1).map_or(self.log.len(), |&(s, _, _, _)| s);
                RipParserTrack {
                    num: *num,
                    filename: filename.clone(),
                    block: self.log[*body..end].to_owned(),
                }
            })
            .collect()
    }
}

impl Parser for RipParserSingle {}

impl Extractor for RipParserSingle {
    fn extract_ripper(&self) -> Ripper {
        Ripper::Rip
    }

    fn extract_ripper_version(&self) -> String {
        self.capture(&RIPPER_VERSION, 1).unwrap_or(String::from("Unknown"))
    }

    fn extract_language(&self) -> String {
        String::from("English")
    }

    fn extract_release_info(&self) -> ReleaseInfo {
        match (self.capture(&NAME, 1), self.capture(&NAME, 2)) {
            (Some(artist), Some(title)) => ReleaseInfo::new(artist, title),
            _ => ReleaseInfo::default(),
        }
    }

    fn extract_drive(&self) -> String {
        match self.capture(&DRIVE, 1) {
            Some(d) => DRIVE_REVISION.replace(&d, "").trim().to_owned(),
            None => String::default(),
        }
    }

    fn extract_read_offset(&self) -> Option<i16> {
        self.capture(&READ_OFFSET, 1).and_then(|v| v.parse::<i16>().ok())
    }

    fn extract_accurate_stream(&self) -> Quartet {
        match self.capture(&STREAM_ACCURATE, 1).as_deref() {
            Some("Yes") => Quartet::True,
            Some("No") => Quartet::False,
            _ => Quartet::Unknown,
        }
    }

    /// Rip reports each track as verified or not, which is the outcome of
    /// comparing reads rather than a setting it states up front.
    fn extract_test_and_copy(&self) -> Quartet {
        match COPY_VERIFIED.is_match(&self.log) {
            true => Quartet::True,
            false => Quartet::Unknown,
        }
    }

    fn extract_toc(&self) -> Toc {
        let entries: Vec<TocEntry> = TOC_ROW
            .captures_iter(&self.log)
            .filter_map(|c| {
                Some(TocEntry::new(
                    c["track"].parse::<u32>().ok()?,
                    msf_to_time(&c["start"]),
                    msf_to_time(&c["dur"]),
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

impl IntegrityChecker for RipParserSingle {}

impl RipParserTrack {
    fn field(&self, re: &Regex, group: usize) -> Option<String> {
        re.captures(&self.block)
            .and_then(|c| c.get(group))
            .map(|m| m.as_str().trim().to_owned())
    }
}

impl ParserTrack for RipParserTrack {}

impl TrackExtractor for RipParserTrack {
    fn extract_num(&self) -> u8 {
        self.num
    }

    fn extract_is_range(&self) -> bool {
        false
    }

    fn extract_filenames(&self) -> Vec<String> {
        match self.filename.is_empty() {
            true => Vec::new(),
            false => vec![self.filename.clone()],
        }
    }

    fn extract_peak_level(&self) -> Option<f64> {
        self.field(&T_PEAK, 1).and_then(|v| v.parse::<f64>().ok())
    }

    fn extract_gain(&self) -> Option<f64> {
        self.field(&T_GAIN, 1).and_then(|v| v.parse::<f64>().ok())
    }

    fn extract_ar_info(&self) -> Vec<AccurateRipUnit> {
        let Some(crc) = self.field(&T_AR_CRC, 1).map(|v| low32(&v)) else {
            return Vec::new();
        };

        let confidence = self
            .field(&T_CONFIDENCE, 1)
            .and_then(|v| v.parse::<u32>().ok());

        let status = match self.field(&T_ACCURATE, 1).as_deref() {
            Some("Yes") => AccurateRipStatus::Match,
            Some(_) => AccurateRipStatus::Mismatch,
            None if T_AR_FAILED.is_match(&self.block) => AccurateRipStatus::NotFound,
            None => AccurateRipStatus::NotFound,
        };

        vec![AccurateRipUnit::new(
            None,
            crc.clone(),
            crc,
            confidence.map(|c| {
                AccurateRipConfidence::new(Some(c), None, AccurateRipOffset::Same)
            }),
            status,
        )]
    }
}
