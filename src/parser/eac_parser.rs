mod translation_table;
mod rijndael;

use std::str::FromStr;

use aho_corasick::AhoCorasickBuilder;
use phf::OrderedMap;
use regex::{Regex, RegexBuilder};
use rayon::prelude::*;

use crate::{extract::{Extractor, Quartet, Ripper, ReadMode, Gap, TrackExtractor}, translate::{Translator, TranslatorCombined}, integrity::IntegrityChecker, toc::{TocEntry, TocRaw, Toc}, track::{TrackEntry, TestAndCopy, TrackError, TrackErrorRange, TrackErrorData}, util::Time};
use simple_text_decode::DecodedText;

use self::{translation_table::{LANGS, L_DUMMY_MAP, L_47AB3DF2_MAP}, rijndael::Rijndael};

use super::{Parser, ParsedLog, ParserCombined, ParsedLogCombined, ParserTrack};

static SPLIT_SEP: &str = "\r\n------------------------------------------------------------\r\n";

lazy_static! {
    static ref RIPPER_VERSION: Regex = Regex::new(r"Exact Audio Copy (.+) from").unwrap();
    static ref USED_DRIVE: Regex = Regex::new(r"Used drive( *): (.+)").unwrap();
    static ref DRIVE_TRIM: Regex = Regex::new(r"\s*Adapter:\s*\d+\s*ID:\s*\d+").unwrap();

    static ref READ_MODE: Regex = Regex::new(r"Read mode( *): (\w+)").unwrap();
    static ref ACCURATE_STREAM: Regex = Regex::new(r"Utilize accurate stream( *): (?P<boolean>Yes|No)").unwrap();
    static ref ACCURATE_STREAM_LEGACY: Regex = Regex::new(r"Read mode( *): (\w+) with (NO )?C2, (?P<boolean>NO )?accurate stream").unwrap();
    static ref DEFEAT_AUDIO_CACHE: Regex = Regex::new(r"Defeat audio cache( *): (?P<boolean>Yes|No)").unwrap();
    static ref DEFEAT_AUDIO_CACHE_LEGACY: Regex = Regex::new(r"Read mode( *): (\w+) with (NO )?C2, (NO )?accurate stream, (?P<boolean>NO )?disable cache").unwrap();
    static ref USE_C2: Regex = Regex::new(r"Make use of C2 pointers( *): (?P<boolean>Yes|No)").unwrap();
    static ref USE_C2_LEGACY: Regex = Regex::new(r"Read mode( *): (\w+) with (?P<boolean>NO )?C2").unwrap();

    static ref READ_OFFSET_CORRECTION: Regex = Regex::new(r"Read offset correction( *): ([+-]?[0-9]+)").unwrap();
    static ref COMBINED_OFFSET_CORRECTION: Regex = Regex::new(r"Combined read/write offset correction( *): ([+-]?[0-9]+)").unwrap();
    static ref OVERREAD: Regex = Regex::new(r"Overread into Lead-In and Lead-Out( *): (?P<boolean>Yes|No)").unwrap();
    static ref FILL_SILENCE: Regex = Regex::new(r"Fill up missing offset samples with silence( *): (?P<boolean>Yes|No)").unwrap();
    static ref DELETE_SILENCE: Regex = Regex::new(r"Delete leading and trailing silent blocks( *): (?P<boolean>Yes|No)").unwrap();
    static ref USE_NULL_SAMPLES: Regex = Regex::new(r"Null samples used in CRC calculations( *): (?P<boolean>Yes|No)").unwrap();
    static ref GAP_HANDLING: Regex = Regex::new(r"Gap handling( *): (.+)").unwrap();
    static ref USED_OUTPUT_FMT: Regex = RegexBuilder::new(r"Used output format( *): (.*)(?P<fmt>flac|wav|mp3|m4a|ape|tta|ogg)").case_insensitive(true).build().unwrap();
    static ref CLI_ENCODER: Regex = Regex::new(r"Command line compressor( *): (.+)").unwrap();

    static ref TEST_AND_COPY: Regex = Regex::new(r"Test CRC ([0-9A-F]{8})").unwrap();
    static ref NORMALIZE: Regex = Regex::new(r"Normalize to( +): ([0-9% ]+)").unwrap();
    static ref ID3_ENABLED: Regex = Regex::new(r"Add ID3 tag( *): (?P<boolean>Yes|No)").unwrap();
    
    static ref CHECKSUM: Regex = Regex::new(r"==== (.+) ([0-9A-Z]{64}) ====").unwrap();
    static ref TOC: Regex = Regex::new(r"\s+(?P<track>\d+)\s+\|\s+(?P<start>[0-9:\.]+)\s+\|\s+(?P<length>[0-9:\.]+)\s+\|\s+(?P<start_sector>\d+)\s+\|\s+(?P<end_sector>\d+)").unwrap();

    static ref SPLIT_TRACKS: Regex = RegexBuilder::new(r"Track\s*\d+.+?Copy (OK|finished|aborted)").dot_matches_new_line(true).build().unwrap();
    static ref RANGE_TRACKS: Regex = RegexBuilder::new(r"Range status and errors.+?Copy (OK|finished|aborted)").dot_matches_new_line(true).build().unwrap();

    static ref TRACK_NUMBER: Regex = Regex::new(r"Track\s*(?P<value>\d+)").unwrap();
    static ref COPY_ABORTED: Regex = RegexBuilder::new(r"Copy aborted").multi_line(true).build().unwrap();
    static ref FILENAME: Regex = RegexBuilder::new(r"Filename (?P<value>.+)$").multi_line(true).build().unwrap();
    static ref PREGAP: Regex = Regex::new(r"Pre-gap length(\s*)(?P<time>\d:\d{2}:\d{2}\.\d{2})").unwrap();
    static ref PEAK_LEVEL: Regex = Regex::new(r"Peak level (?P<value>.+)%").unwrap();
    static ref EXTRACTION_SPEED: Regex = Regex::new(r"Extraction speed (?P<value>.+)X").unwrap();
    static ref TRACK_QUALITY: Regex = Regex::new(r"Track quality (?P<value>.+)%").unwrap();
    static ref TEST_CRC: Regex = RegexBuilder::new(r"Test CRC (?P<value>.+)$").multi_line(true).build().unwrap();
    static ref COPY_CRC: Regex = RegexBuilder::new(r"Copy CRC (?P<value>.+)$").multi_line(true).build().unwrap();
    static ref ERROR: Regex = Regex::new(r"(?P<type>Suspicious position|Timing problem)(\s*)(?P<start>\d:\d{2}:\d{2})( - (?P<end>\d:\d{2}:\d{2}))?").unwrap();
}

