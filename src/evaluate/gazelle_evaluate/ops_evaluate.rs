use std::{cmp::min, collections::{HashMap, HashSet}};

use crate::{evaluate::{Evaluator, EvaluationCombined, EvaluationUnit, Evaluation, EvaluatorType, EvaluationUnitScope}, parser::{ParsedLogCombined, ParsedLog}, extract::{Ripper, Quartet, MediaType, ReadMode, Gap}, track::TrackEntry, integrity::Integrity, drive::{DriveUtils, DriveMatchQuality}};

use super::{GazelleDeductionData, GazelleDeductionFail, GazelleDeductionRelease, GazelleDeductionTrack, GazelleDeduction};

use regex::{Regex, RegexBuilder};
use semver::{Version, Prerelease, BuildMetadata};
use serde_yaml::from_str;
use strum::IntoEnumIterator;
use rayon::prelude::*;

lazy_static! {
    static ref EXTENSION: Regex = Regex::new(r"\..+$").unwrap();
    static ref WHIPPER_VERSION: Regex = RegexBuilder::new(r"(?P<major>\d+)\.(?P<minor>\d+)\.(?P<patch>\d+)(\.(?P<pre>[0-9a-z]+))?(\+(?P<build>[0-9a-z]+))?").case_insensitive(true).build().unwrap();
    static ref OPS_EXTENSION_ALLOWLIST: Regex = Regex::new(r"(wav|flac|ape)$").unwrap();
}

static WHIPPER_VERSION_THRESH: Version = Version {
    major: 0,
    minor: 7,
    patch: 3,
    pre: Prerelease::EMPTY,
    build: BuildMetadata::EMPTY,
};

#[derive(Default)]
pub struct OpsEvaluator;

impl OpsEvaluator {
    pub fn new() -> Self {
        Self
    }

    pub fn check_fail(parsed_log: &ParsedLog, data: GazelleDeductionFail) -> bool {
        match data {
            GazelleDeductionFail::UnknownEncoding => false,
            GazelleDeductionFail::UnknownRipper => parsed_log.ripper != Ripper::EAC && parsed_log.ripper != Ripper::XLD && parsed_log.ripper != Ripper::Whipper,
            GazelleDeductionFail::WhipperVersionLowerLimit => {
                if parsed_log.ripper != Ripper::Whipper {
                    return false;
                }

                match WHIPPER_VERSION.captures(&parsed_log.ripper_version) {
                    Some(c) => {
                        let r_v = Version {
                            major: c.name("major").unwrap().as_str().parse().unwrap(),
                            minor: c.name("minor").unwrap().as_str().parse().unwrap(),
                            patch: c.name("patch").unwrap().as_str().parse().unwrap(),
                            pre: if c.name("pre").is_some() { Prerelease::new(c.name("patch").unwrap().as_str()).unwrap() } else { Prerelease::EMPTY },
                            build: if c.name("build").is_some() { BuildMetadata::new(c.name("build").unwrap().as_str()).unwrap() } else { BuildMetadata::EMPTY },
                        };

                        r_v < WHIPPER_VERSION_THRESH
                    },
                    None => true,
                }
            },
            GazelleDeductionFail::CouldNotParseWhipper => false,
        }
    }

