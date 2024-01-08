mod whipper_yaml;

use regex::Regex;
use sha2::{Sha256, Digest};

use crate::{extract::{Extractor, Quartet, Ripper, ReadMode, Gap, TrackExtractor, MediaType}, translate::{Translator, TranslatorCombined}, integrity::IntegrityChecker, toc::{TocEntry, TocRaw, Toc}, util::Time, track::{TrackEntry, TestAndCopy}};
use simple_text_decode::DecodedText;

use self::whipper_yaml::{WhipperLogYaml, WhipperTrackEntry};

use super::{Parser, ParsedLog, ParserCombined, ParsedLogCombined, ParserTrack};

lazy_static! {
    static ref RIPPER_VERSION: Regex = Regex::new(r"whipper ([a-zA-Z0-9.+]+) .*").unwrap();
    static ref CHECKSUM: Regex = Regex::new(r"\nSHA-256 hash: [a-zA-Z0-9]{64}").unwrap();
    
    static ref SANITISE_RELEASE: Regex = Regex::new(r"(Release|Album): (.+)").unwrap();
}

pub struct WhipperParser {
    encoded_log: DecodedText,
}

struct WhipperParserSingle {
    log: String,
    language: String,
    yaml: WhipperLogYaml,
}

struct WhipperParserTrack<'a> {
    num: u8,
    yaml: &'a WhipperTrackEntry,
}

impl WhipperParser {
    pub fn new(encoded_log: DecodedText) -> WhipperParser {
        WhipperParser {
            encoded_log,
        }
    }
}

impl WhipperParserSingle {
    pub fn new(log: String) -> WhipperParserSingle {
        let (language, _) = WhipperParserSingle::translate(log.clone());

        let yaml_sanitised = SANITISE_RELEASE.replace(&log, "${1}: \"{2}\"");
        let yaml: WhipperLogYaml = serde_yaml::from_str(&yaml_sanitised).unwrap_or_default();
        
        WhipperParserSingle {
            log,
            language,
            yaml,
        }
    }

    fn boolean_matcher(value: &Option<String>) -> Quartet {
        if value.is_none() {
            return Quartet::Unknown;
        }

        match value.as_ref().unwrap().as_str() {
            "true" | "Yes" => Quartet::True,
            "false" | "No" => Quartet::False,
            _ => Quartet::Unknown,
        }
    }
}

impl<'a> WhipperParserTrack<'a> {
    fn new(num: u8, yaml: &'a WhipperTrackEntry) -> Self {
        Self { num, yaml }
    }
}

impl ParserCombined for WhipperParser {
    fn parse_combined(&self) -> ParsedLogCombined {
        let parsed_logs: Vec<ParsedLog> = vec![WhipperParserSingle::new(self.encoded_log.text.trim().to_string()).parse()];

        ParsedLogCombined {
            parsed_logs,
            encoding: self.encoded_log.orig_encoding.to_string()
        }
    }
}

impl TranslatorCombined for WhipperParser {
    fn translate_combined(&self) -> String {
        self.encoded_log.text.clone()
    }
}

impl Parser for WhipperParserSingle {}

impl Extractor for WhipperParserSingle {
    fn extract_ripper(&self) -> Ripper {
        Ripper::Whipper
    }
    
    fn extract_ripper_version(&self) -> String {
        let captures = RIPPER_VERSION.captures(&self.yaml.version);
        match captures {
            Some(captures) => captures.get(1).unwrap().as_str().to_string(),
            None => String::from("Unknown"),
        }
    }

    fn extract_read_offset(&self) -> Option<i16> {
        Some(self.yaml.ripping_phase_info.read_offset)
    }

    fn extract_language(&self) -> String {
        self.language.clone()
    }

    fn extract_drive(&self) -> String {
        self.yaml.ripping_phase_info.drive.clone()
    }

    fn extract_accurate_stream(&self) -> Quartet {
        Quartet::True
    }

    fn extract_defeat_audio_cache(&self) -> Quartet {
        Self::boolean_matcher(&self.yaml.ripping_phase_info.cache)
    }

    fn extract_overread(&self) -> Quartet {
        Self::boolean_matcher(&self.yaml.ripping_phase_info.overread)
    }

    fn extract_use_null_samples(&self) -> Quartet {
        Quartet::True
    }

    fn extract_test_and_copy(&self) -> Quartet {
        Quartet::True
    }

    fn extract_read_mode(&self) -> ReadMode {
        ReadMode::Secure
    }

    fn extract_gap_handling(&self) -> Gap {
        Gap::Append
    }

    fn extract_media_type(&self) -> MediaType {
        match Self::boolean_matcher(&self.yaml.ripping_phase_info.cdr) {
            Quartet::True => MediaType::CDR,
            Quartet::False => MediaType::Pressed,
            _ => MediaType::Unknown,
        }
    }

    fn extract_toc(&self) -> Toc {
        let mut entries: Vec<TocEntry> = Vec::new();

        for (k, v) in &self.yaml.toc {
            entries.push(TocEntry::new(
                *k,
                Time::from_mm_ss(&v.start),
                Time::from_mm_ss(&v.length),
                v.start_sector,
                v.end_sector,
            ));
        }

        entries.sort_by_key(|e| e.track);

        Toc::new(TocRaw::new(entries))
    }

    fn extract_tracks(&self) -> Vec<TrackEntry> {
        let mut tracks: Vec<TrackEntry> = Vec::new();
        
        for (num, track) in &self.yaml.tracks {
            tracks.push(WhipperParserTrack::new(num.to_owned().try_into().unwrap_or_default(), track).parse_track());
        }

        tracks
    }
}

impl Translator for WhipperParserSingle {
    fn translate(log: String) -> (String, String) {
        (String::from("English"), log)
    }
}

impl IntegrityChecker for WhipperParserSingle {
    fn extract_checksum(&self) -> String {
        if self.yaml.checksum == "INVALID" {
            return String::default();
        }
        self.yaml.checksum.clone().to_uppercase()
    }

    fn calculate_checksum(&self) -> String {
        // This DOES NOT consider CRLF
        let checksum_stripped = CHECKSUM.replace_all(&self.log, "");
        let mut hasher = Sha256::new();
        
        hasher.update(checksum_stripped.as_bytes());
        let result = hasher.finalize();

        hex::encode_upper(result)
    }
}

impl<'a> ParserTrack for WhipperParserTrack<'a> {}

impl<'a> TrackExtractor for WhipperParserTrack<'a> {
    fn extract_num(&self) -> u8 {
        self.num
    }

    fn extract_is_range(&self) -> bool {
        false
    }

    fn extract_filename(&self) -> String {
        self.yaml.filename.clone()
    }

    fn extract_peak_level(&self) -> Option<f64> {
        Some(self.yaml.peak_level)
    }

    fn extract_pregap_length(&self) -> Option<Time> {
        self.yaml.pregap.clone().map(|p| Time::from_mm_ss_cs(p.as_str()))
    }

    fn extract_extraction_speed(&self) -> Option<f64> {
        self.yaml.extraction_speed.trim_end_matches(" X").parse::<f64>().ok()
    }

    fn extract_preemphasis(&self) -> Option<bool> {
        match WhipperParserSingle::boolean_matcher(&self.yaml.preemphasis) {
            Quartet::True => Some(true),
            Quartet::False => Some(false),
            _ => None,
        }
    }

    fn extract_test_and_copy(&self) -> TestAndCopy {
        TestAndCopy::new_no_skipzero(self.yaml.test_crc.clone(), self.yaml.copy_crc.clone())
    }
}