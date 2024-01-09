use simple_text_decode::DecodedText;

use crate::{translate::TranslatorCombined, extract::{Ripper, Extractor, Quartet, ReadMode, Gap, TrackExtractor}, track::{TrackEntry, TestAndCopy}, toc::{TocEntry, Toc, TocRaw}, util::Time};

use super::{eac_parser::EacParserSingle, ParsedLog, ParsedLogCombined, ParserCombined, Parser, IntegrityChecker, ParserTrack};

use regex::{Regex, Captures};

lazy_static! {
    static ref RIPPER_VERSION: Regex = Regex::new(r"CUERipper v(.+) Copyright").unwrap();
    static ref EAC_VARIANT: Regex = Regex::new(r"EAC extraction logfile from ").unwrap();
    static ref USED_DRIVE: Regex = Regex::new(r"Used drive( *): (.+)").unwrap();

    static ref READ_MODE: Regex = Regex::new(r"Secure mode( *): (\d+)").unwrap();

    static ref READ_OFFSET_CORRECTION: Regex = Regex::new(r"Read offset correction( *): ([+-]?[0-9]+)").unwrap();

    static ref TOC: Regex = Regex::new(r"\s+(?P<track>\d+)\s+\|\s+(?P<start>[0-9:\.]+)\s+\|\s+(?P<length>[0-9:\.]+)\s+\|\s+(?P<start_sector>\d+)\s+\|\s+(?P<end_sector>\d+)").unwrap();

    static ref FILENAME: Regex = Regex::new(r"    (.+\..\w+)(\r|\n|\r\n|\n\r)").unwrap();
    static ref PREGAP: Regex = Regex::new(r"\s+(?P<track>\d+)\s+\|\s+(?P<pregap>[0-9:]+)\s+\|\s+(?P<indices>\d+)").unwrap();
    static ref PEAK_CRC: Regex = Regex::new(r"\s+(?P<track>\d{2})\s+(?P<peak>[0-9\.]+)\s+\[(?P<crc>[A-F0-9]{8})\]\s+\[(?P<crcnull>[A-F0-9]{8})\]").unwrap();
}

pub struct CueRipperParser {
    encoded_log: DecodedText,
}

pub struct CueRipperParserSingle {
    log: String,
    language: String,
}

pub struct CueRipperParserTrack<'a> {
    filename: String,
    pregap: Captures<'a>,
    peak_crc: Option<Captures<'a>>,
}

impl CueRipperParser {
    pub fn new(encoded_log: DecodedText) -> Self {
        CueRipperParser {
            encoded_log,
        }
    }
}

impl ParserCombined for CueRipperParser {
    fn parse_combined(&self) -> ParsedLogCombined {        
        let parsed_logs: Vec<ParsedLog> = vec![CueRipperParserSingle::new(self.encoded_log.text.trim().to_owned()).parse()];

        ParsedLogCombined {
            parsed_logs,
            encoding: self.encoded_log.orig_encoding.to_string()
        }
    }
}

impl TranslatorCombined for CueRipperParser {
    fn translate_combined(&self) -> String {
        self.encoded_log.text.clone()
    }
}

impl<'a> CueRipperParserTrack<'a> {
    fn new(filename: String, pregap: Captures<'a>, peak_crc: Option<Captures<'a>>) -> Self {
        Self { filename, pregap, peak_crc }
    }
}

impl CueRipperParserSingle {
    pub fn new(log: String) -> Self {
        Self {
            log,
            language: String::from("English"),
        }
    }
}

impl Parser for CueRipperParserSingle {
    fn parse(&mut self) -> ParsedLog {
        let captures = EAC_VARIANT.captures(&self.log);
        let parsed_log: ParsedLog = match captures {
            Some(_) => {
                let mut eac_variant = EacParserSingle::new(self.log.trim().to_string()).parse();
                eac_variant.ripper = self.extract_ripper();
                eac_variant.ripper_version = self.extract_ripper_version();
                eac_variant.checksum = self.get_checksum();
                eac_variant.id3_enabled = self.extract_id3_enabled();
                eac_variant
            },
            // Call the default impl
            None => Parser::parse(self),
        };
        parsed_log
    }
}