    pub fn check_release(parsed_log: &ParsedLog, data: GazelleDeductionRelease) -> bool {

        match data {
            GazelleDeductionRelease::VirtualDrive => parsed_log.drive.to_lowercase().contains("generic dvd-rom scsi cdrom device"),
            GazelleDeductionRelease::NullDrive => parsed_log.drive.to_lowercase().contains("(null) (null) (revision (null))"),
            GazelleDeductionRelease::IncorrectReadOffset => {
                match DriveUtils::fuzzy_search_model(parsed_log.drive.clone()) {
                    DriveMatchQuality::STRONG(matched_offset) => {
                        parsed_log.read_offset.is_some() && matched_offset.is_some() && parsed_log.read_offset.unwrap() != matched_offset.unwrap()
                    }
                    DriveMatchQuality::WEAK(_) => {
                        false
                    },
                }
            },
            GazelleDeductionRelease::DriveNotFoundDb => {
                match DriveUtils::fuzzy_search_model(parsed_log.drive.clone()) {
                    DriveMatchQuality::STRONG(matched_offset) => {
                        matched_offset.is_none()
                    }
                    DriveMatchQuality::WEAK(_) => {
                        parsed_log.read_offset.is_some() && parsed_log.read_offset.unwrap() == 0
                    },
                }
            },
            GazelleDeductionRelease::DefeatAudioCacheDisabled => parsed_log.defeat_audio_cache == Quartet::False,
            GazelleDeductionRelease::EacVersionOld => parsed_log.ripper == Ripper::EAC && (parsed_log.ripper_version == "Unknown" || parsed_log.ripper_version.cmp(&String::from("0.99")).is_lt()),
            GazelleDeductionRelease::XldNoChecksum => false,
            GazelleDeductionRelease::Mp3Log => !parsed_log.audio_encoder.is_empty() && parsed_log.audio_encoder.iter().all(|encoder| encoder.contains("mp3") || encoder.contains("lame")),
            GazelleDeductionRelease::CouldNotVerifyDrive => parsed_log.drive == "Unknown Drive",
            GazelleDeductionRelease::CouldNotVerifyMedia => parsed_log.ripper == Ripper::XLD && parsed_log.ripper_version.cmp(&String::from("20130127")).is_ge() && parsed_log.media_type == MediaType::Unknown, 
            GazelleDeductionRelease::CouldNotVerifyReadMode => parsed_log.read_mode == ReadMode::Unknown,
            GazelleDeductionRelease::CouldNotVerifyMaxRetry => false, // TODO: XLD specific prop, does not affect scoring
            GazelleDeductionRelease::CouldNotVerifyAccurateStream => parsed_log.read_mode == ReadMode::Secure && parsed_log.accurate_stream == Quartet::Unknown,
            GazelleDeductionRelease::CouldNotVerifyDefeatAudioCache => parsed_log.read_mode == ReadMode::Secure && parsed_log.defeat_audio_cache == Quartet::Unknown,
            GazelleDeductionRelease::CouldNotVerifyC2 => parsed_log.read_mode == ReadMode::Secure && parsed_log.use_c2 == Quartet::Unknown,
            GazelleDeductionRelease::CouldNotVerifyReadOffset => parsed_log.read_offset.is_none(),
            GazelleDeductionRelease::CombinedOffsetUnverifiable => parsed_log.combined_rw_offset.is_some(),
            GazelleDeductionRelease::CouldNotVerifyMissingOffsetSamples => parsed_log.ripper == Ripper::EAC && parsed_log.fill_silence == Quartet::Unknown,
            GazelleDeductionRelease::CouldNotVerifySilentBlocks => parsed_log.delete_silence == Quartet::Unknown, 
            GazelleDeductionRelease::CouldNotVerifyNullSamples => parsed_log.use_null_samples == Quartet::Unknown,
            GazelleDeductionRelease::CouldNotVerifyGapHandling => parsed_log.gap_handling == Gap::Unknown,
            GazelleDeductionRelease::CouldNotVerifyId3 => parsed_log.id3_enabled == Quartet::Unknown,
            GazelleDeductionRelease::CouldNotVerifyAlbumGain => false, // TODO: XLD specific prop, does not affect scoring
            GazelleDeductionRelease::RippedWithCompressionOffset => false, // TODO: EAC specific prop, does not affect scoring
            GazelleDeductionRelease::RangeRip => {
                if parsed_log.ripper != Ripper::EAC {
                    return false;
                }
                
                for track in parsed_log.tracks.iter() {
                    if track.is_range {
                        return true;
                    }
                }
                false
            },
            GazelleDeductionRelease::TestAndCopyNotUsed => {
                for track in &parsed_log.tracks {
                    if track.aborted {
                        return false;
                    }
                }
                parsed_log.test_and_copy != Quartet::True
            },
            // They don't account for XLD not being secure
            GazelleDeductionRelease::RipModeNotSecure => parsed_log.ripper == Ripper::EAC && parsed_log.read_mode != ReadMode::Secure,
            GazelleDeductionRelease::NotPressedCd => parsed_log.ripper != Ripper::EAC && parsed_log.media_type != MediaType::Pressed,
            GazelleDeductionRelease::LowMaxRetryCount => false, // TODO: XLD specific prop, does not affect scoring
            GazelleDeductionRelease::AccurateStreamNotUtilized => parsed_log.accurate_stream == Quartet::False,
            GazelleDeductionRelease::UsedC2 => parsed_log.use_c2 == Quartet::True,
            GazelleDeductionRelease::DoesNotFillMissingOffsetSamples => parsed_log.fill_silence == Quartet::False,
            GazelleDeductionRelease::LeadingTrailingBlocksDeleted => parsed_log.delete_silence == Quartet::True,
            GazelleDeductionRelease::NullSamplesNotUsed => parsed_log.use_null_samples == Quartet::False,
            GazelleDeductionRelease::NormalizationUsed => parsed_log.normalize == Quartet::True,
            GazelleDeductionRelease::IncorrectGapHandling => parsed_log.gap_handling != Gap::Unknown && parsed_log.gap_handling != Gap::Append && parsed_log.gap_handling != Gap::AppendNoHtoa,
            GazelleDeductionRelease::Id3OnFlac => {
                let id3_valid_encoder = parsed_log.audio_encoder.iter().any(|encoder| encoder.contains("mp3") || encoder.contains("lame"));
                parsed_log.id3_enabled == Quartet::True && !id3_valid_encoder
            },
            GazelleDeductionRelease::NotSecureCrcMismatch => {
                if parsed_log.read_mode == ReadMode::Secure || (parsed_log.ripper == Ripper::XLD && parsed_log.read_mode == ReadMode::Paranoid) {
                    return false;
                }

                parsed_log.tracks.iter().any(|t| t.test_and_copy.integrity == Integrity::Mismatch)
            },
            GazelleDeductionRelease::NotSecureNoTC => {
                if parsed_log.read_mode == ReadMode::Secure || (parsed_log.ripper == Ripper::XLD && parsed_log.read_mode == ReadMode::Paranoid) {
                    return false;
                }

                parsed_log.test_and_copy != Quartet::True && parsed_log.tracks.iter().all(|t| !t.aborted)
            },
        }
    }

