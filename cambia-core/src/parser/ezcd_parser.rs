use regex::Regex;
use simple_text_decode::DecodedText;

use crate::extract::{Extractor, Quartet, ReleaseInfo, Ripper, TrackExtractor};
use crate::toc::{Toc, TocEntry, TocRaw};
use crate::track::{TestAndCopy, TrackEntry};
use crate::translate::TranslatorCombined;
use crate::util::Time;

use super::{IntegrityChecker, ParsedLog, ParsedLogCombined, Parser, ParserCombined, ParserTrack};

lazy_static! {
    static ref RIPPER_VERSION: Regex = Regex::new(r"EZ CD Audio Converter (\S+)").unwrap();
    static ref ALBUM: Regex = Regex::new(r"(?m)^(.+?) - (.+?)\s*$").unwrap();
    // Device: [ F: ] ASUS DRW-24D5MT 2.00  -- the trailing firmware is not
    // part of the AccurateRip name, and neither is the mount point
    static ref DEVICE: Regex = Regex::new(r"(?m)^Device:\s*(?:\[[^\]]*\])?\s*(.+?)\s*$").unwrap();
    static ref FIRMWARE: Regex = Regex::new(r"\s+[0-9]+\.[0-9]+$").unwrap();
    static ref SETTING: Regex = Regex::new(r"(?m)^(?P<key>[^:\n]+?)\s*:+\s*(?P<value>.+?)\s*$").unwrap();

    static ref TOC_ROW: Regex = Regex::new(
        r"(?m)^\s*\[.\]\s+(?P<track>\d+)\s+(?P<start>[\d:.]+)\s+(?P<length>[\d:.]+)\s+(?P<first>\d+)\s+(?P<last>\d+)(?P<rest>.*)$"
    ).unwrap();
    static ref PREGAP: Regex = Regex::new(r"(\d+:\d+\.\d+)").unwrap();
    static ref PREEMPH: Regex = Regex::new(r"(Yes|No)\s*$").unwrap();

    static ref SUMMARY_ROW: Regex = Regex::new(
        r#"(?m)^\s*(?P<track>\d+)\s+(?P<status>\S+)\s+(?P<copy>[0-9A-Fa-f]{8})(?:\s+(?P<test>[0-9A-Fa-f]{8}))?\s+(?P<errors>\d+)\s+"(?P<file>.*)""#
    ).unwrap();
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

pub struct EzcdParser {
    encoded_log: DecodedText,
}

pub struct EzcdParserSingle {
    log: String,
}

pub struct EzcdParserTrack {
    num: u8,
    status: String,
    copy_crc: String,
    test_crc: String,
    filename: String,
    pregap: Option<Time>,
    preemphasis: Option<bool>,
}

impl EzcdParser {
    pub fn new(encoded_log: DecodedText) -> Self {
        EzcdParser { encoded_log }
    }
}

impl ParserCombined for EzcdParser {
    fn parse_combined(&self) -> ParsedLogCombined {
        let parsed_logs: Vec<ParsedLog> =
            vec![EzcdParserSingle::new(self.encoded_log.text.trim().to_owned()).parse()];

        ParsedLogCombined {
            parsed_logs,
            encoding: self.encoded_log.orig_encoding.to_string(),
        }
    }
}

impl TranslatorCombined for EzcdParser {
    fn translate_combined(&self) -> String {
        self.encoded_log.text.clone()
    }
}

impl EzcdParserSingle {
    pub fn new(log: String) -> Self {
        Self { log }
    }

    /// The settings block is a run of aligned "key : value" pairs.
    fn setting(&self, key: &str) -> Option<String> {
        SETTING
            .captures_iter(&self.log)
            .find(|c| c["key"].trim() == key)
            .map(|c| c["value"].trim().to_owned())
    }

    fn enabled(&self, key: &str) -> Quartet {
        match self.setting(key).as_deref() {
            Some("Enabled") => Quartet::True,
            Some("Disabled") => Quartet::False,
            _ => Quartet::Unknown,
        }
    }

    fn toc_rows(&self) -> Vec<(u8, Option<Time>, Option<bool>)> {
        TOC_ROW
            .captures_iter(&self.log)
            .filter_map(|c| {
                let num = c["track"].parse::<u8>().ok()?;
                let rest = &c["rest"];
                let pregap = PREGAP.captures(rest).map(|p| msf_to_time(&p[1]));
                let preemph = PREEMPH.captures(rest).map(|p| &p[1] == "Yes");
                Some((num, pregap, preemph))
            })
            .collect()
    }
}

impl Parser for EzcdParserSingle {}

impl Extractor for EzcdParserSingle {
    fn extract_ripper(&self) -> Ripper {
        Ripper::EZCD
    }

    fn extract_ripper_version(&self) -> String {
        RIPPER_VERSION
            .captures(&self.log)
            .map(|c| c[1].to_owned())
            .unwrap_or(String::from("Unknown"))
    }

    fn extract_language(&self) -> String {
        String::from("English")
    }

    /// The release sits on its own line between the creation date and the
    /// device, with no label of its own.
    fn extract_release_info(&self) -> ReleaseInfo {
        let candidate = self
            .log
            .lines()
            .skip_while(|l| !l.starts_with("Log creation date"))
            .skip(1)
            .find(|l| !l.trim().is_empty());

        match candidate.and_then(|l| ALBUM.captures(l)) {
            Some(c) => ReleaseInfo::new(c[1].trim().to_owned(), c[2].trim().to_owned()),
            None => ReleaseInfo::default(),
        }
    }

    fn extract_drive(&self) -> String {
        match DEVICE.captures(&self.log) {
            Some(c) => FIRMWARE.replace(c[1].trim(), "").trim().to_owned(),
            None => String::default(),
        }
    }

    fn extract_read_offset(&self) -> Option<i16> {
        self.setting("Sample offset").and_then(|v| v.parse::<i16>().ok())
    }

    fn extract_use_c2(&self) -> Quartet {
        self.enabled("C2 pointers")
    }

    /// "Verify audio" is the second pass whose CRC the summary compares.
    fn extract_test_and_copy(&self) -> Quartet {
        self.enabled("Verify audio")
    }

    fn extract_normalize(&self) -> Quartet {
        self.enabled("Calculate ReplayGain")
    }

    fn extract_audio_encoder(&self) -> Vec<String> {
        match self.setting("Encoder") {
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
        let toc = self.toc_rows();

        SUMMARY_ROW
            .captures_iter(&self.log)
            .filter_map(|c| {
                let num = c["track"].parse::<u8>().ok()?;
                let row = toc.iter().find(|(n, _, _)| *n == num);
                Some(
                    EzcdParserTrack {
                        num,
                        status: c["status"].to_owned(),
                        copy_crc: c["copy"].to_uppercase(),
                        test_crc: c.name("test").map(|m| m.as_str().to_uppercase()).unwrap_or_default(),
                        filename: c["file"].to_owned(),
                        pregap: row.and_then(|(_, p, _)| *p),
                        preemphasis: row.and_then(|(_, _, e)| *e),
                    }
                    .parse_track(),
                )
            })
            .collect()
    }
}

impl IntegrityChecker for EzcdParserSingle {}

impl ParserTrack for EzcdParserTrack {}

impl TrackExtractor for EzcdParserTrack {
    fn extract_num(&self) -> u8 {
        self.num
    }

    fn extract_is_range(&self) -> bool {
        false
    }

    fn extract_is_aborted(&self) -> bool {
        self.status != "Success"
    }

    fn extract_filenames(&self) -> Vec<String> {
        match self.filename.is_empty() {
            true => Vec::new(),
            false => vec![self.filename.clone()],
        }
    }

    fn extract_pregap_length(&self) -> Option<Time> {
        self.pregap
    }

    fn extract_preemphasis(&self) -> Option<bool> {
        self.preemphasis
    }

    fn extract_test_and_copy(&self) -> TestAndCopy {
        TestAndCopy::new(
            self.test_crc.clone(),
            self.copy_crc.clone(),
            String::new(),
            String::new(),
        )
    }
}
