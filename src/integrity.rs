use serde::{Serialize, Deserialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, PartialEq, TS, Clone)]
#[ts(export)]
pub enum Integrity {
    Match,
    Mismatch,
    Unknown,
}

#[derive(Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Checksum {
    pub log: String,
    pub calculated: String,
    pub integrity: Integrity,
}

pub trait IntegrityChecker {
    fn extract_checksum(&self) -> String {
        String::new()
    }

    fn calculate_checksum(&self) -> String {
        String::new()
    }

    fn get_checksum(&self) -> Checksum {
        let old = self.extract_checksum();
        let new = self.calculate_checksum();
        let integrity = Integrity::check_integrity(&old, &new);
        Checksum::new(old, new, integrity)
    }
}

impl Integrity {
    pub fn check_integrity(old: &str, new: &str) -> Integrity {
        if old.is_empty() || new.is_empty() { Integrity::Unknown }
        else {
            match old == new {
                true => Integrity::Match,
                false => Integrity::Mismatch
            }
        }
    }
}

impl Checksum {
    pub fn new(log: String, calculated: String, integrity: Integrity) -> Checksum {
        Checksum {
            log,
            calculated,
            integrity
        }
    }
}