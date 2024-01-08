use serde::{Serialize, Deserialize};
use sha1::{Sha1, Digest};
use base64::{Engine as _, engine::GeneralPurpose, engine::general_purpose::PAD, alphabet::Alphabet};
use ts_rs::TS;
use urlencoding::encode;

use crate::util::Time;

#[derive(Serialize, Deserialize, Clone, Copy, TS)]
#[ts(export)]
pub struct TocEntry {
    pub track: u32,
    pub start: Time,
    pub length: Time,
    pub start_sector: u32,
    pub end_sector: u32,
}

#[derive(Serialize, Deserialize, Clone, Default, TS)]
#[ts(export)]
pub struct TocRaw {
    pub entries: Vec<TocEntry>,
    pub lead_out: u32,
    pub data_tracks: u32,
}

#[derive(Serialize, Deserialize, TS)]
#[ts(export)]
pub struct TocHash {
    hash: String,
    url: String,
}

#[derive(Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Toc {
    pub raw: TocRaw,
    pub mbz: TocHash,
    pub ctdb_tocid: TocHash,
    pub gn: TocHash,
    pub mcdi: TocHash,
}

pub struct TocError;

impl TocEntry {
    pub fn new(track: u32, start: Time, length: Time, start_sector: u32, end_sector: u32) -> TocEntry {
        TocEntry {
            track,
            start,
            length,
            start_sector,
            end_sector,
        }
    }
}

impl TocRaw {
    pub fn new(entries: Vec<TocEntry>) -> TocRaw {

        let data_tracks = get_data_tracks(&entries);

        match data_tracks {
            Ok(val) => {
                let lead_out = entries[entries.len().saturating_sub(val as usize).saturating_sub(1)].end_sector + 1;

                TocRaw {
                    entries,
                    lead_out,
                    data_tracks: val,
                }
            }
            Err(_) => TocRaw::default(),
        } 
    }
}

impl TocHash {
    pub fn new(hash: String, url: String) -> TocHash {
        TocHash { hash, url }
    }
}

impl Default for TocHash {
    fn default() -> Self {
        TocHash::new(String::default(), String::default())
    }
}

impl Toc {
    #[allow(clippy::redundant_clone)]
    pub fn new(toc_raw: TocRaw) -> Toc {
        Toc {
            raw: toc_raw.clone(),
            mbz: raw_to_mbz(toc_raw.clone()),
            ctdb_tocid: raw_to_ctdb_tocid(toc_raw.clone()),
            gn: raw_to_gn(toc_raw.clone()),
            mcdi: raw_to_mcdi(toc_raw.clone()),
        }
    }
}

impl Default for Toc {
    fn default() -> Self {
        Toc::new(TocRaw::default())
    }
}

pub fn raw_to_mbz(toc_raw: TocRaw) -> TocHash {
    let mut offsets: Vec<TocEntry> = toc_raw.entries.clone();

    if offsets.is_empty() {
        return TocHash::default();
    }

    if toc_raw.data_tracks > 0 {
        offsets.truncate(offsets.len().saturating_sub(toc_raw.data_tracks as usize));
    }

    let mut sb = format!("{:02X}{:02X}{:08X}",
                                offsets[0].track,
                                // TODO: Not sure if it should use the last audio track or include the data tracks
                                offsets.last().copied().unwrap().track,
                                toc_raw.lead_out + 150);
    // TODO: Loop can probably be avoided
    for idx in 0..99 {
        if idx < offsets.len() {
            sb.push_str(format!("{:08X}", offsets[idx].start_sector + 150).as_str());
        } else {
            sb.push_str("00000000");
        }
    }

    let mut hasher = Sha1::new();
    hasher.update(sb.as_bytes());
    let result = hasher.finalize();

    let mbz_b64_alphabet = Alphabet::new("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789._").unwrap();
    let mbz_b64_engine = GeneralPurpose::new(&mbz_b64_alphabet, PAD);

    let url_param: String = format!("{} {} {} {}",
                                    offsets[0].track,
                                    offsets.last().copied().unwrap().track,
                                    (toc_raw.lead_out + 150),
                                    offsets.iter().map(|offset| (offset.start_sector + 150).to_string()).collect::<Vec<String>>().join(" ")
                                );

    TocHash::new(mbz_b64_engine.encode(result).replace('=', "-"), format!("https://musicbrainz.org/cdtoc/attach?toc={}", encode(&url_param)))

    
}

