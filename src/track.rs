use serde::{Serialize, Deserialize};
use ts_rs::TS;

use crate::{integrity::Integrity, util::Time};

#[derive(Serialize, Deserialize, TS)]
#[ts(export)]
pub struct TrackEntry {
    pub num: u8,
    pub is_range: bool,
    pub aborted: bool,
    pub filenames: Vec<String>,
    pub peak_level: Option<f64>,
    pub pregap_length: Option<Time>,
    pub extraction_speed: Option<f64>,
    pub gain: Option<f64>,
    pub preemphasis: Option<bool>,
    pub test_and_copy: TestAndCopy,
    pub errors: TrackError,
    pub ar_info: Vec<AccurateRipUnit>,
}

#[derive(Serialize, Deserialize, TS)]
#[ts(export)]
pub struct TestAndCopy {
    pub test_hash: String,
    pub copy_hash: String,
    pub test_skipzero_hash: String,
    pub copy_skipzero_hash: String,
    pub integrity: Integrity,
    pub integrity_skipzero: Integrity,
}

#[derive(Serialize, Deserialize, TS)]
#[ts(export)]
pub struct AccurateRipUnit {
    pub version: Option<u8>,
    pub sign: String,
    pub offset_sign: String,
    pub confidence: Option<AccurateRipConfidence>,
    pub status: AccurateRipStatus,
}

impl AccurateRipUnit {
    pub fn new(version: Option<u8>, sign: String, offset_sign: String, confidence: Option<AccurateRipConfidence>, status: AccurateRipStatus) -> Self {
        Self {
            version,
            sign,
            offset_sign,
            confidence,
            status,
        }
    }

    pub fn new_eac(version: u8, sign: String, matching: u32) -> Self {
        Self {
            version: Some(version),
            sign: sign.clone(),
            offset_sign: sign,
            confidence: Some(AccurateRipConfidence::new(
                Some(matching),
                None,
                AccurateRipOffset::Same
            )),
            status: AccurateRipStatus::Match,
        }
    }

    pub fn new_eac_mismatch(version: u8, sign: String, offset_sign: String, matching: u32) -> Self {
        Self {
            version: Some(version),
            sign,
            offset_sign,
            confidence: Some(AccurateRipConfidence::new(
                Some(matching),
                None,
                AccurateRipOffset::Different(None)
            )),
            status: AccurateRipStatus::Mismatch,
        }
    }

    pub fn new_eac_notfound() -> Self {
        Self {
            version: None,
            sign: String::default(),
            offset_sign: String::default(),
            confidence: None,
            status: AccurateRipStatus::NotFound,
        }
    }

    pub fn new_xld(version: Option<u8>, sign: String, offset_sign: String, confidence: Option<AccurateRipConfidence>) -> Self {
        let status = if confidence.as_ref().is_none() {
            AccurateRipStatus::NotFound
        } else {
            match confidence.as_ref().unwrap().offset {
                AccurateRipOffset::Same => AccurateRipStatus::Match,
                AccurateRipOffset::Different(_) => AccurateRipStatus::Offsetted,
            }
        };
        
        Self {
            version,
            sign,
            offset_sign,
            confidence,
            status,
        }
    }

