use std::fmt;
use encoding_rs::Encoding;
use unicode_bom::Bom;
use widestring::Utf32String;

use chardetng::EncodingDetector;

#[derive(Debug)]
pub struct DecodedText {
    pub text: String,
    pub orig_encoding: String,
}

pub struct DecodingError;

impl DecodedText {
    pub fn new(raw: Vec<u8>) -> Result<DecodedText, DecodingError> {
        
        let bom: Bom = Bom::from(raw.as_slice());
        let mut encoding: Option<&Encoding> = None;
        println!("BOM {}", bom);

        match bom {
            Bom::Utf8 => encoding = Encoding::for_label(b"utf-8"),
            Bom::Utf16Be => encoding = Encoding::for_label(b"utf-16be"),
            Bom::Utf16Le => encoding = Encoding::for_label(b"utf-16le"),
            Bom::Gb18030 => {
                encoding = Encoding::for_label(b"gb18030");
                // encoding_rs claims to not consider gb18030 BOM as it is not a web standard
                let decoded = encoding.unwrap().decode(&raw[2..]);
                return Ok(DecodedText {
                    text: decoded.0.into_owned(),
                    orig_encoding: encoding.unwrap().name().to_owned(),
                });
            },
            Bom::Utf32Be => {
                let wide_bytes: Vec<u32> = raw.chunks(4)
                                            .map(|chunk| chunk.iter().fold(0u32, |acc, &x| (acc << 8) | x as u32))
                                            .collect();
                match Utf32String::from_vec(wide_bytes) {
                    Ok(d) => {
                        // Not sure if this lib handles BOM
                        return Ok(DecodedText {
                            text: d.to_string(),
                            orig_encoding: String::from("UTF-32BE"),
                        });
                    }
                    Err(_) => { return Err(DecodingError); } 
                }
            },
            Bom::Utf32Le => {
                let wide_bytes: Vec<u32> = raw.chunks(4)
                                            .map(|chunk| chunk.iter().rev().fold(0u32, |acc, &x| (acc << 8) | x as u32))
                                            .collect();
                match Utf32String::from_vec(wide_bytes) {
                    Ok(d) => {
                        // Not sure if this lib handles BOM
                        return Ok(DecodedText {
                            text: d.to_string(),
                            orig_encoding: String::from("UTF-32LE"),
                        });
                    }
                    Err(_) => { return Err(DecodingError); } 
                }
            },
            // No encoding_rs support for post UTF-8 encodings
            Bom::Bocu1 | Bom::Scsu | Bom::UtfEbcdic | Bom::Utf1 | Bom::Utf7 => (),
            Bom::Null => {
                let mut detector = EncodingDetector::new();
                detector.feed(&raw, true);
                let guess = detector.guess_assess(None, true);
                if guess.1 {
                    encoding = Some(guess.0);
                }
            },
        }

        if encoding.is_none() {
            return Err(DecodingError);
        }

        println!("ENCODING: {} BRUH", encoding.unwrap().name());
        
        let decoded = encoding.unwrap().decode(&raw);

        Ok(DecodedText {
            text: decoded.0.into_owned(),
            orig_encoding: encoding.unwrap().name().to_owned(),
        })
    }
}

impl fmt::Display for DecodingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Decoding Error")
    }
}

impl Default for DecodedText {
    fn default() -> Self {
        Self { text: String::default(), orig_encoding: String::from("unknown") }
    }
}