    fn check_track(parsed_log: &ParsedLog, track_entry: &TrackEntry, data: GazelleDeductionTrack) -> bool {
        match data {
            GazelleDeductionTrack::CouldNotVerifyFilenameTooLong => {
                if !track_entry.is_range && !track_entry.filenames.is_empty() {
                    let filename = track_entry.filenames.first().unwrap();
                    if !OPS_EXTENSION_ALLOWLIST.is_match(filename) {
                        return false;
                    }
                    return parsed_log.ripper == Ripper::EAC && filename.len() >= 243;
                }
                false
            },
            GazelleDeductionTrack::CouldNotVerifyFilenameOrExt => {
                if track_entry.filenames.is_empty() {
                    return true;
                }
                if track_entry.is_range {
                    return false;
                }
                let filename = track_entry.filenames.first().unwrap();
                !OPS_EXTENSION_ALLOWLIST.is_match(filename) && parsed_log.ripper == Ripper::EAC && filename.len() < 243
            },
            // TODO: XLD specific, would probably need to make the count field optional
            GazelleDeductionTrack::CouldNotVerifyReadErrors => false,
            GazelleDeductionTrack::CouldNotVerifySkippedErrors => false,
            GazelleDeductionTrack::CouldNotVerifyEdgeJitterErrors => false,
            GazelleDeductionTrack::CouldNotVerifyAtomJitterErrors => false,
            GazelleDeductionTrack::CouldNotVerifyJitterErrors => false,
            GazelleDeductionTrack::CouldNotVerifyRetrySectorCount => false,
            GazelleDeductionTrack::CouldNotVerifyDamagedSectorCount => false,
            GazelleDeductionTrack::CouldNotVerifyDriftErrors => false,
            GazelleDeductionTrack::CouldNotVerifyDroppedBytesErrors => false,
            GazelleDeductionTrack::CouldNotVerifyDuplicatedBytesErrors => false,
            GazelleDeductionTrack::CouldNotVerifyInconsistentErrorSectors => false,
            GazelleDeductionTrack::SusPositionsFound => (parsed_log.ripper == Ripper::EAC && track_entry.errors.read.count > 0) || (parsed_log.ripper == Ripper::XLD && track_entry.errors.inconsistent_err_sectors.count > 0),
            GazelleDeductionTrack::TimingProblemsFound => parsed_log.ripper == Ripper::EAC && track_entry.errors.jitter_generic.count > 0,
            GazelleDeductionTrack::MissingSamplesFound => track_entry.errors.missing_samples.count > 0,
            GazelleDeductionTrack::CopyAborted => track_entry.aborted,
            GazelleDeductionTrack::CrcMismatch => track_entry.test_and_copy.integrity == Integrity::Mismatch,
            GazelleDeductionTrack::ReadErrors(_) => parsed_log.ripper == Ripper::XLD && track_entry.errors.read.count > 0,
            GazelleDeductionTrack::SkippedErrors(_) => parsed_log.ripper == Ripper::XLD && track_entry.errors.skip.count > 0,
            GazelleDeductionTrack::DamagedSectors(_) => parsed_log.ripper == Ripper::XLD && track_entry.errors.damaged_sectors.count > 0,
            GazelleDeductionTrack::InconsistenciesInErrorSectors(_) => parsed_log.ripper == Ripper::XLD && track_entry.errors.inconsistent_err_sectors.count > 0,
        }
    }
}