pub struct EacParser {
    encoded_log: DecodedText,
}

pub struct EacParserSingle {
    log: String,
    translated_log: String,
    language: String,
}

impl EacParser {
    pub fn new(encoded_log: DecodedText) -> EacParser {
        EacParser {
            encoded_log,
        }
    }

    pub fn split_combined(&self) -> Vec<&str> {
        /* TODO: When the log anomaly pipeline is implemented
           Make sure that combined logs that don't use this sep are detected and handled */
        // TODO: Might need to use str::split_inclusive in the future
        self.encoded_log.text.split(SPLIT_SEP).collect::<Vec<_>>()
    }
}

pub struct EacParserTrack {
    is_range: bool,
    use_null_samples: Quartet,
    raw: String,
}

impl EacParserSingle {
    pub fn new(log: String) -> EacParserSingle {
        let (language, translated_log) = EacParserSingle::translate(log.clone());
        // println!("{}", &log);
        // println!("{}", &translated_log);
        EacParserSingle {
            log,
            translated_log,
            language,
        }
    }

    fn boolean_matcher(&self, regex: &Regex) -> Quartet {
        let captures = regex.captures(&self.translated_log);
        match captures {
            Some(captures) => {
                let value = captures.name("boolean").unwrap().as_str();
                match value {
                    "Yes" => Quartet::True,
                    "No" => Quartet::False,
                    _ => Quartet::Unknown,
                }
            },
            None => Quartet::Unknown,
        }
    }

    fn boolean_matcher_legacy(&self, regex: &Regex) -> Quartet {
        let captures = regex.captures(&self.translated_log);
        match captures {
            Some(captures) => {
                let value = captures.name("boolean");
                match value {
                    Some(_) => Quartet::False,
                    None => Quartet::True,
                }
            },
            None => Quartet::Unknown,
        }
    }
}

impl ParserCombined for EacParser {
    fn parse_combined(&self) -> ParsedLogCombined {
        let split_logs = self.split_combined();

        let parsed_logs: Vec<ParsedLog> = split_logs.par_iter().map(
            |split_log| EacParserSingle::new(split_log.trim().to_string()).parse()
        ).collect();
        
        ParsedLogCombined {
            parsed_logs,
            encoding: self.encoded_log.orig_encoding.to_string()
        }
    }
}

