#![allow(dead_code)]
use core::fmt;
use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry;
use std::ffi::OsStr;
use std::fs::OpenOptions;
use std::io::{Write, Read};
use std::path::{Path, PathBuf};
use std::hash::{Hash, Hasher};

use codegen::{Block, Formatter, Scope};
use regex::Regex;
use walkdir::WalkDir;
use sha2::{Sha256, Digest};
use scraper::{Html, Selector};
use itertools::Itertools;

use simple_text_decode::DecodedText;
use accuraterip_drive_db::DriveEntry;

extern crate codegen;
extern crate reqwest;
extern crate scraper;
extern crate itertools;

type CodeMap = HashMap<String, HashMap<String, String>>;
type EacLangMap = HashMap<String, HashSet<String>>;

#[derive(Eq)]
struct DriveEntryMini(String, i16);

static AR_DRIVE_DB: &str = "http://www.accuraterip.com/driveoffsets.htm";

impl fmt::Display for DriveEntryMini {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(r#\"{}\"#, &{}_i16)", self.0, self.1)
    }
}

impl PartialEq for DriveEntryMini {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Hash for DriveEntryMini {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

fn get_sha_digest(text: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(text.as_bytes());
    let result = hasher.finalize();
    hex::encode(result)
}

fn get_nth_td_selector(n: i8) -> Selector {
    Selector::parse(format!("td:nth-child({})", n).as_str()).unwrap()
}

fn create_eac_translation_table() {

    fn append_imports(formatter: &mut Formatter) {
        let mut scope = Scope::new();
        scope.import("phf", "{phf_ordered_map, OrderedMap}");
        scope.import("super", "EacLanguage");

        scope.fmt(formatter).unwrap();
    }

    fn walk_translation_files(path: &Path) -> Vec<PathBuf> {
        let mut file_paths: Vec<PathBuf> = Vec::new();
        for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
            if entry.path().is_file() && entry.path().extension() == Some(OsStr::new("txt")) {
                file_paths.push(entry.path().to_path_buf());
            }
        }
        file_paths
    }

    fn generate_indexmap_preamble(name: &str) -> String {
        format!("pub static L_{}_MAP: OrderedMap<&'static str, &'static str> = phf_ordered_map!", name)
    }