impl GazelleDeduction for GazelleDeductionFail {
    fn deduct(&self, _parsed_log: &ParsedLog) -> EvaluationUnit {
        let deduction_score: u32 = match &self {
            GazelleDeductionFail::UnknownEncoding => 100,
            GazelleDeductionFail::UnknownRipper => 100,
            GazelleDeductionFail::WhipperVersionLowerLimit => 100,
            GazelleDeductionFail::CouldNotParseWhipper => 100,
        };
        EvaluationUnit::new_from_u32(deduction_score, self.get_deduction_data())
    }
}

impl GazelleDeduction for GazelleDeductionRelease {
    fn deduct(&self, parsed_log: &ParsedLog) -> EvaluationUnit {
        let deduction_score: u32 = match &self {
            GazelleDeductionRelease::VirtualDrive => 20,
            GazelleDeductionRelease::NullDrive => 20,
            GazelleDeductionRelease::IncorrectReadOffset => 5,
            GazelleDeductionRelease::DriveNotFoundDb => if parsed_log.read_offset == Some(0) {5} else {0},
            GazelleDeductionRelease::DefeatAudioCacheDisabled => 10,
            GazelleDeductionRelease::EacVersionOld => 30,
            GazelleDeductionRelease::XldNoChecksum => 15,
            GazelleDeductionRelease::Mp3Log => 100,
            GazelleDeductionRelease::CouldNotVerifyDrive => 1,
            GazelleDeductionRelease::CouldNotVerifyMedia => 1,
            GazelleDeductionRelease::CouldNotVerifyReadMode => 1,
            GazelleDeductionRelease::CouldNotVerifyMaxRetry => 0,
            GazelleDeductionRelease::CouldNotVerifyAccurateStream => 20,
            GazelleDeductionRelease::CouldNotVerifyDefeatAudioCache => 1,
            GazelleDeductionRelease::CouldNotVerifyC2 => 1,
            GazelleDeductionRelease::CouldNotVerifyReadOffset => 1,
            GazelleDeductionRelease::CombinedOffsetUnverifiable => 4,
            GazelleDeductionRelease::CouldNotVerifyMissingOffsetSamples => 1,
            GazelleDeductionRelease::CouldNotVerifySilentBlocks => 1,
            GazelleDeductionRelease::CouldNotVerifyNullSamples => 0,
            GazelleDeductionRelease::CouldNotVerifyGapHandling => 10,
            GazelleDeductionRelease::CouldNotVerifyId3 => 1,
            GazelleDeductionRelease::CouldNotVerifyAlbumGain => 0,
            GazelleDeductionRelease::RippedWithCompressionOffset => 0,
            GazelleDeductionRelease::RangeRip => 30,
            GazelleDeductionRelease::TestAndCopyNotUsed => 10,
            GazelleDeductionRelease::RipModeNotSecure => 20,
            GazelleDeductionRelease::NotPressedCd => 0,
            GazelleDeductionRelease::LowMaxRetryCount => 0,
            GazelleDeductionRelease::AccurateStreamNotUtilized => 20,
            GazelleDeductionRelease::UsedC2 => 10,
            GazelleDeductionRelease::DoesNotFillMissingOffsetSamples => 5,
            GazelleDeductionRelease::LeadingTrailingBlocksDeleted => 5,
            GazelleDeductionRelease::NullSamplesNotUsed => 5,
            GazelleDeductionRelease::NormalizationUsed => 100,
            GazelleDeductionRelease::IncorrectGapHandling => 10,
            GazelleDeductionRelease::Id3OnFlac => 1,
            GazelleDeductionRelease::NotSecureCrcMismatch => 20,
            GazelleDeductionRelease::NotSecureNoTC => 40,
        };
        EvaluationUnit::new_from_u32(deduction_score, self.get_deduction_data())
    }
}

