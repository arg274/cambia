mod whipper_yaml;

use regex::Regex;
use sha2::{Sha256, Digest};

use crate::{extract::{Extractor, Quartet, Ripper, ReadMode, Gap}, translate::Translator, integrity::IntegrityChecker, toc::{TocEntry, TocRaw, Toc}, util::Time};
use simple_text_decode::DecodedText;

use self::whipper_yaml::WhipperLogYaml;

use super::{Parser, ParsedLog, ParserCombined, ParsedLogCombined};

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
        // println!("{}", &log);

        let yaml_sanitised = SANITISE_RELEASE.replace(&log, "${1}: \"{2}\"");
        // FIXME: Would panic on invalid YAML
        let yaml: WhipperLogYaml = serde_yaml::from_str(&yaml_sanitised).unwrap();
        
        WhipperParserSingle {
            log,
            language,
            yaml,
        }
    }

    fn boolean_matcher(&self, value: &str) -> Quartet {
        match value.to_uppercase().as_str() {
            "TRUE" => Quartet::True,
            "YES" => Quartet::True,
            "FALSE" => Quartet::False,
            "NO" => Quartet::False,
            _ => Quartet::Unknown,
        }
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

impl Parser for WhipperParserSingle {
    fn parse(&mut self) -> ParsedLog {
        ParsedLog {
            ripper: self.extract_ripper(),
            ripper_version: self.extract_ripper_version(),
            language: self.extract_language(),
            read_offset: self.extract_read_offset(),
            combined_rw_offset: self.extract_combined_rw_offset(),
            drive: self.extract_drive(),
            media_type: self.extract_media_type(),
            accurate_stream: self.extract_accurate_stream(),
            defeat_audio_cache: self.extract_defeat_audio_cache(),
            use_c2: self.extract_use_c2(),
            overread: self.extract_overread(),
            fill_silence: self.extract_fill_silence(),
            delete_silence: self.extract_delete_silence(),
            use_null_samples: self.extract_use_null_samples(),
            test_and_copy: self.extract_test_and_copy(),
            normalize: self.extract_normalize(),
            read_mode: self.extract_read_mode(),
            gap_handling: self.extract_gap_handling(),
            checksum: self.get_checksum(),
            toc: self.extract_toc(),
            tracks: self.extract_tracks(),
            id3_enabled: self.extract_id3_enabled(),
        }
    }
}

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
        self.boolean_matcher(&self.yaml.ripping_phase_info.cache)
    }

    fn extract_overread(&self) -> Quartet {
        self.boolean_matcher(&self.yaml.ripping_phase_info.overread)
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

    fn extract_toc(&self) -> Toc {
        let mut entries: Vec<TocEntry> = Vec::new();

        for (k, v) in &self.yaml.toc {
            entries.push(TocEntry::new(
                *k,
                Time::from_mm_ss(&v.start),
                Time::from_mm_ss(&v.length),
                v.start_sector,
                v.end_sector,
            ))
        }

        entries.sort_by_key(|e| e.track);

        Toc::new(TocRaw::new(entries))
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