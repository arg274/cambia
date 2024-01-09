pub use crate::extract::{self, Quartet, Ripper, ReadMode, Gap, Extractor, TrackExtractor};
use crate::toc::Toc;
use crate::track::TrackEntry;
pub use crate::translate::Translator;
pub use crate::integrity::{Checksum, IntegrityChecker};
use crate::translate::TranslatorCombined;

#[cfg(feature = "eac")]
pub mod eac_parser;
#[cfg(feature = "xld")]
pub mod xld_parser;
#[cfg(feature = "whipper")]
pub mod whipper_parser;
#[cfg(feature = "cueripper")]
pub mod cueripper_parser;

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
            audio_encoder: self.extract_audio_encoder(),
        }
    }
}

pub trait ParserSingle: Translator {}

pub trait ParserCombined: TranslatorCombined {
    fn parse_combined(&self) -> ParsedLogCombined;
}

pub trait ParserTrack: TrackExtractor {
    fn parse_track(&self) -> TrackEntry {
        TrackEntry {
            num: self.extract_num(),
            is_range: self.extract_is_range(),
            aborted: self.extract_is_aborted(),
            filenames: self.extract_filenames(),
            peak_level: self.extract_peak_level(),
            pregap_length: self.extract_pregap_length(),
            extraction_speed: self.extract_extraction_speed(),
            gain: self.extract_gain(),
            preemphasis: self.extract_preemphasis(),
            test_and_copy: self.extract_test_and_copy(),
            errors: self.extract_errors(),
        }
    }
}