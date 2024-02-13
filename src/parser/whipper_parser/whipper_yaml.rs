use indexmap::IndexMap;
use serde::{Serialize, Deserialize};

use crate::extract::ReleaseInfo;

// This separate struct is to stop ts-rs from complaining
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ReleaseInfoYaml {
    #[serde(rename = "Artist")]
    artist: String,
    #[serde(rename = "Title")]
    title: String,
}

impl From<ReleaseInfoYaml> for ReleaseInfo {
    fn from(val: ReleaseInfoYaml) -> Self {
        ReleaseInfo::new(val.artist, val.title)
    }
}

impl From<ReleaseInfo> for ReleaseInfoYaml {
    fn from(value: ReleaseInfo) -> Self {
        Self { artist: value.artist, title: value.title }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ReleaseInfoUnion {
    ReleaseInfo(ReleaseInfoYaml),
    String(String),
}

impl Default for ReleaseInfoUnion {
    fn default() -> Self {
        Self::ReleaseInfo(ReleaseInfo::default().into())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WhipperRippingPhaseInfo {
    #[serde(rename = "Drive")]
    pub drive: String,
    #[serde(rename = "Extraction engine")]
    pub engine: String,
    #[serde(rename = "Defeat audio cache")]
    pub cache: Option<String>,
    #[serde(rename = "Read offset correction")]
    pub read_offset: i16,
    #[serde(rename = "Overread into lead-out")]
    pub overread: Option<String>,
    #[serde(rename = "Gap detection")]
    pub gap: String,
    #[serde(rename = "CD-R detected")]
    pub cdr: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WhipperCDMetadata {
    #[serde(default, alias = "Release", alias = "Album")]
    pub release: ReleaseInfoUnion,
    #[serde(default, rename = "CDDB Disc ID")]
    pub cddb_id: String,
    #[serde(default, rename = "MusicBrainz Disc ID")]
    pub mbz_id: String,
    #[serde(default, alias = "MusicBrainz lookup URL", alias = "MusicBrainz lookup url")]
    pub mbz_lookup_url: String,
    #[serde(default, alias = "MusicBrainz Release URL", alias = "MusicBrainz Release url")]
    pub mbz_release_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WhipperTocEntry {
    #[serde(rename = "Start")]
    pub start: String,
    #[serde(rename = "Length")]
    pub length: String,
    #[serde(rename = "Start sector")]
    pub start_sector: u32,
    #[serde(rename = "End sector")]
    pub end_sector: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WhipperTrackEntry {
    #[serde(rename = "Filename")]
    pub filename: String,
    #[serde(rename = "Pre-gap length")]
    pub pregap: Option<String>,
    #[serde(rename = "Peak level")]
    pub peak_level: f64,
    #[serde(rename = "Pre-emphasis")]
    pub preemphasis: Option<String>,
    #[serde(rename = "Extraction speed")]
    pub extraction_speed: String,
    #[serde(rename = "Extraction quality")]
    pub extraction_quality: String,
    #[serde(rename = "Test CRC")]
    pub test_crc: String,
    #[serde(rename = "Copy CRC")]
    pub copy_crc: String,
    #[serde(rename = "Status")]
    pub status: String,
    // FIXME: AccurateRip missing
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WhipperLogYaml {
    #[serde(rename = "Log created by")]
    pub version: String,
    #[serde(rename = "Log creation date")]
    pub rip_date: String,
    #[serde(rename = "Ripping phase information")]
    pub ripping_phase_info: WhipperRippingPhaseInfo,
    #[serde(rename = "CD metadata")]
    pub cd_metadata: WhipperCDMetadata,
    #[serde(rename = "TOC")]
    pub toc: IndexMap<u32, WhipperTocEntry>,
    #[serde(rename = "Tracks")]
    pub tracks: IndexMap<usize, WhipperTrackEntry>,
    #[serde(default, rename = "SHA-256 hash")]
    pub checksum: String,
}

impl Default for WhipperLogYaml {
    fn default() -> Self {
        WhipperLogYaml {
            version: String::from("Unknown"),
            rip_date: String::from("Unknown"),
            ripping_phase_info: WhipperRippingPhaseInfo {
                drive: String::from("Unknown"),
                engine: String::from("Unknown"),
                cache: None,
                read_offset: 0,
                overread: None,
                gap: String::from("Unknown"),
                cdr: None,
            },
            cd_metadata: WhipperCDMetadata {
                release: ReleaseInfoUnion::default(),
                cddb_id: String::from("Unknown"),
                mbz_id: String::from("Unknown"),
                mbz_lookup_url: String::from("Unknown"),
                mbz_release_url: String::from("Unknown"),
            },
            toc: IndexMap::new(),
            tracks: IndexMap::new(),
            checksum: String::from("Unknown")
        }
    }
}