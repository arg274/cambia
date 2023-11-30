use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct WhipperRippingPhaseInfo {
    #[serde(rename = "Drive")]
    pub drive: String,
    #[serde(rename = "Extraction engine")]
    pub engine: String,
    #[serde(rename = "Defeat audio cache")]
    pub cache: String,
    #[serde(rename = "Read offset correction")]
    pub read_offset: i16,
    #[serde(rename = "Overread into lead-out")]
    pub overread: String,
    #[serde(rename = "Gap detection")]
    pub gap: String,
    #[serde(rename = "CD-R detected")]
    pub cdr: String,
}

#[derive(Serialize, Deserialize)]
pub struct WhipperCDMetadata {
    #[serde(alias = "Release")]
    #[serde(alias = "Album")]
    pub release: String,
    #[serde(rename = "CDDB Disc ID")]
    pub cddb_id: String,
    #[serde(rename = "MusicBrainz Disc ID")]
    pub mbz_id: String,
    #[serde(rename = "MusicBrainz lookup URL")]
    pub mbz_lookup_url: String,
    #[serde(rename = "MusicBrainz Release URL")]
    pub mbz_release_url: String,
}

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub struct WhipperLogYaml {
    #[serde(rename = "Log created by")]
    pub version: String,
    #[serde(rename = "Log creation date")]
    pub rip_date: String,
    #[serde(rename = "Ripping phase information")]
    pub ripping_phase_info: WhipperRippingPhaseInfo,
    // #[serde(rename = "CD metadata")]
    // pub cd_metadata: WhipperCDMetadata,
    #[serde(rename = "TOC")]
    pub toc: HashMap<u32, WhipperTocEntry>,
    #[serde(default)]
    #[serde(rename = "SHA-256 hash")]
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
                cache: String::from("Unknown"),
                read_offset: 0,
                overread: String::from("Unknown"),
                gap: String::from("Unknown"),
                cdr: String::from("Unknown"),
            },
            // cd_metadata: WhipperCDMetadata {
            //     release: String::from("Unknown"),
            //     cddb_id: String::from("Unknown"),
            //     mbz_id: String::from("Unknown"),
            //     mbz_lookup_url: String::from("Unknown"),
            //     mbz_release_url: String::from("Unknown"),
            // },
            toc: HashMap::new(),
            checksum: String::from("Unknown")
        }
    }
}