impl Extractor for CueRipperParserSingle {
    fn extract_ripper(&self) -> Ripper {
        Ripper::CueRipper
    }

    fn extract_ripper_version(&self) -> String {
        let captures = RIPPER_VERSION.captures(&self.log);
        match captures {
            Some(captures) => captures.get(1).unwrap().as_str().trim_start_matches('V').to_string(),
            None => String::from("Unknown"),
        }
    }

    fn extract_language(&self) -> String {
        self.language.clone()
    }

    fn extract_drive(&self) -> String {
        let captures = USED_DRIVE.captures(&self.log);
        match captures {
            Some(captures) => captures.get(2).unwrap().as_str().trim().to_string(),
            None => String::default(),
        }
    }

    fn extract_read_offset(&self) -> Option<i16> {
        let captures = READ_OFFSET_CORRECTION.captures(&self.log);
        captures.map(|captures| captures.get(2).unwrap().as_str().parse::<i16>().unwrap())
    }

    fn extract_read_mode(&self) -> ReadMode {
        let captures = READ_MODE.captures(&self.log);
        match captures {
            Some(captures) => {
                let value = captures.get(2).unwrap().as_str().parse::<i8>().unwrap_or(-1);
                match value {
                    0 => ReadMode::Burst,
                    1 => ReadMode::Secure,
                    2 => ReadMode::Paranoid,
                    _ => ReadMode::Unknown,
                }
            },
            None => ReadMode::Unknown,
        }
    }

    // Could be AppendUndetected as well, no way to know
    fn extract_gap_handling(&self) -> Gap {
        Gap::Append
    }

    fn extract_id3_enabled(&self) -> Quartet {
        Quartet::False
    }

    fn extract_toc(&self) -> Toc {
        let mut entries: Vec<TocEntry> = Vec::new();
        let captures_all = TOC.captures_iter(&self.log);

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

        let mut filename_all = FILENAME.captures_iter(&self.log);
        let mut peak_crc_all = PEAK_CRC.captures_iter(&self.log);
        let pregap_all = PREGAP.captures_iter(&self.log);

        for pregap in pregap_all {
            let track_parser = CueRipperParserTrack::new(
                match filename_all.next() {
                    Some(f) => f.get(1).unwrap().as_str().trim_start().to_owned(),
                    None => String::new(),
                },
                pregap,
                peak_crc_all.next()
            );
            tracks.push(track_parser.parse_track());
        }

        tracks
    }
}

impl IntegrityChecker for CueRipperParserSingle {
    fn extract_checksum(&self) -> String {
        String::new()
    }

    fn calculate_checksum(&self) -> String {
        String::new()
    }
}

impl<'a> ParserTrack for CueRipperParserTrack<'a> {}

impl<'a> TrackExtractor for CueRipperParserTrack<'a> {
    fn extract_num(&self) -> u8 {
        match &self.peak_crc {
            Some(c) => c.name("track").unwrap().as_str().parse::<u8>().unwrap_or_default(),
            None => u8::default(),
        }
    }

    fn extract_is_range(&self) -> bool {
        false
    }

    fn extract_filenames(&self) -> Vec<String> {
        if self.filename.is_empty() { Vec::new() } else { vec![self.filename.clone()] }
    }

    fn extract_peak_level(&self) -> Option<f64> {
        match &self.peak_crc {
            Some(c) => c.name("peak").map(|v| v.as_str().parse::<f64>().unwrap() / 100.0),
            None => None,
        }
    }

    fn extract_pregap_length(&self) -> Option<Time> {
        self.pregap.name("pregap").map(|v| Time::from_mm_ss_cs(v.as_str()))
    }

    fn extract_test_and_copy(&self) -> TestAndCopy {
        let crc = match &self.peak_crc {
            Some(c) => c.name("crc").unwrap().as_str().to_owned(),
            None => String::new(),
        };
        let crc_nonull = match &self.peak_crc {
            Some(c) => c.name("crcnull").unwrap().as_str().to_owned(),
            None => String::new(),
        };

        TestAndCopy::new(crc.clone(), crc, crc_nonull.clone(), crc_nonull)
    }
}