impl TranslatorCombined for EacParser {
    fn translate_combined(&self) -> String {
        let split_logs = self.split_combined();

        let translated_logs: Vec<String> = split_logs.par_iter().map(
            |split_log| EacParserSingle::translate(split_log.trim().to_owned()).1
        ).collect();
        
        translated_logs.join(SPLIT_SEP)
    }
}

impl Parser for EacParserSingle {}

impl Extractor for EacParserSingle {
    fn extract_ripper(&self) -> Ripper {
        Ripper::EAC
    }

    fn extract_ripper_version(&self) -> String {
        let captures = RIPPER_VERSION.captures(&self.translated_log);
        match captures {
            Some(captures) => captures.get(1).unwrap().as_str().trim_start_matches('V').to_string(),
            None => String::from("Unknown"),
        }
    }

    fn extract_read_offset(&self) -> Option<i16> {
        let captures = READ_OFFSET_CORRECTION.captures(&self.translated_log);
        captures.map(|captures| captures.get(2).unwrap().as_str().parse::<i16>().unwrap())
    }

    fn extract_combined_rw_offset(&self) -> Option<i32> {
        let captures = COMBINED_OFFSET_CORRECTION.captures(&self.translated_log);
        captures.map(|captures| captures.get(2).unwrap().as_str().parse::<i32>().unwrap())
    }

    fn extract_language(&self) -> String {
        self.language.clone()
    }
    
    fn extract_drive(&self) -> String {
        let captures = USED_DRIVE.captures(&self.translated_log);
        match captures {
            Some(captures) => {
                let untrimmed = captures.get(2).unwrap().as_str().trim().to_string();
                DRIVE_TRIM.replace_all(&untrimmed, "").to_string()
            },
            None => String::default(),
        }
    }

    fn extract_accurate_stream(&self) -> Quartet {
        match self.boolean_matcher(&ACCURATE_STREAM) {
            Quartet::Unknown => self.boolean_matcher_legacy(&ACCURATE_STREAM_LEGACY),
            other => other
        }
    }

    fn extract_defeat_audio_cache(&self) -> Quartet {
        match self.boolean_matcher(&DEFEAT_AUDIO_CACHE) {
            Quartet::Unknown => self.boolean_matcher_legacy(&DEFEAT_AUDIO_CACHE_LEGACY),
            other => other
        }
    }

    fn extract_use_c2(&self) -> Quartet {
        match self.boolean_matcher(&USE_C2) {
            Quartet::Unknown => self.boolean_matcher_legacy(&USE_C2_LEGACY),
            other => other
        }
    }

    fn extract_overread(&self) -> Quartet {
        self.boolean_matcher(&OVERREAD)
    }

    fn extract_fill_silence(&self) -> Quartet {
        self.boolean_matcher(&FILL_SILENCE)
    }

    fn extract_delete_silence(&self) -> Quartet {
        self.boolean_matcher(&DELETE_SILENCE)
    }

    fn extract_use_null_samples(&self) -> Quartet {
        self.boolean_matcher(&USE_NULL_SAMPLES)
    }

    fn extract_id3_enabled(&self) -> Quartet {
        self.boolean_matcher(&ID3_ENABLED)
    }

    fn extract_normalize(&self) -> Quartet {
        let captures = NORMALIZE.captures(&self.translated_log);
        match captures {
            Some(_) => Quartet::True,
            // FIXME: Value can be unknown based on EAC version
            None => Quartet::False,
        }
    }

    fn extract_test_and_copy(&self) -> Quartet {
        let captures = TEST_AND_COPY.captures(&self.translated_log);
        match captures {
            Some(_) => Quartet::True,
            None => Quartet::False,
        }
    }

    fn extract_read_mode(&self) -> ReadMode {
        let captures = READ_MODE.captures(&self.translated_log);
        match captures {
            Some(captures) => {
                let value = captures.get(2).unwrap().as_str();
                match value {
                    "Secure" => ReadMode::Secure,
                    "Paranoid" => ReadMode::Paranoid,
                    "Fast" => ReadMode::Fast,
                    "Burst" => ReadMode::Burst,
                    _ => ReadMode::Unknown,
                }
            },
            None => ReadMode::Unknown,
        }
    }

    fn extract_gap_handling(&self) -> Gap {
        let captures = GAP_HANDLING.captures(&self.translated_log);
        match captures {
            Some(captures) => {
                let value = captures.get(2).unwrap().as_str().trim().to_ascii_lowercase();
                match value {
                    unknown if unknown.contains("not detected, thus appended to previous track") => Gap::AppendUndetected,
                    prepend if prepend.contains("appended to next track") => Gap::Prepend,
                    discard if discard.contains("left out") => Gap::Discard,
                    append if append.contains("appended to previous track") => Gap::Append,
                    _ => Gap::Unknown,
                }
            },
            None => Gap::Unknown,
        }
    }