impl GazelleDeduction for GazelleDeductionTrack {
    fn deduct(&self, _parsed_log: &ParsedLog) -> EvaluationUnit {
        let deduction_score: u32 = match &self {
            GazelleDeductionTrack::CouldNotVerifyFilenameTooLong => 0,
            GazelleDeductionTrack::CouldNotVerifyFilenameOrExt => 1,
            GazelleDeductionTrack::CouldNotVerifyReadErrors => 0,
            GazelleDeductionTrack::CouldNotVerifySkippedErrors => 0,
            GazelleDeductionTrack::CouldNotVerifyEdgeJitterErrors => 0,
            GazelleDeductionTrack::CouldNotVerifyAtomJitterErrors => 0,
            GazelleDeductionTrack::CouldNotVerifyJitterErrors => 0,
            GazelleDeductionTrack::CouldNotVerifyRetrySectorCount => 0,
            GazelleDeductionTrack::CouldNotVerifyDamagedSectorCount => 0,
            GazelleDeductionTrack::CouldNotVerifyDriftErrors => 0,
            GazelleDeductionTrack::CouldNotVerifyDroppedBytesErrors => 0,
            GazelleDeductionTrack::CouldNotVerifyDuplicatedBytesErrors => 0,
            GazelleDeductionTrack::CouldNotVerifyInconsistentErrorSectors => 0,
            GazelleDeductionTrack::SusPositionsFound => 20,
            GazelleDeductionTrack::TimingProblemsFound => 20,
            GazelleDeductionTrack::MissingSamplesFound => 20,
            GazelleDeductionTrack::CopyAborted => 100,
            GazelleDeductionTrack::CrcMismatch => 30,
            GazelleDeductionTrack::ReadErrors(read_error_count) => min(*read_error_count, 10),
            GazelleDeductionTrack::SkippedErrors(skip_error_count) => min(*skip_error_count, 10),
            GazelleDeductionTrack::InconsistenciesInErrorSectors(inconsistency_count) => min(*inconsistency_count, 10),
            GazelleDeductionTrack::DamagedSectors(damaged_sector_count) => min(*damaged_sector_count, 10),
        };
        EvaluationUnit::new_from_u32(deduction_score, self.get_deduction_data())
    }
}

impl Evaluator for OpsEvaluator {
    fn evaluate_combined(&mut self, plc: &ParsedLogCombined) -> EvaluationCombined {
        let mut evaluations: Vec<Evaluation> = Vec::new();
        let mut track_deduction_map: HashMap<usize, Vec<EvaluationUnit>> = HashMap::new();
        let mut release_deduction_set: HashSet<EvaluationUnit> = HashSet::new();

        // This is wrong on so many levels but it's how OPS implements it
        // TODO: This probably isn't efficient, should drop storing entire deductions in the map and only keep the scores
        for log in plc.parsed_logs.iter() {
            let evaluation = self.evaluate(log);
            let mut log_track_deduction_map: HashMap<usize, Vec<EvaluationUnit>> = HashMap::new();

            for deduction in evaluation.evaluation_units.iter() {
                match deduction.data.scope {
                    EvaluationUnitScope::Release => {
                        release_deduction_set.insert(deduction.clone());
                    },
                    EvaluationUnitScope::Track(t) => {
                        log_track_deduction_map
                            .entry(t.unwrap() as usize)
                            .or_default()
                            .push(deduction.clone());
                    },
                }
            }

            // Overwrite the main map
            let (start_track, total_tracks): (usize, usize) = match (!log.toc.raw.entries.is_empty(), !log.tracks.is_empty()) {
                (true, true) => if log.tracks.first().unwrap().is_range && log.ripper == Ripper::EAC { (0, 0) } else { (1, log.toc.raw.entries.len()) },
                // Impossible to know total track count with full certainty
                (false, true) => if log.tracks.first().unwrap().is_range && log.ripper == Ripper::EAC { (0, 0) } else { (1, log.tracks.last().unwrap().num as usize) },
                // This should never happen, skip if it does
                (_, false) => (1, 0),
            };
            let tracks_ripped = log.tracks.par_iter().map(|t| t.num as usize).collect::<HashSet<_>>();
            for t in start_track..=total_tracks {
                if tracks_ripped.contains(&t) {
                    track_deduction_map.insert(t, log_track_deduction_map.remove(&t).unwrap_or_default().to_owned());
                }
            }

            evaluations.push(evaluation);
        }

        // Remove mp3 deduction if combined (apparently it doesn't matter if all of them are mp3)
        if plc.parsed_logs.len() > 1 {
            let mp3 = GazelleDeductionRelease::Mp3Log.deduct(plc.parsed_logs.first().unwrap());
            release_deduction_set.remove(&mp3);
        }

        // OPS evaluator seems to have this unholy chimera of a deduction that's neither release-level nor track-level
        // Should not be used in scoring unless the last log has it
        let nscm = GazelleDeductionRelease::NotSecureCrcMismatch.deduct(plc.parsed_logs.last().unwrap());
        if release_deduction_set.contains(&nscm) && !evaluations.last().unwrap().evaluation_units.contains(&nscm) {
            release_deduction_set.remove(&nscm);
        }

        // Deduction aggregates
        let release_deduction_score: i32 = release_deduction_set
                                            .into_iter()
                                            .map(|x| x.unit_score.parse::<i32>().unwrap())
                                            .sum();
        let track_deduction_score: i32 = track_deduction_map
                                            .values()
                                            .flat_map(|ds| ds.iter().map(|d| d.unit_score.parse::<i32>().unwrap_or_default()))
                                            .sum();
        
        let combined_score: i32 = 100 - release_deduction_score - track_deduction_score;
        EvaluationCombined::new(EvaluatorType::OPS, combined_score.to_string(), evaluations)
    }

