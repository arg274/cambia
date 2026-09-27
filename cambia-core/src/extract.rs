use serde::{Serialize, Deserialize};
use ts_rs::TS;

use crate::{toc::Toc, track::{AccurateRipUnit, TestAndCopy, TrackEntry, TrackError}, util::Time};

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, TS)]
#[ts(export)]
pub enum Ripper {
    #[serde(rename = "Exact Audio Copy")]
    EAC,
    #[serde(rename = "X Lossless Decoder")]
    XLD,
    Whipper,
    #[serde(rename = "CUERipper")]
    CueRipper,
    #[serde(rename = "dBpoweramp")]
    DBPA,
    #[serde(rename = "cyanrip")]
    CyanRip,
    #[serde(rename = "EZ CD Audio Converter")]
    EZCD,
    #[serde(rename = "morituri")]
    Morituri,
    #[serde(rename = "Rip")]
    Rip,
    #[serde(rename = "fre:ac")]
    FreAc,
    Other,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, TS)]
#[ts(export)]
pub enum MediaType {
    Pressed,
    #[serde(rename = "CD-R")]
    CDR,
    Other,
    Unknown,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, TS)]
#[ts(export)]
pub enum Quartet {
    True,
    False,
    Unknown,
    Unsupported,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, TS)]
#[ts(export)]
pub enum ReadMode {
    Secure,
    Paranoid,
    Fast,
    Burst,
    Unknown,
}

#[derive(Serialize, Deserialize, PartialEq, TS, Debug, Clone)]
#[ts(export)]
pub struct ReleaseInfo {
    pub artist: String,
    pub title: String,
}

impl ReleaseInfo {
    pub fn new(artist: String, title: String) -> Self {
        Self {
            artist: if artist.is_empty() { "Unknown Artist".to_owned() } else { artist },
            title: if title.is_empty() { "Unknown Album".to_owned() } else { title }
        }
    }
}

impl Default for ReleaseInfo {
    fn default() -> Self {
        ReleaseInfo::new("Unknown Artist".to_owned(), "Unknown Album".to_owned())
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, TS)]
#[ts(export)]
pub enum Gap {
    Append,
    #[serde(rename = "Append (except HTOA)")]
    AppendNoHtoa,
    #[serde(rename = "Append (gaps undetected)")]
    AppendUndetected,
    Prepend,
    Discard,
    Unknown,
    Inapplicable,
}

pub trait Extractor {
    fn extract_ripper(&self) -> Ripper {
        Ripper::Other
    }

    fn extract_ripper_version(&self) -> String {
        String::from("Unknown")
    }

    fn extract_release_info(&self) -> ReleaseInfo {
        ReleaseInfo::default()
    }

    fn extract_language(&self) -> String {
        String::from("Unknown")
    }

    fn extract_read_offset(&self) -> Option<i16> {
        None
    }

    fn extract_combined_rw_offset(&self) -> Option<i32> {
        None
    }
    
    fn extract_drive(&self) -> String {
        String::from("Unknown Drive")
    }

    fn extract_media_type(&self) -> MediaType {
        MediaType::Unknown
    }

    fn extract_accurate_stream(&self) -> Quartet {
        Quartet::Unsupported
    }

    fn extract_defeat_audio_cache(&self) -> Quartet {
        Quartet::Unsupported
    }

    fn extract_use_c2(&self) -> Quartet {
        Quartet::Unsupported
    }

    fn extract_overread(&self) -> Quartet {
        Quartet::Unsupported
    }

    fn extract_fill_silence(&self) -> Quartet {
        Quartet::Unsupported
    }

    fn extract_delete_silence(&self) -> Quartet {
        Quartet::Unsupported
    }

    fn extract_use_null_samples(&self) -> Quartet {
        Quartet::Unsupported
    }

    fn extract_test_and_copy(&self) -> Quartet {
        Quartet::Unsupported
    }

    fn extract_normalize(&self) -> Quartet {
        Quartet::Unsupported
    }

    fn extract_read_mode(&self) -> ReadMode {
        ReadMode::Unknown
    }

    fn extract_gap_handling(&self) -> Gap {
        Gap::Inapplicable
    }

    fn extract_toc(&self) -> Toc {
        Toc::default()
    }

    fn extract_tracks(&self) -> Vec<TrackEntry> {
        Vec::new()
    }

    fn extract_id3_enabled(&self) -> Quartet {
        Quartet::Unsupported
    }

    fn extract_audio_encoder(&self) -> Vec<String> {
        Vec::new()
    }
}

pub trait TrackExtractor {
    fn extract_num(&self) -> u8;

    fn extract_is_range(&self) -> bool;

    fn extract_is_aborted(&self) -> bool {
        false
    }

    fn extract_filenames(&self) -> Vec<String> {
        Vec::new()
    }

    fn extract_peak_level(&self) -> Option<f64> {
        None
    }

    fn extract_pregap_length(&self) -> Option<Time> {
        None
    }

    fn extract_extraction_speed(&self) -> Option<f64> {
        None
    }

    fn extract_gain(&self) -> Option<f64> {
        None
    }

    fn extract_preemphasis(&self) -> Option<bool> {
        None
    }

    fn extract_test_and_copy(&self) -> TestAndCopy {
        TestAndCopy::default()
    }

    fn extract_errors(&self) -> TrackError {
        TrackError::default()
    }

    fn extract_ar_info(&self) -> Vec<AccurateRipUnit> {
        Vec::new()
    }
}