    fn extract_audio_encoder(&self) -> Vec<String> {
        let captures = USED_OUTPUT_FMT.captures(&self.translated_log);
        if let Some(m) = captures {
            return vec![m.name("fmt").unwrap().as_str().trim().to_ascii_lowercase().to_owned()];
        }

        let captures_fallback = CLI_ENCODER.captures(&self.translated_log);
        match captures_fallback {
            Some(captures) => {
                let value = captures.get(2).unwrap().as_str().trim().to_ascii_lowercase();
                let executable_path = std::path::Path::new(value.as_str());
                let executable_name = executable_path
                                                .file_stem()
                                                .unwrap_or(executable_path.as_os_str())
                                                .to_str()
                                                .unwrap()
                                                .trim_end_matches(".exe")
                                                .to_owned();
                vec![executable_name]
            },
            None => Vec::new(),
        }
    }

    fn extract_toc(&self) -> Toc {
        let mut entries: Vec<TocEntry> = Vec::new();
        let captures_all = TOC.captures_iter(&self.translated_log);

        for captures in captures_all  {
            entries.push(TocEntry::new(
                str::parse(&captures["track"]).unwrap(),
                Time::from_mm_ss(&captures["start"]),
                Time::from_mm_ss(&captures["length"]),
                str::parse(&captures["start_sector"]).unwrap(),
                str::parse(&captures["end_sector"]).unwrap(),
            ))
        }

        Toc::new(TocRaw::new(entries))
    }

    fn extract_tracks(&self) -> Vec<TrackEntry> {
        let mut tracks: Vec<TrackEntry> = Vec::new();

        let null_flag = self.extract_use_null_samples();

        let captures_all_split = SPLIT_TRACKS.captures_iter(&self.translated_log);
        let captures_all_range = RANGE_TRACKS.captures_iter(&self.translated_log);

        for captures in captures_all_split {
            let track_parser = EacParserTrack::new(false, null_flag, captures.get(0).unwrap().as_str().to_string());
            tracks.push(track_parser.parse_track())
        }

        for captures in captures_all_range {
            let track_parser = EacParserTrack::new(true, null_flag, captures.get(0).unwrap().as_str().to_string());
            tracks.push(track_parser.parse_track())
        }

        tracks
    }
}

impl ParserTrack for EacParserTrack {}

impl Translator for EacParserSingle {
    fn translate(log: String) -> (String, String) {
        let mut log_lang = &EacLanguage::default();
        for cur_lang in LANGS.iter() {
            if log.contains(cur_lang.localised_key) {
                log_lang = cur_lang;
                break;
            }
        }
        
        match log_lang.lang_id {
            "47AB3DF2" => (log_lang.lang_native.to_owned(), log),
            _ => {
                let patterns = log_lang.table.keys();
                let ac = AhoCorasickBuilder::new()
                                                        .match_kind(aho_corasick::MatchKind::LeftmostLongest)
                                                        .build(patterns);
                let mut translated_log = String::new();
                ac.replace_all_with(&log, &mut translated_log, |_, k, v| {
                    // Case-insensitive on k > 16 but not sure if it's really needed
                    let string_id = log_lang.table.get(k).unwrap();
                    if let Some(en_val) = &L_47AB3DF2_MAP.get(string_id) {
                        v.push_str(en_val);
                    }
                    true
                });
                (log_lang.lang_native.to_owned(), translated_log)
            }
        }
    }
}

impl IntegrityChecker for EacParserSingle {
    fn extract_checksum(&self) -> String {
        let captures = CHECKSUM.captures(&self.translated_log);
        match captures {
            Some(captures) => captures.get(2).unwrap().as_str().trim().to_string(),
            None => String::new(),
        }
    }

    fn calculate_checksum(&self) -> String {
        let checksum_stripped = CHECKSUM.replace_all(&self.log, "");
        let utf16data: Vec<u16> = checksum_stripped
                                        .replace(['\r', '\n'], "")
                                        .encode_utf16()
                                        .collect();
        let mut utf16bytes = unsafe { utf16data.align_to::<u8>().1.to_vec() };
        utf16bytes.resize((utf16bytes.len() + 32 - 1) / 32 * 32, 0);
        
        let rijndael = Rijndael::new(
            256,
            &hex::decode("9378716cf13e4265ae55338e940b376184da389e50647726b35f6f341ee3efd9").unwrap()
        ).unwrap();
        
        let mut checksum_calc = vec![0; 32];
        
        for i in (0..utf16bytes.len()).step_by(32) {
            for j in 0..32 {
                checksum_calc[j] ^= utf16bytes[i + j];
            }
            checksum_calc = rijndael.encrypt(&checksum_calc);
        }
        hex::encode(checksum_calc).to_ascii_uppercase()
    }
}