    fn evaluate(&mut self, parsed_log: &ParsedLog) -> Evaluation {
        let mut score: i32 = 100_i32;
        let mut deductions: Vec<EvaluationUnit> = Vec::new();

        for gazelle_deduction_fail in GazelleDeductionFail::iter() {
            if OpsEvaluator::check_fail(parsed_log, gazelle_deduction_fail) {
                let deduction = gazelle_deduction_fail.deduct(parsed_log);
                score -= from_str::<i32>(deduction.unit_score.as_str()).unwrap();
                deductions.push(deduction);
                return Evaluation::new(score.to_string(), deductions);
            }
        }

        let mut deductions_release: Vec<EvaluationUnit> = GazelleDeductionRelease::iter()
            .par_bridge()
            .filter_map(|gazelle_deduction_release| {
                if OpsEvaluator::check_release(parsed_log, gazelle_deduction_release) {
                    let deduction = gazelle_deduction_release.deduct(parsed_log);
                    Some(deduction)
                } else {
                    None
                }
            })
            .collect();
        let score_release: i32 = deductions_release
            .iter()
            .map(|deduction| from_str::<i32>(deduction.unit_score.as_str()).unwrap())
            .sum();

        deductions.append(&mut deductions_release);
        score -= score_release;

        let mut deductions_track: Vec<_> = parsed_log
            .tracks
            .par_iter()
            .flat_map(|track| {
                GazelleDeductionTrack::iter()
                    .filter_map(|gazelle_deduction_track| {
                        let gazelle_deduction_track_variant: GazelleDeductionTrack = match gazelle_deduction_track {
                            GazelleDeductionTrack::ReadErrors(_) => GazelleDeductionTrack::ReadErrors(track.errors.read.count),
                            GazelleDeductionTrack::SkippedErrors(_) => GazelleDeductionTrack::SkippedErrors(track.errors.skip.count),
                            GazelleDeductionTrack::DamagedSectors(_) => GazelleDeductionTrack::DamagedSectors(track.errors.damaged_sectors.count),
                            GazelleDeductionTrack::InconsistenciesInErrorSectors(_) => GazelleDeductionTrack::InconsistenciesInErrorSectors(track.errors.inconsistent_err_sectors.count),
                            other => other,
                        };
                        if OpsEvaluator::check_track(parsed_log, track, gazelle_deduction_track_variant) {
                            let mut deduction = gazelle_deduction_track_variant.deduct(parsed_log);
                            deduction.data.scope = EvaluationUnitScope::Track(Some(track.num)); // TODO: Special considerations for HTOA (?)
                            Some(deduction)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        let score_track: i32 = deductions_track
            .iter()
            .map(|deduction| from_str::<i32>(deduction.unit_score.as_str()).unwrap())
            .sum();

        deductions.append(&mut deductions_track);
        score -=score_track;
        
        Evaluation::new(score.to_string(), deductions)
    }
}