use std::{ops::{self, RangeInclusive}, time::Duration};

use serde::{Serialize, Deserialize};
use ts_rs::TS;

const PORT_RANGE: RangeInclusive<usize> = 1..=65535;

#[derive(Clone, Copy, PartialEq)]
pub struct Time(Duration);

impl Time {
    pub fn from_ss(ss: &str) -> Time {
        Time(Duration::from_secs_f64(str::parse(ss).unwrap()))
    }

    pub fn from_mm_ss(mm_ss: &str) -> Time {
        let split: Vec<&str> = mm_ss.split(':').collect();
        let m: u64 = str::parse(split[0]).unwrap();
        let s: f64 = str::parse(split[1]).unwrap();
        Time(Duration::from_secs(m * 60) + Duration::from_secs_f64(s))
    }
    
    pub fn from_h_mm_ss(h_mm_ss: &str) -> Time {
        let split: Vec<&str> = h_mm_ss.split(':').collect();
        let h: u64 = str::parse(split[0]).unwrap();
        let m: u64 = str::parse(split[1]).unwrap();
        let s: f64 = str::parse(split[2]).unwrap();
        Time(Duration::from_secs(h * 3600) + Duration::from_secs(m * 60) + Duration::from_secs_f64(s))
    }

    pub fn from_mm_ss_cs(mm_ss_cs: &str) -> Time {
        let split: Vec<&str> = mm_ss_cs.split(':').collect();
        let m: u64 = str::parse(split[0]).unwrap();
        let s: u64 = str::parse(split[1]).unwrap();
        let cs: u64 = str::parse(split[2]).unwrap();
        Time(Duration::from_secs(m * 60) + Duration::from_secs(s) + Duration::from_millis(cs * 10))
    }
}

impl ops::Add<Time> for Time {
    type Output = Time;

    fn add(self, rhs: Time) -> Self::Output {
        Time(self.0 + rhs.0)
    }
}

impl ops::Sub<Time> for Time {
    type Output = Time;

    fn sub(self, rhs: Time) -> Self::Output {
        Time(self.0 - rhs.0)
    }
}

impl Serialize for Time {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        self.0.as_secs_f64().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Time {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        match Deserialize::deserialize(deserializer) {
            Ok(v) => Ok(Time::from_ss(v)),
            Err(e) => Err(e),
        }
    }
}

impl TS for Time {
    fn name() -> String {
        String::from("string")
    }

    fn dependencies() -> Vec<ts_rs::Dependency> {
        Vec::new()
    }

    fn transparent() -> bool {
        false
    }
}

pub fn first_line(string: &str) -> &str {
    string.lines().next().unwrap()
}

pub fn env_getter(key: &str, default: &str) -> String {
    let env_val = std::env::var_os(key).unwrap_or_default();
    std::str::from_utf8(env_val.as_encoded_bytes()).unwrap_or(default).to_owned()
}

pub fn port_in_range(s: &str) -> Result<String, String> {
    let port: usize = s
        .parse()
        .map_err(|_| format!("`{s}` isn't a port number"))?;
    if PORT_RANGE.contains(&port) {
        Ok(port.to_string())
    } else {
        Err(format!(
            "port not in range {}-{}",
            PORT_RANGE.start(),
            PORT_RANGE.end()
        ))
    }
}