impl EacParserTrack {
    fn new(is_range: bool, use_null_samples: Quartet, raw: String) -> Self {
        EacParserTrack { is_range, use_null_samples, raw }
    }

    fn string_match(&self, regex: &Regex) -> String {
        match regex.captures(&self.raw) {
            Some(val) => val.name("value").unwrap().as_str().trim().to_string(),
            None => String::default(),
        }
    }

    fn optional_match<T: FromStr>(&self, regex: &Regex) -> Option<T> 
    where
        T: FromStr,
        <T as FromStr>::Err: std::fmt::Debug, {
        regex.captures(&self.raw).map(|val| val.name("value").unwrap().as_str().trim().parse::<T>().unwrap())
    }
}

impl TrackExtractor for EacParserTrack {
    fn extract_num(&self) -> u8 {
        if self.is_range { 0 } else { self.string_match(&TRACK_NUMBER).parse::<u8>().unwrap_or_default() }
    }

    fn extract_is_range(&self) -> bool {
        self.is_range
    }

    fn extract_is_aborted(&self) -> bool {
        let captures = COPY_ABORTED.captures(&self.raw);
        captures.is_some()
    }

    fn extract_filename(&self) -> String {
        self.string_match(&FILENAME)
    }

    fn extract_peak_level(&self) -> Option<f64> {
        self.optional_match::<f64>(&PEAK_LEVEL).map(|val| val / 100.0)
    }

    fn extract_pregap_length(&self) -> Option<Time> {
        let captures = PREGAP.captures(&self.raw);
        captures.map(|captures| Time::from_h_mm_ss(captures.name("time").unwrap().as_str()))
    }

    fn extract_extraction_speed(&self) -> Option<f64> {
        self.optional_match(&EXTRACTION_SPEED)
    }

    fn extract_test_and_copy(&self) -> TestAndCopy {
        let test_crc = self.string_match(&TEST_CRC);
        let copy_crc = self.string_match(&COPY_CRC);

        match self.use_null_samples {
            Quartet::True => TestAndCopy::new_no_skipzero(test_crc, copy_crc),
            Quartet::False => TestAndCopy::new_skipzero(test_crc, copy_crc),
            _ => TestAndCopy::new_integrity_no_data(test_crc, copy_crc)
        }
    }

    fn extract_errors(&self) -> TrackError {
        let captures_all = ERROR.captures_iter(&self.raw);
        let mut read_errors: Vec<TrackErrorRange> = Vec::new();
        let mut jitter_errors: Vec<TrackErrorRange> = Vec::new();

        for captures in captures_all {
            let error_type = captures.name("type").unwrap().as_str();
            let start = Time::from_h_mm_ss(captures.name("start").unwrap().as_str());
            let error_range: TrackErrorRange;

            if let Some(end) = captures.name("end") {
                error_range = TrackErrorRange::new_from_end(start, Time::from_h_mm_ss(end.as_str()));
            } else {
                error_range = TrackErrorRange::new(start, Time::from_ss("0"))
            }

            match error_type {
                "Suspicious position" => read_errors.push(error_range),
                "Timing problem" => jitter_errors.push(error_range),
                _ => {}
            }
        }

        TrackError::new_eac(
            TrackErrorData::new(read_errors.len() as u32, read_errors),
            TrackErrorData::new(jitter_errors.len() as u32, jitter_errors)
        )
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct EacLanguage {
    localised_key: &'static str,
    lang_id: &'static str,
    lang_native: &'static str,
    lang_roman: &'static str,
    table: &'static OrderedMap<&'static str, &'static str>,
}

impl EacLanguage {
    const fn new(localised_key: &'static str, lang_id: &'static str, lang_native: &'static str, lang_roman: &'static str, table: &'static OrderedMap<&'static str, &'static str>) -> EacLanguage {
        EacLanguage {
            localised_key,
            lang_id,
            lang_native,
            lang_roman,
            table,
        }
    }
}

impl Default for EacLanguage {
    fn default() -> Self {
        Self { localised_key: Default::default(), lang_id: Default::default(), lang_native: Default::default(), lang_roman: Default::default(), table: &L_DUMMY_MAP }
    }
}