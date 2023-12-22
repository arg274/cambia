pub use crate::extract::{self, Quartet, Ripper, ReadMode, Gap, Extractor, TrackExtractor};
use crate::toc::Toc;
use crate::track::TrackEntry;
pub use crate::translate::Translator;
pub use crate::integrity::{Checksum, IntegrityChecker};

pub mod eac_parser;
pub mod xld_parser;
pub mod whipper_parser;

use serde::{Serialize, Deserialize};
use ts_rs::TS;

use self::extract::MediaType;

// TODO: I need to revisit this and make some fields optional
// Current thought is to make an enum that holds different RipperTypeOptionalData fields in each variant
#[derive(Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ParsedLog {
    pub ripper: Ripper,
    pub ripper_version: String,
    pub language: String,
    pub read_offset: Option<i16>,
    pub combined_rw_offset: Option<i32>,
    pub drive: String,
    pub media_type: MediaType,
    pub accurate_stream: Quartet,
    pub defeat_audio_cache: Quartet,
    pub use_c2: Quartet,
    pub overread: Quartet,
    pub fill_silence: Quartet,
    pub delete_silence: Quartet,
    pub use_null_samples: Quartet,
    pub test_and_copy: Quartet,
    pub normalize: Quartet,
    pub read_mode: ReadMode,
    pub gap_handling: Gap,
    pub checksum: Checksum,
    pub toc: Toc,
    pub tracks: Vec<TrackEntry>,
    pub id3_enabled: Quartet,
    pub audio_encoder: Vec<String>,
}

#[derive(Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ParsedLogCombined {
    pub parsed_logs: Vec<ParsedLog>,
    pub encoding: String,
}

pub trait Parser: Extractor + IntegrityChecker {
    fn parse(&mut self) -> ParsedLog;
}

pub trait ParserSingle: Translator {}

pub trait ParserCombined {
    fn parse_combined(&self) -> ParsedLogCombined;
}

pub trait ParserTrack: TrackExtractor {
    fn parse_track(&self) -> TrackEntry;
}