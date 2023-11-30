use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;
use strum::IntoEnumIterator;

use crate::{parser::{ParsedLogCombined, ParsedLog}, extract::{Quartet, ReadMode, Gap}};

use super::{DeductionCategory, DeductionField, DeductionData, Evaluator, Deduction, EvaluationCombined, Evaluation, EvaluatorType};

pub trait CambiaDeductionData {
    fn get_deduction_data(&self) -> DeductionData;
}

#[derive(Serialize, Deserialize, EnumIter, Clone, Copy)]
pub enum CambiaDeduction {
    VirtualDrive,
    IncorrectReadOffset,
    DefeatAudioCacheDisabled,
    LossyLog,
    TestAndCopyNotUsed,
    RipModeNotAccurate,
    AccurateStreamNotUtilized,
    UsedC2,
    DoesNotFillMissingOffsetSamples,
    LeadingTrailingBlocksDeleted,
    NullSamplesNotUsed,
    NormalizationUsed,
    IncorrectGapHandling,
}

// TODO: Will possibly require 4 levels to indicate the really bad settings
#[derive(Serialize, Deserialize, strum_macros::Display)]
pub enum CambiaGrade {
    Good,
    NotIdeal,
    Bad,
}

impl CambiaDeductionData for CambiaDeduction {
    fn get_deduction_data(&self) -> DeductionData {
        match &self {
            CambiaDeduction::VirtualDrive => DeductionData::new(
                DeductionCategory::Release,
                DeductionField::Drive,
                "Virtual drives are unlikely to have correct offset, and can be used for deceit"
            ),
            CambiaDeduction::IncorrectReadOffset => DeductionData::new(
                DeductionCategory::Release,
                DeductionField::Offset,
                "Incorrect read offset for drive"
            ),
            CambiaDeduction::DefeatAudioCacheDisabled => DeductionData::new(
                DeductionCategory::Release,
                DeductionField::Cache,
                "Audio cache should be defeated/disabled"
            ),
            CambiaDeduction::LossyLog => DeductionData::new(
                DeductionCategory::Release,
                DeductionField::Encoder,
                "Lossy codecs are not meant for archival"
            ),
            CambiaDeduction::TestAndCopyNotUsed => DeductionData::new(
                DeductionCategory::Release,
                DeductionField::TestAndCopy,
                "Test and copy was not used"
            ),
            CambiaDeduction::RipModeNotAccurate => DeductionData::new(
                DeductionCategory::Release,
                DeductionField::ReadMode,
                "Rip mode should be secure/paranoid"
            ),
            CambiaDeduction::AccurateStreamNotUtilized => DeductionData::new(
                DeductionCategory::Release,
                DeductionField::AccurateStream,
                "Accurate stream should be utilised"
            ),
            CambiaDeduction::UsedC2 => DeductionData::new(
                DeductionCategory::Release,
                DeductionField::C2,
                "C2 pointers rely on guesswork and can be used as a DRM tactic"
            ),
            CambiaDeduction::DoesNotFillMissingOffsetSamples => DeductionData::new(
                DeductionCategory::Release,
                DeductionField::Samples,
                "Does not fill up missing offset samples with silence"
            ),
            CambiaDeduction::LeadingTrailingBlocksDeleted => DeductionData::new(
                DeductionCategory::Release,
                DeductionField::SilentBlocks,
                "Deletes leading and trailing silent blocks"
            ),
            CambiaDeduction::NullSamplesNotUsed => DeductionData::new(
                DeductionCategory::Release,
                DeductionField::NullSamples,
                "Null samples should be used in CRC calculations"
            ),
            CambiaDeduction::NormalizationUsed => DeductionData::new(
                DeductionCategory::Release,
                DeductionField::Normalization,
                "Normalization during ripping irreversibly alters data and should never be used in this context"
            ),
            CambiaDeduction::IncorrectGapHandling => DeductionData::new(
                DeductionCategory::Release,
                DeductionField::Gap,
                "Gaps should be appended"
            ),
        }
    }
}