    pub fn new_disabled() -> Self {
        Self {
            version: None,
            sign: String::default(),
            offset_sign: String::default(),
            confidence: None,
            status: AccurateRipStatus::Disabled,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, TS, Clone, Copy)]
#[ts(export)]
pub enum AccurateRipOffset {
    Same,
    Different(Option<i16>),
}

#[derive(Serialize, Deserialize, PartialEq, TS, Clone, Copy)]
#[ts(export)]
pub enum AccurateRipConfidenceTotal {
    All(u32),
    Version(u32),
}

#[derive(Serialize, Deserialize, TS)]
#[ts(export)]
pub struct AccurateRipConfidence {
    pub matching: Option<u32>,
    pub total: Option<AccurateRipConfidenceTotal>,
    pub offset: AccurateRipOffset
}

impl AccurateRipConfidence {
    pub fn new(matching: Option<u32>, total: Option<AccurateRipConfidenceTotal>, offset: AccurateRipOffset) -> Self {
        Self {
            matching,
            total,
            offset
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, TS)]
#[ts(export)]
pub enum AccurateRipStatus {
    Match,
    Mismatch,
    Offsetted,
    NotFound,
    Disabled,
}

// TODO: Append [key: string]: TrackErrorData
#[derive(Serialize, Deserialize, Default, TS)]
#[ts(export)]
pub struct TrackError {
    #[serde(skip_serializing_if = "TrackErrorData::is_default")]
    pub read: TrackErrorData,
    #[serde(skip_serializing_if = "TrackErrorData::is_default")]
    pub skip: TrackErrorData,
    #[serde(skip_serializing_if = "TrackErrorData::is_default", rename = "Jitter (maybe fixed)")]
    pub jitter_generic: TrackErrorData,
    #[serde(skip_serializing_if = "TrackErrorData::is_default", rename = "Edge jitter (maybe fixed)")]
    pub jitter_edge: TrackErrorData,
    #[serde(skip_serializing_if = "TrackErrorData::is_default", rename = "Atom jitter (maybe fixed)")]
    pub jitter_atom: TrackErrorData,
    #[serde(skip_serializing_if = "TrackErrorData::is_default")]
    pub drift: TrackErrorData,
    #[serde(skip_serializing_if = "TrackErrorData::is_default", rename = "Dropped bytes")]
    pub dropped: TrackErrorData,
    #[serde(skip_serializing_if = "TrackErrorData::is_default", rename = "Duplicated bytes")]
    pub duplicated: TrackErrorData,
    #[serde(skip_serializing_if = "TrackErrorData::is_default", rename = "Damaged sectors")]
    pub damaged_sectors: TrackErrorData,
    #[serde(skip_serializing_if = "TrackErrorData::is_default", rename = "Inconsistency in error sectors")]
    pub inconsistent_err_sectors: TrackErrorData,
    #[serde(skip_serializing_if = "TrackErrorData::is_default", rename = "Missing samples")]
    pub missing_samples: TrackErrorData,
}

#[derive(Serialize, Deserialize, Default, PartialEq, TS)]
#[ts(export)]
pub struct TrackErrorData {
    pub count: u32,
    pub ranges: Vec<TrackErrorRange>,
}

#[derive(Serialize, Deserialize, PartialEq, TS)]
#[ts(export)]
pub struct TrackErrorRange {
    pub start: Time,
    pub length: Time,
}

impl TrackError {
    pub fn new_eac(read: TrackErrorData, jitter_generic: TrackErrorData) -> Self {
        Self {
            read,
            skip: TrackErrorData::default(),
            jitter_generic,
            jitter_edge: TrackErrorData::default(),
            jitter_atom: TrackErrorData::default(),
            drift: TrackErrorData::default(),
            dropped: TrackErrorData::default(),
            duplicated: TrackErrorData::default(),
            damaged_sectors: TrackErrorData::default(),
            inconsistent_err_sectors: TrackErrorData::default(),
            missing_samples: TrackErrorData::default(),
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new_xld(r_c: u32, s_c: u32, jg_c: u32, je_c: u32, ja_c: u32, drf_c: u32, drp_c: u32, dup_c: u32, dmg_d: TrackErrorData, inc_d: TrackErrorData, m_s: bool) -> Self {
        TrackError {
            read: TrackErrorData::new_from_count(r_c),
            skip: TrackErrorData::new_from_count(s_c),
            jitter_generic: TrackErrorData::new_from_count(jg_c),
            jitter_edge: TrackErrorData::new_from_count(je_c),
            jitter_atom: TrackErrorData::new_from_count(ja_c),
            drift: TrackErrorData::new_from_count(drf_c),
            dropped: TrackErrorData::new_from_count(drp_c),
            duplicated: TrackErrorData::new_from_count(dup_c),
            damaged_sectors: dmg_d,
            inconsistent_err_sectors: inc_d,
            missing_samples: TrackErrorData::new_from_bool(m_s),
        }
    }
}

impl TrackErrorData {
    pub fn new(count: u32, ranges: Vec<TrackErrorRange>) -> Self {
        TrackErrorData { count, ranges }
    }

    pub fn new_from_count(count: u32) -> Self {
        TrackErrorData { count, ranges: Vec::new() }
    }

    pub fn new_from_bool(errored: bool) -> Self {
        TrackErrorData { count: u32::from(errored), ranges: Vec::new() }
    }

    pub fn is_default(&self) -> bool {
        *self == Self::default()
    }
}

impl TrackErrorRange {
    pub fn new(start: Time, length: Time) -> Self {
        TrackErrorRange { start, length }
    }

    pub fn new_from_end(start: Time, end: Time) -> Self {
        let length = end - start;
        TrackErrorRange { start, length }
    }
}

impl TestAndCopy {
    pub fn new(
        test_hash: String,
        copy_hash: String,
        test_skipzero_hash: String,
        copy_skipzero_hash: String
    ) -> Self {
        let integrity = Integrity::check_integrity(&test_hash, &copy_hash);
        let integrity_skipzero = Integrity::check_integrity(&test_skipzero_hash, &copy_skipzero_hash);
        TestAndCopy {
            test_hash,
            copy_hash,
            test_skipzero_hash,
            copy_skipzero_hash,
            integrity,
            integrity_skipzero,
        }
    }

    pub fn new_no_skipzero(test_hash: String, copy_hash: String) -> Self {
        TestAndCopy::new(test_hash, copy_hash, String::default(), String::default())
    }

    pub fn new_skipzero(test_hash: String, copy_hash: String) -> Self {
        TestAndCopy::new(String::default(), String::default(), test_hash, copy_hash)
    }

    pub fn new_integrity_no_data(test_hash: String, copy_hash: String) -> Self {
        let mut tc = TestAndCopy::new_no_skipzero(test_hash, copy_hash);
        if tc.integrity == Integrity::Match && !tc.test_hash.is_empty() && !tc.copy_hash.is_empty() {
            tc.integrity = Integrity::Unknown;
            tc.integrity_skipzero = Integrity::Unknown;
        }
        tc
    }
}

impl Default for TestAndCopy {
    fn default() -> Self {
        TestAndCopy {
            test_hash: String::default(),
            copy_hash: String::default(),
            test_skipzero_hash: String::default(),
            copy_skipzero_hash: String::default(),
            integrity: Integrity::Unknown,
            integrity_skipzero: Integrity::Unknown,
        }
    }
}