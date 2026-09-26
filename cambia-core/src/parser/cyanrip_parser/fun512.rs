use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use sha2::{Digest, Sha512};

pub const MARKER: &str = "Log FUN512: ";

const MAX_IDX: u8 = 16;

fn permute(sha512_digest: &[u8; 64], idx: u8) -> String {
    let mut digest = *sha512_digest;

    for b in digest.iter_mut() {
        *b ^= 0x81u8.wrapping_add(idx);
    }
    for j in 0..64 {
        for k in 0..64 {
            if j != k {
                digest[j] ^= digest[k];
            }
        }
    }

    let encoded = STANDARD.encode(digest);
    encoded
        .trim_end_matches('=')
        .chars()
        .map(|c| match c {
            '/' => '_',
            '+' => '.',
            other => other,
        })
        .collect()
}

pub fn extract(log: &str) -> Option<&str> {
    let pos = log.rfind(MARKER)?;
    let rest = &log[pos + MARKER.len()..];
    Some(rest.split(['\r', '\n']).next().unwrap_or(""))
}

pub fn calculate(log: &str) -> String {
    let Some(pos) = log.rfind(MARKER) else {
        return String::new();
    };

    let rest = &log[pos + MARKER.len()..];
    let truth_len = rest.find(['\r', '\n']).unwrap_or(rest.len());
    if !rest[truth_len..].trim_matches(['\r', '\n']).is_empty() {
        return String::new();
    }

    let mut hasher = Sha512::new();
    hasher.update(&log.as_bytes()[..pos]);
    let digest: [u8; 64] = hasher.finalize().into();

    let truth = &rest[..truth_len];
    (0..MAX_IDX)
        .map(|idx| permute(&digest, idx))
        .find(|candidate| candidate == truth)
        .unwrap_or_else(|| permute(&digest, 0))
}

#[cfg(test)]
mod tests {
    use super::*;

    // Vectors and cases lifted from cyanrip's own tests/fun512.c
    const BODY: &str = "cyanrip FUN512 test vector
";
    const BODY_FUN512_0: &str =
        "yeSm9QaxaDSB2ZGPkY9tC.dSvAdPrPSAzIEp0y_sIyYH7B9w0lbIuQmYoimZ0Mf6CbLwzYm_za_y4MKvwSQS4Q";
    const BODY_FUN512_1: &str =
        "yeel9gWyazeC2pKMkoxuCORRvwRMr_eDz4Iq0CzvICUE7xxz0VXLugqboSqa08T5CrHzzoq8zqzx48GswicR4g";

    fn digest_of(body: &str) -> [u8; 64] {
        let mut hasher = Sha512::new();
        hasher.update(body.as_bytes());
        hasher.finalize().into()
    }

    #[test]
    fn matches_upstream_vectors() {
        let digest = digest_of(BODY);
        assert_eq!(permute(&digest, 0), BODY_FUN512_0);
        assert_eq!(permute(&digest, 1), BODY_FUN512_1);
    }

    #[test]
    fn accepts_a_log_from_either_output_index() {
        for want in [BODY_FUN512_0, BODY_FUN512_1] {
            let log = format!("{BODY}{MARKER}{want}
");
            assert_eq!(calculate(&log), want);
        }
    }

    #[test]
    fn tampered_body_does_not_match() {
        let log = format!("not the body
{MARKER}{BODY_FUN512_0}
");
        assert_ne!(calculate(&log), BODY_FUN512_0);
    }

    #[test]
    fn a_log_without_a_checksum_yields_nothing() {
        assert_eq!(extract(BODY), None);
        assert!(calculate(BODY).is_empty());
    }

    #[test]
    fn permutation_is_stable_and_index_dependent() {
        let digest = [0u8; 64];
        let a = permute(&digest, 0);
        assert_eq!(a, permute(&digest, 0));
        assert_ne!(a, permute(&digest, 1));
        assert!(!a.contains('/') && !a.contains('+') && !a.contains('='));
    }

    #[test]
    fn extracts_the_last_marker() {
        let log = format!("body\n{MARKER}FIRST\nmore\n{MARKER}SECOND\n");
        assert_eq!(extract(&log), Some("SECOND"));
    }

    #[test]
    fn trailing_data_after_the_checksum_line_fails() {
        let log = format!("body\n{MARKER}ABC\ntampered\n");
        assert!(calculate(&log).is_empty());
    }

    #[test]
    fn round_trips_against_itself() {
        let body = "cyanrip 0.9.3 (c23f1c4)\nRipping errors: 0\n";
        let mut hasher = Sha512::new();
        hasher.update(body.as_bytes());
        let digest: [u8; 64] = hasher.finalize().into();
        let log = format!("{body}{MARKER}{}\n", permute(&digest, 3));
        assert_eq!(calculate(&log), extract(&log).unwrap());
    }
}