    fn generate_static_language(l_k: &str, l: &str, idx: usize) -> String {
        format!(r#"pub static EL_{}_{}: EacLanguage = EacLanguage::new("{}", "{}", &L_{}_MAP);"#, l, idx, l_k, l, l)
    }

    fn get_mappings(file_paths: &Vec<PathBuf>) -> (CodeMap, EacLangMap) {

        let whitelist_vec = vec!["1", "10", "11", "12", "1200", "1203", "1204", "1210", "1211", "1212", "1213", "1214",
        "1215", "1216", "1217", "1218", "1219", "1220", "1221", "1222", "1223", "1224", "1225", "1226", "1227", "1228", "1229",
        "1230", "1232", "1233", "1234", "1235", "1236", "1237", "1238", "1239", "1240", "1241", "1242", "1243", "1244", "1245",
        "1246", "1247", "1248", "1249", "1250", "1251", "1252", "1253", "1254", "1255", "1256", "1257", "1258", "1259", "1260",
        "1261", "1262", "1263", "1264", "1265", "1266", "1267", "1268", "1269", "1270", "1271", "1272", "1273", "1274", "1275",
        "1276", "1277", "1278", "1279", "1280", "1281", "1282", "1283", "1284", "1285", "1286", "1287", "1288", "1289", "1290",
        "1291", "1292", "1293", "1294", "1295", "1296", "1297", "1298", "1299", "1305", "1306", "1307", "1308", "1309", "1310",
        "1320", "1321", "1322", "1323", "1324", "1325", "1328", "1329", "1330", "1331", "1332", "1333", "1334", "1335", "1336",
        "1337", "1338", "1339", "1340", "1341", "1342", "1343", "1344", "15", "16", "2", "2501", "31", "4270", "4271", "4272",
        "5", "50", "51", "52", "6", "7", "8", "81700", "81701", "81702", "81703", "81704", "81705", "81706", "81707", "81708",
        "81709", "81710", "81711", "81712", "81713", "81714", "81715", "81716", "81717", "81718", "81719", "81720"];
        let whitelist: HashSet<&str> = HashSet::from_iter(whitelist_vec);

        let pattern = Regex::new(r#"(?P<id>\d+) = \s*"(?P<str>.+)"$"#).unwrap();
        let pattern_l_id = Regex::new(r#"\s+4 = \s*"(?P<l_id>.+)""#).unwrap();
        let pattern_trail_colon = Regex::new(r#"\s*:\s*$"#).unwrap();
        let mut code_mapping: HashMap<String, HashMap<String, String>> = HashMap::new();
        let mut eac_lang_mapping: HashMap<String, HashSet<String>> = HashMap::new();
        let mut checked_files: HashSet<String> = HashSet::new();
        
        for file_path in file_paths {
            let mut file_bytes: Vec<u8> = Vec::new();
            let mut fh = OpenOptions::new().read(true).open(file_path).expect(
                "Could not open file",
            );

            fh.read_to_end(&mut file_bytes).expect(
                "Could not read file"
            );
            
            let file_content = match DecodedText::new(file_bytes) {
                Ok(log) => log.text,
                Err(e) => panic!("{}", e),
            };

            let file_text_hash = get_sha_digest(&file_content);
            if checked_files.contains(&file_text_hash) {
                continue;
            }
            checked_files.insert(file_text_hash);
            
            let lang_id = pattern_l_id.captures(&file_content)
                                                    .unwrap()["l_id"]
                                                    .to_string()
                                                    .to_ascii_uppercase();

            let mut mapping: HashMap<String, String> = HashMap::new();

            for line in file_content.lines() {
                if let Some(capture) = pattern.captures(line) {
                    let str_id = capture["id"].to_string();
                    let sub_str = capture["str"].to_string();

                    if !whitelist.contains(str_id.as_str()) {
                        continue;
                    }

                    if sub_str.chars().all(char::is_numeric) {
                        continue;
                    }

                    if str_id.eq("1274") {
                        match eac_lang_mapping.entry(lang_id.clone()) {
                            Entry::Occupied(mut o) => { o.get_mut().insert(sub_str.clone()); }
                            Entry::Vacant(v) => {
                                let mut set = HashSet::new();
                                set.insert(sub_str.clone());
                                v.insert(set);
                            }
                        }
                    }

                    let decolon_sub_str = pattern_trail_colon.replace_all(&sub_str, "").to_string();
                    if lang_id.eq("47AB3DF2") {
                        mapping.insert(str_id, decolon_sub_str);
                    } else {
                        mapping.insert(decolon_sub_str, str_id);
                    }
                }
            }
            
            if lang_id.is_empty() {
                // println!("{}", file_path.as_path().to_str().unwrap());
                panic!();
            }

            match code_mapping.entry(lang_id) {
                Entry::Occupied(mut o) => { o.get_mut().extend(mapping); },
                Entry::Vacant(v) => { v.insert(mapping); }
            }
            // break;
        }
        (code_mapping, eac_lang_mapping)
    }

    let src_dir_path = Path::new("./src/parser/eac_parser/translation_files");
    let out_file_path = Path::new("./src/parser/eac_parser/translation_table.rs");

    let out_file = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(out_file_path)
                    .unwrap();

    let mut buf = String::new();
    let mut formatter = Formatter::new(&mut buf);

    append_imports(&mut formatter);
    let (mappings, eac_mappings) = get_mappings(&walk_translation_files(src_dir_path));
    
    let mut dummy_block = Block::new(&generate_indexmap_preamble("DUMMY"));
    dummy_block.after(";");
    dummy_block.fmt(&mut formatter).unwrap();

    for (k, v) in mappings.iter() {
        let mut lang_block = Block::new(&generate_indexmap_preamble(k));
        let mut sorted_strlen: Vec<(&String, &String)> = v.iter().collect();
        sorted_strlen.sort_by(|a, b| b.0.len().cmp(&a.0.len()));
        for (sub, str_id) in sorted_strlen {
            lang_block.line(format!("r#\"{}\"# => \"{}\",", sub, str_id));
        }
        lang_block.after(";");
        lang_block.fmt(&mut formatter).unwrap();
    }

    let mut lang_vec: Vec<String> = Vec::new();
    
    for (lang, localised_key_set) in eac_mappings {
        for (idx, localised_key) in localised_key_set.iter().enumerate() {
            buf.push_str(generate_static_language(localised_key, &lang, idx).as_str());
            buf.push('\n');
            lang_vec.push(format!("&EL_{}_{}", lang, idx));
        }
    }

    buf.push_str(format!("pub static LANGS: [&EacLanguage; {}] = [", lang_vec.len()).as_str());
    buf.push_str("\n    ");
    buf.push_str(lang_vec.join(",\n    ").as_str());
    buf.push('\n');
    buf.push_str("];");

    write!(&out_file, "{}", buf).unwrap();
}

fn fetch_drive_offsets() {
    fn append_imports(formatter: &mut Formatter) {
        let mut scope = Scope::new();
        scope.import("phf", "{phf_map, Map}");
        scope.fmt(formatter).unwrap();
    }

    fn generate_vendor_map_preamble() -> String {
        String::from("pub static VENDOR_MAP: Map<&'static str, &'static [(&'static str, &'static i16)]> = phf_map!")
    }

    let resp = reqwest::blocking::get(AR_DRIVE_DB).unwrap();
    assert!(resp.status().is_success());

    let body = resp.text().unwrap();
    let fragment = Html::parse_document(&body);
    let selector = Selector::parse("body > table table > tbody > tr").unwrap();
    let mut drives: Vec<DriveEntry> = Vec::new();

    let drive_name_selector: Selector = get_nth_td_selector(1);
    let offset_selector: Selector = get_nth_td_selector(2);
    let submission_count_selector: Selector = get_nth_td_selector(3);
    let percentage_agree_selector: Selector = get_nth_td_selector(4); 
    
    for entry in fragment.select(&selector).skip(1) {
        let drive = DriveEntry::new(
            entry.select(&drive_name_selector).next().unwrap().text().next().unwrap_or_default().trim().to_owned(),
            entry.select(&offset_selector).next().unwrap().text().next().unwrap_or_default().trim().parse::<i16>().ok(),
            entry.select(&submission_count_selector).next().unwrap().text().next().unwrap_or_default().trim().parse::<i32>().ok(),
            entry.select(&percentage_agree_selector).next().unwrap().text().next().unwrap_or_default().replace('%', "").trim().parse::<f64>().ok(),
        );
        println!("{:?}", drive);
        if drive.offset.is_some() {
            drives.push(drive);
        }
    }

    let pattern: Regex = Regex::new(r"[^\s\w]").unwrap();
    let ws_pattern: Regex = Regex::new(r"\s+").unwrap();
    let mut drive_map: HashMap<String, HashSet<DriveEntryMini>> = HashMap::new();

    for drive in drives {
        let sanitised_drive_name = pattern.replace_all(drive.name.as_str(), "").trim().to_ascii_uppercase();
        let drive_vendor = sanitised_drive_name.as_str().split_whitespace().next().unwrap_or_default().to_string();
        drive_map.entry(drive_vendor).or_default().insert(DriveEntryMini(ws_pattern.replace_all(sanitised_drive_name.as_str(), "").to_string(), drive.offset.unwrap()));
    }

    let out_file_path = Path::new("./src/drive/offset_table.rs");

    let out_file = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(out_file_path)
                    .unwrap();

    let mut buf = String::new();
    let mut formatter = Formatter::new(&mut buf);

    append_imports(&mut formatter);

    let mut vendor_block = Block::new(&generate_vendor_map_preamble());
    for vendor in drive_map.keys() {
        vendor_block.line(format!("r#\"{}\"# => &VND_{},", vendor, vendor));
    }
    vendor_block.after(";\n");
    vendor_block.fmt(&mut formatter).unwrap();

    for (vendor, drive_list) in drive_map {
        buf.push_str(format!("pub static VND_{}: [(&'static str, &'static i16); {}] = [", vendor, drive_list.len()).as_str());
        buf.push_str("\n    ");
        buf.push_str(drive_list.iter().join(",\n    ").as_str());
        buf.push_str("\n];\n\n");
    }

    write!(&out_file, "{}", buf).unwrap();
}

fn main() {
    // create_eac_translation_table();
    // fetch_drive_offsets();
}