pub fn raw_to_ctdb_tocid(toc_raw: TocRaw) -> TocHash {
    let mut entries: Vec<TocEntry> = toc_raw.entries.clone();

    if entries.is_empty() {
        return TocHash::default();
    }

    if toc_raw.data_tracks > 0 {
        entries.truncate(entries.len().saturating_sub(toc_raw.data_tracks as usize));
    }

    let pregap = entries[0].start_sector;
    let mut sb: String = String::new();

    for entry in entries.iter().skip(1) {
        sb.push_str(format!("{:08X}", entry.start_sector - pregap).as_str())
    }
    sb.push_str(format!("{:08X}", toc_raw.lead_out - pregap).as_str());
    // TODO: Loop can probably be avoided
    for _ in 0..(100_usize.saturating_sub(entries.len())) {
        sb.push_str("00000000");
    }

    let mut hasher = Sha1::new();
    hasher.update(sb.as_bytes());
    let result = hasher.finalize();

    let ctdb_b64_alphabet = Alphabet::new("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789._").unwrap();
    let ctdb_b64_engine = GeneralPurpose::new(&ctdb_b64_alphabet, PAD);

    let hash = ctdb_b64_engine.encode(result).replace('=', "-");
    let url_param: String = encode(&hash).to_string();
    TocHash::new(hash, format!("http://db.cuetools.net/top.php?tocid={}", url_param))
}

pub fn raw_to_gn(toc_raw: TocRaw) -> TocHash {

    let mut offsets: Vec<u32> = toc_raw.entries.iter().map(|toc_entry| toc_entry.start_sector).collect();

    if offsets.is_empty() {
        return TocHash::default();
    }

    if toc_raw.data_tracks > 0 {
        offsets.truncate(offsets.len().saturating_sub(toc_raw.data_tracks as usize));
    }

    offsets.push(toc_raw.lead_out);

    let pregap: u32 = if offsets[0] == 0 {
        150
    } else {
        0
    };

    let offsets_str: Vec<String> = offsets.iter().map(|offset| (offset + pregap).to_string()).collect();
    
    TocHash::new(offsets_str.join(" "), String::default())
}

pub fn raw_to_mcdi(toc_raw: TocRaw) -> TocHash {

    let mut offsets: Vec<u32> = toc_raw.entries.iter().map(|toc_entry| toc_entry.start_sector).collect();

    if offsets.is_empty() {
        return TocHash::default();
    }

    if toc_raw.data_tracks > 0 {
        offsets.truncate(offsets.len().saturating_sub(toc_raw.data_tracks as usize));
    }

    offsets.push(toc_raw.lead_out);
    
    let pregap: u32 = if offsets[0] == 0 {
        150
    } else {
        0
    };

    let offsets_str: Vec<String> = offsets.iter().map(|offset| format!("{:X}", offset + pregap)).collect();
    let offsets_joined = offsets_str.join("+");
    
    TocHash::new(offsets_joined.clone(), format!("https://musicmatch-ssl.xboxlive.com/cdinfo/GetMDRCD.aspx?locale=409&geoid=f4&version=12.0.17134.48&userlocale=409&CD={}", offsets_joined))
}

fn get_data_tracks(entries: &Vec<TocEntry>) -> Result<u32, TocError> {
    if entries.is_empty() {
        return Err(TocError);
    }

    let last_idx = entries.len().saturating_sub(1);

    for (idx, _) in entries.iter().enumerate() {
        if idx < last_idx {
            if entries[idx].end_sector > entries[idx + 1].start_sector {
                return Err(TocError);
            }
            let gap = entries[idx + 1].start_sector - entries[idx].end_sector - 1;
            if gap != 0 {
                if gap == 11400 {
                    return Ok((last_idx - idx) as u32);
                } else {
                    return Err(TocError);
                }
            }
        }
    }

    Ok(0)
}