pub struct CambiaEvaluator;

impl CambiaDeduction {
    // TODO: Requires oversight
    fn deduct(&self, parsed_log: &ParsedLog) -> Deduction {
        let grade: CambiaGrade = match &self {
            // FIXME: Virtual drive detection
            CambiaDeduction::VirtualDrive => CambiaGrade::Good,
            // FIXME: Read offset
            CambiaDeduction::IncorrectReadOffset => CambiaGrade::Good,
            CambiaDeduction::DefeatAudioCacheDisabled => if parsed_log.defeat_audio_cache == Quartet::False { CambiaGrade::Bad } else { CambiaGrade::Good },
            // FIXME: Lossy rip
            CambiaDeduction::LossyLog => CambiaGrade::Good,
            CambiaDeduction::TestAndCopyNotUsed => if parsed_log.test_and_copy == Quartet::False { CambiaGrade::Bad } else { CambiaGrade::Good },
            CambiaDeduction::RipModeNotAccurate => if parsed_log.read_mode != ReadMode::Secure && parsed_log.read_mode != ReadMode::Paranoid { CambiaGrade::Bad } else { CambiaGrade::Good },
            CambiaDeduction::AccurateStreamNotUtilized => if parsed_log.accurate_stream != Quartet::True { CambiaGrade::Bad } else { CambiaGrade::Good },
            CambiaDeduction::UsedC2 => if parsed_log.use_c2 == Quartet::True { CambiaGrade::Bad } else { CambiaGrade::Good },
            CambiaDeduction::DoesNotFillMissingOffsetSamples => if parsed_log.fill_silence == Quartet::False { CambiaGrade::Bad } else { CambiaGrade::Good },
            CambiaDeduction::LeadingTrailingBlocksDeleted => if parsed_log.delete_silence == Quartet::True { CambiaGrade::Bad } else { CambiaGrade::Good },
            CambiaDeduction::NullSamplesNotUsed => if parsed_log.use_null_samples == Quartet::False { CambiaGrade::Bad } else { CambiaGrade::Good },
            CambiaDeduction::NormalizationUsed => if parsed_log.normalize == Quartet::True || parsed_log.normalize == Quartet::Unknown { CambiaGrade::Bad } else { CambiaGrade::Good },
            CambiaDeduction::IncorrectGapHandling => if parsed_log.gap_handling != Gap::Append && parsed_log.gap_handling != Gap::AppendNoHtoa { CambiaGrade::Bad } else { CambiaGrade::Good },
        };

        Deduction::new(grade.to_string(), self.get_deduction_data())
    }
}

impl CambiaEvaluator {
    pub fn new() -> Self {
        Self
    }
}

impl Default for CambiaEvaluator {
    fn default() -> Self {
        Self::new()
    }
}

impl Evaluator for CambiaEvaluator {
    fn evaluate_combined(&mut self, parsed_logs: &ParsedLogCombined) -> EvaluationCombined {
        let mut evaluations: Vec<Evaluation> = Vec::new();

        for parsed_log in parsed_logs.parsed_logs.iter() {
            evaluations.push(self.evaluate(parsed_log));
        }

        // TODO: Use mode for aggregate (?) or use some other weighting
        /*
        TODO: A smart approach would be to pick max from a cluster of same discID
        ^Will NOT work for releases that have different disc content with same discID (i.e. vocals and inst on same release)
         */
        EvaluationCombined::new(EvaluatorType::Cambia, String::from("N/A"), evaluations)
    }

    fn evaluate(&mut self, parsed_log: &ParsedLog) -> Evaluation {
        let mut deductions: Vec<Deduction> = Vec::new();

        for cambia_deduction in CambiaDeduction::iter() {
            deductions.push(cambia_deduction.deduct(parsed_log))
        }

        // TODO: For a score to be meaningful, worst offenders need to be distinguishable from the "worse"
        Evaluation::new(String::from("N/A"), deductions)
    }
}