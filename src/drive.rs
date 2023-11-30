use std::ops::Deref;

use self::offset_table::VENDOR_MAP;
use aho_corasick::AhoCorasick;
use rayon::prelude::IntoParallelRefIterator;
use textdistance::str::levenshtein;
use regex::Regex;
use rayon::prelude::*;

#[allow(clippy::redundant_static_lifetimes)]
mod offset_table;

lazy_static! {
    static ref DRIVE_SANITISATION: Regex = Regex::new(r"(?i)revision(.*)|[^\w\s]").unwrap();
    static ref WS_FILTER: Regex = Regex::new(r"\s+").unwrap();
}

pub static VENDOR_SUB_KEYS: &[&str] = &["JLMS", "HLDTST", "MATSHITA"];
pub static VENDOR_SUB_VALS: &[&str] = &["LITEON", "LG ELECTRONICS", "PANASONIC"];
static DISTANCE_THRESHOLD: usize = 5;

pub enum DriveMatchQuality {
    STRONG(i16),
    WEAK(i16),
}

pub struct DriveUtils;

impl DriveUtils {
    fn santitise_drive(drive: String) -> String {
        let drive_sanitised = DRIVE_SANITISATION.replace_all(drive.as_str(), "").to_string();

        let ac = AhoCorasick::new(VENDOR_SUB_KEYS);
        ac.replace_all(&drive_sanitised, VENDOR_SUB_VALS).to_ascii_uppercase()
    }
    
    pub fn fuzzy_search_vendor(drive: String, sanitise: bool) -> String {

        let drive_sanitised: String = if sanitise { Self::santitise_drive(drive) } else { drive };

        let log_vendor = drive_sanitised.split_whitespace().next().unwrap_or_default();
        let (matched_vendor, _distance) = VENDOR_MAP.keys()
            .map(|vendor| (vendor, levenshtein(vendor, log_vendor)))
            .min_by_key(|&(_, dist)| dist)
            .unwrap();
        matched_vendor.to_string()
    }

    pub fn fuzzy_search_model(drive: String) -> DriveMatchQuality {
        let mut drive_sanitised = Self::santitise_drive(drive);
        let vendor = Self::fuzzy_search_vendor(drive_sanitised.clone(), false);
        drive_sanitised = WS_FILTER.replace_all(&drive_sanitised, "").to_string();

        let (_matched_drive, offset, distance) = VENDOR_MAP.get(&vendor).unwrap()
            .deref().par_iter()
            .map(|&(drv, offset)| (drv, offset, levenshtein(drv, &drive_sanitised)))
            .min_by_key(|&(_, _, dist)| dist)
            .unwrap();

        // println!("Log drive: {}", drive_sanitised);
        // println!("Matched drive: {} w/ offset: {}", _matched_drive, offset);

        if distance > DISTANCE_THRESHOLD {
            DriveMatchQuality::WEAK(*offset)
        } else {
            DriveMatchQuality::STRONG(*offset)
        }
    }
}