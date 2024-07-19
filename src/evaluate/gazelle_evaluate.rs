use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use crate::parser::ParsedLog;

use super::{EvaluationUnitCategory, EvaluationUnitField, EvaluationUnitData, Evaluator, EvaluationUnit};

#[cfg(feature = "red_ev")]
pub mod red_evaluate;
#[cfg(feature = "ops_ev")]
pub mod ops_evaluate;

pub trait GazelleEvaluator: Evaluator {
    fn deduct(&mut self, data: dyn GazelleDeductionData) -> EvaluationUnit;
}

pub trait GazelleDeductionData {
    fn get_deduction_data(&self) -> EvaluationUnitData;
}

pub trait GazelleDeduction {
    // TODO: Do we need to send the ParsedLog for a single use case (drive not in DB check)?
    fn deduct(&self, parsed_log: &ParsedLog) -> EvaluationUnit;
}

#[derive(Serialize, Deserialize, EnumIter, Clone, Copy)]
pub enum GazelleDeductionFail {
    UnknownEncoding,
    UnknownRipper,
    WhipperVersionLowerLimit,
    CouldNotParseWhipper,
}

#[derive(Serialize, Deserialize, EnumIter, Clone, Copy)]
pub enum GazelleDeductionRelease {
    VirtualDrive,
    NullDrive,
    IncorrectReadOffset,
    DriveNotFoundDb,
    DefeatAudioCacheDisabled,
    EacVersionOld,
    XldNoChecksum,
    Mp3Log,
    CouldNotVerifyDrive,
    CouldNotVerifyMedia,
    CouldNotVerifyReadMode,
    CouldNotVerifyMaxRetry,
    CouldNotVerifyAccurateStream,
    CouldNotVerifyDefeatAudioCache,
    CouldNotVerifyC2,
    CouldNotVerifyReadOffset,
    CouldNotVerifyMissingOffsetSamples,
    CouldNotVerifySilentBlocks,
    CouldNotVerifyNullSamples,
    CouldNotVerifyGapHandling,
    CouldNotVerifyId3,
    CouldNotVerifyAlbumGain,
    CombinedOffsetUnverifiable,
    RippedWithCompressionOffset,
    RangeRip,
    TestAndCopyNotUsed,
    RipModeNotSecure,
    NotPressedCd,
    LowMaxRetryCount,
    AccurateStreamNotUtilized,
    UsedC2,
    DoesNotFillMissingOffsetSamples,
    LeadingTrailingBlocksDeleted,
    NullSamplesNotUsed,
    NormalizationUsed,
    IncorrectGapHandling,
    Id3OnFlac,
    NotSecureCrcMismatch,
    NotSecureNoTC,
}

#[derive(Serialize, Deserialize, EnumIter, Clone, Copy)]
pub enum GazelleDeductionTrack {
    CouldNotVerifyFilenameTooLong,
    CouldNotVerifyFilenameOrExt,
    CouldNotVerifyReadErrors,
    CouldNotVerifySkippedErrors,
    CouldNotVerifyEdgeJitterErrors,
    CouldNotVerifyAtomJitterErrors,
    CouldNotVerifyJitterErrors,
    CouldNotVerifyRetrySectorCount,
    CouldNotVerifyDamagedSectorCount,
    CouldNotVerifyDriftErrors,
    CouldNotVerifyDroppedBytesErrors,
    CouldNotVerifyDuplicatedBytesErrors,
    CouldNotVerifyInconsistentErrorSectors,
    SusPositionsFound,
    TimingProblemsFound,
    MissingSamplesFound,
    CopyAborted,
    CrcMismatch,
    ReadErrors(u32),
    SkippedErrors(u32),
    InconsistenciesInErrorSectors(u32),
    DamagedSectors(u32),
}

impl GazelleDeductionData for GazelleDeductionFail {
    fn get_deduction_data(&self) -> EvaluationUnitData {
        match &self {
            GazelleDeductionFail::UnknownEncoding => EvaluationUnitData::new(
                EvaluationUnitCategory::Release,
                EvaluationUnitField::Encoding,
                "Could not detect log encoding, log is corrupt"
            ),
            GazelleDeductionFail::UnknownRipper => EvaluationUnitData::new(
                EvaluationUnitCategory::Release,
                EvaluationUnitField::Ripper,
                "Unknown log file, could not determine ripper"
            ),
            GazelleDeductionFail::WhipperVersionLowerLimit => EvaluationUnitData::new(
                EvaluationUnitCategory::Release,
                EvaluationUnitField::RipperVersion,
                "Logs must be produced by whipper 0.7.3+"
            ),
            GazelleDeductionFail::CouldNotParseWhipper => EvaluationUnitData::new(
                EvaluationUnitCategory::Release,
                EvaluationUnitField::Ripper,
                "Could not parse whipper log"
            ),
        }
    }
}

impl GazelleDeductionData for GazelleDeductionRelease {
    fn get_deduction_data(&self) -> EvaluationUnitData {
        match &self {
            GazelleDeductionRelease::VirtualDrive => EvaluationUnitData::new(
                EvaluationUnitCategory::Release,
                EvaluationUnitField::Drive,
                "Virtual drive used"
            ),
            GazelleDeductionRelease::NullDrive => EvaluationUnitData::new(
                EvaluationUnitCategory::Release,
                EvaluationUnitField::Drive,
                "Null drive used"
            ),
            GazelleDeductionRelease::IncorrectReadOffset => EvaluationUnitData::new(
                EvaluationUnitCategory::Release,
                EvaluationUnitField::Offset,
                "Incorrect read offset for drive"
            ),
            GazelleDeductionRelease::DriveNotFoundDb => EvaluationUnitData::new(
                EvaluationUnitCategory::Release,
                EvaluationUnitField::Drive,
                "The drive was not found in the database"
            ),
            GazelleDeductionRelease::DefeatAudioCacheDisabled => EvaluationUnitData::new(
                EvaluationUnitCategory::Release,
                EvaluationUnitField::Cache,
                "\"Defeat audio cache\" should be Yes/true"
            ),
            GazelleDeductionRelease::EacVersionOld => EvaluationUnitData::new(
                EvaluationUnitCategory::Release,
                EvaluationUnitField::RipperVersion,
                "EAC version older than 0.99"
            ),
            GazelleDeductionRelease::XldNoChecksum => EvaluationUnitData::new(
                EvaluationUnitCategory::Release,
                EvaluationUnitField::Checksum,
                "No checksum with XLD 20121222 or newer"
            ),
            GazelleDeductionRelease::Mp3Log => EvaluationUnitData::new(
                EvaluationUnitCategory::Release,
                EvaluationUnitField::Encoder,
                "Invalid Log (MP3)"
            ),
            GazelleDeductionRelease::CouldNotVerifyDrive => EvaluationUnitData::new(
                EvaluationUnitCategory::Release,
                EvaluationUnitField::Drive,
                "Could not verify used drive"
            ),
            GazelleDeductionRelease::CouldNotVerifyMedia => EvaluationUnitData::new(
                EvaluationUnitCategory::Release,
                EvaluationUnitField::MediaType,
                "Could not verify media type"
            ),
            GazelleDeductionRelease::CouldNotVerifyReadMode => EvaluationUnitData::new(
                EvaluationUnitCategory::Release,
                EvaluationUnitField::ReadMode,
                "Could not verify read mode"
            ),
            GazelleDeductionRelease::CouldNotVerifyMaxRetry => EvaluationUnitData::new(
                EvaluationUnitCategory::Release,
                EvaluationUnitField::MaxRetryCount,
                "Could not verify max retry count"
            ),
            GazelleDeductionRelease::CouldNotVerifyAccurateStream => EvaluationUnitData::new(
                EvaluationUnitCategory::Release,
                EvaluationUnitField::AccurateStream,
                "Could not verify accurate stream"
            ),
            GazelleDeductionRelease::CouldNotVerifyDefeatAudioCache => EvaluationUnitData::new(
                EvaluationUnitCategory::Release,
                EvaluationUnitField::Cache,
                "Could not verify defeat audio cache"
            ),
            GazelleDeductionRelease::CouldNotVerifyC2 => EvaluationUnitData::new(
                EvaluationUnitCategory::Release,
                EvaluationUnitField::C2,
                "Could not verify C2 pointers"
            ),
            GazelleDeductionRelease::CouldNotVerifyReadOffset => EvaluationUnitData::new(
                EvaluationUnitCategory::Release,
                EvaluationUnitField::Offset,
                "Could not verify read offset"
            ),
            GazelleDeductionRelease::CombinedOffsetUnverifiable => EvaluationUnitData::new(
                EvaluationUnitCategory::Release,
                EvaluationUnitField::Offset,
                "Combined read/write offset cannot be verified"
            ),
            GazelleDeductionRelease::CouldNotVerifyMissingOffsetSamples => EvaluationUnitData::new(
                EvaluationUnitCategory::Release,
                EvaluationUnitField::Samples,
                "Could not verify missing offset samples"
            ),
            GazelleDeductionRelease::CouldNotVerifySilentBlocks => EvaluationUnitData::new(
                EvaluationUnitCategory::Release,
                EvaluationUnitField::SilentBlocks,
                "Could not verify silent blocks"
            ),
            GazelleDeductionRelease::CouldNotVerifyNullSamples => EvaluationUnitData::new(
                EvaluationUnitCategory::Release,
                EvaluationUnitField::NullSamples,
                "Could not verify null samples"
            ),
            GazelleDeductionRelease::CouldNotVerifyGapHandling => EvaluationUnitData::new(
                EvaluationUnitCategory::Release,
                EvaluationUnitField::Gap,
                "Could not verify gap handling"
            ),
            GazelleDeductionRelease::CouldNotVerifyId3 => EvaluationUnitData::new(
                EvaluationUnitCategory::Release,
                EvaluationUnitField::Tag,
                "Could not verify id3 tag setting"
            ),
            GazelleDeductionRelease::CouldNotVerifyAlbumGain => EvaluationUnitData::new(
                EvaluationUnitCategory::Release,
                EvaluationUnitField::Gain,
                "Could not verify album gain"
            ),
            GazelleDeductionRelease::RippedWithCompressionOffset => EvaluationUnitData::new(
                EvaluationUnitCategory::Release,
                EvaluationUnitField::Offset,
                "Ripped with compression offset"
            ),
            GazelleDeductionRelease::RangeRip => EvaluationUnitData::new(
                EvaluationUnitCategory::Release,
                EvaluationUnitField::RangeSplit,
                "Range rip detected"
            ),
            GazelleDeductionRelease::TestAndCopyNotUsed => EvaluationUnitData::new(
                EvaluationUnitCategory::Release,
                EvaluationUnitField::TestAndCopy,
                "Test and copy was not used"
            ),
            GazelleDeductionRelease::RipModeNotSecure => EvaluationUnitData::new(
                EvaluationUnitCategory::Release,
                EvaluationUnitField::ReadMode,
                "Rip mode not secure"
            ),
            GazelleDeductionRelease::NotPressedCd => EvaluationUnitData::new(
                EvaluationUnitCategory::Release,
                EvaluationUnitField::MediaType,
                "Not a pressed cd"
            ),
            GazelleDeductionRelease::LowMaxRetryCount => EvaluationUnitData::new(
                EvaluationUnitCategory::Release,
                EvaluationUnitField::MaxRetryCount,
                "Low \"max retry count\" (potentially bad setting)"
            ),
            GazelleDeductionRelease::AccurateStreamNotUtilized => EvaluationUnitData::new(
                EvaluationUnitCategory::Release,
                EvaluationUnitField::AccurateStream,
                "\"Utilize accurate stream\" should be yes"
            ),
            GazelleDeductionRelease::UsedC2 => EvaluationUnitData::new(
                EvaluationUnitCategory::Release,
                EvaluationUnitField::C2,
                "C2 pointers were used"
            ),
            GazelleDeductionRelease::DoesNotFillMissingOffsetSamples => EvaluationUnitData::new(
                EvaluationUnitCategory::Release,
                EvaluationUnitField::Samples,
                "Does not fill up missing offset samples with silence"
            ),
            GazelleDeductionRelease::LeadingTrailingBlocksDeleted => EvaluationUnitData::new(
                EvaluationUnitCategory::Release,
                EvaluationUnitField::SilentBlocks,
                "Deletes leading and trailing silent blocks"
            ),
            GazelleDeductionRelease::NullSamplesNotUsed => EvaluationUnitData::new(
                EvaluationUnitCategory::Release,
                EvaluationUnitField::NullSamples,
                "Null samples should be used in CRC calculations"
            ),
            GazelleDeductionRelease::NormalizationUsed => EvaluationUnitData::new(
                EvaluationUnitCategory::Release,
                EvaluationUnitField::Normalization,
                "Normalization should be not be active"
            ),
            GazelleDeductionRelease::IncorrectGapHandling => EvaluationUnitData::new(
                EvaluationUnitCategory::Release,
                EvaluationUnitField::Gap,
                "Incorrect gap handling"
            ),
            GazelleDeductionRelease::NotSecureCrcMismatch => EvaluationUnitData::new(
                EvaluationUnitCategory::Release,
                EvaluationUnitField::TestAndCopy,
                "Rip was not done in Secure mode, and experienced CRC mismatches"
            ),
            GazelleDeductionRelease::NotSecureNoTC => EvaluationUnitData::new(
                EvaluationUnitCategory::Release,
                EvaluationUnitField::TestAndCopy,
                "Rip was not done in Secure mode, and T+C was not used - as a result, we cannot verify the authenticity of the rip"
            ),
            GazelleDeductionRelease::Id3OnFlac => EvaluationUnitData::new(
                EvaluationUnitCategory::Release,
                EvaluationUnitField::Tag,
                "ID3 tags should not be added to FLAC files - they are mainly for MP3 files."
            ),
        }
    }
}

impl GazelleDeductionData for GazelleDeductionTrack {
    fn get_deduction_data(&self) -> EvaluationUnitData {
        match &self {
            GazelleDeductionTrack::CouldNotVerifyFilenameTooLong => EvaluationUnitData::new(
                EvaluationUnitCategory::Track(None),
                EvaluationUnitField::Filename,
                "Could not verify filename, too long"
            ),
            GazelleDeductionTrack::CouldNotVerifyFilenameOrExt => EvaluationUnitData::new(
                EvaluationUnitCategory::Track(None),
                EvaluationUnitField::Filename,
                "Could not verify filename or file extension"
            ),
            GazelleDeductionTrack::CouldNotVerifyReadErrors => EvaluationUnitData::new(
                EvaluationUnitCategory::Track(None),
                EvaluationUnitField::Filename,
                "Could not verify read errors"
            ),
            GazelleDeductionTrack::CouldNotVerifySkippedErrors => EvaluationUnitData::new(
                EvaluationUnitCategory::Track(None),
                EvaluationUnitField::SkipError,
                "Could not verify skipped errors"
            ),
            GazelleDeductionTrack::CouldNotVerifyEdgeJitterErrors => EvaluationUnitData::new(
                EvaluationUnitCategory::Track(None),
                EvaluationUnitField::JitterEdgeError,
                "Could not verify edge jitter errors"
            ),
            GazelleDeductionTrack::CouldNotVerifyAtomJitterErrors => EvaluationUnitData::new(
                EvaluationUnitCategory::Track(None),
                EvaluationUnitField::JitterAtomError,
                "Could not verify atom jitter errors"
            ),
            GazelleDeductionTrack::CouldNotVerifyJitterErrors => EvaluationUnitData::new(
                EvaluationUnitCategory::Track(None),
                EvaluationUnitField::JitterGenericError,
                "Could not verify jitter errors"
            ),
            GazelleDeductionTrack::CouldNotVerifyRetrySectorCount => EvaluationUnitData::new(
                EvaluationUnitCategory::Track(None),
                EvaluationUnitField::MaxRetryCount,
                "Could not verify retry sector count"
            ),
            GazelleDeductionTrack::CouldNotVerifyDamagedSectorCount => EvaluationUnitData::new(
                EvaluationUnitCategory::Track(None),
                EvaluationUnitField::DamagedSector,
                "Could not verify damaged sector count"
            ),
            GazelleDeductionTrack::CouldNotVerifyDriftErrors => EvaluationUnitData::new(
                EvaluationUnitCategory::Track(None),
                EvaluationUnitField::DriftError,
                "Could not verify drift errors"
            ),
            GazelleDeductionTrack::CouldNotVerifyDroppedBytesErrors => EvaluationUnitData::new(
                EvaluationUnitCategory::Track(None),
                EvaluationUnitField::DroppedError,
                "Could not verify dropped bytes errors"
            ),
            GazelleDeductionTrack::CouldNotVerifyDuplicatedBytesErrors => EvaluationUnitData::new(
                EvaluationUnitCategory::Track(None),
                EvaluationUnitField::DuplicatedError,
                "Could not verify duplicated bytes errors"
            ),
            GazelleDeductionTrack::CouldNotVerifyInconsistentErrorSectors => EvaluationUnitData::new(
                EvaluationUnitCategory::Track(None),
                EvaluationUnitField::InconsistentErrorSectors,
                "Could not verify inconsistent error sectors"
            ),
            GazelleDeductionTrack::SusPositionsFound => EvaluationUnitData::new(
                EvaluationUnitCategory::Track(None),
                EvaluationUnitField::ReadError,
                "Suspicious position(s) found"
            ),
            GazelleDeductionTrack::TimingProblemsFound => EvaluationUnitData::new(
                EvaluationUnitCategory::Track(None),
                EvaluationUnitField::JitterGenericError,
                "Timing problem(s) found"
            ),
            GazelleDeductionTrack::MissingSamplesFound => EvaluationUnitData::new(
                EvaluationUnitCategory::Track(None),
                EvaluationUnitField::DamagedSector,
                "Missing sample(s) found"
            ),
            GazelleDeductionTrack::CopyAborted => EvaluationUnitData::new(
                EvaluationUnitCategory::Track(None),
                EvaluationUnitField::Abort,
                "Copy aborted"
            ),
            GazelleDeductionTrack::CrcMismatch => EvaluationUnitData::new(
                EvaluationUnitCategory::Track(None),
                EvaluationUnitField::TestAndCopy,
                "CRC mismatch"
            ),
            GazelleDeductionTrack::ReadErrors(_) => EvaluationUnitData::new(
                EvaluationUnitCategory::Track(None),
                EvaluationUnitField::ReadError,
                "Read error"
            ),
            GazelleDeductionTrack::SkippedErrors(_) => EvaluationUnitData::new(
                EvaluationUnitCategory::Track(None),
                EvaluationUnitField::SkipError,
                "Skipped error"
            ),
            GazelleDeductionTrack::InconsistenciesInErrorSectors(_) => EvaluationUnitData::new(
                EvaluationUnitCategory::Track(None),
                EvaluationUnitField::InconsistentErrorSectors,
                "Inconsistencies in error sectors detected"
            ),
            GazelleDeductionTrack::DamagedSectors(_) => EvaluationUnitData::new(
                EvaluationUnitCategory::Track(None),
                EvaluationUnitField::DamagedSector,
                "Damaged sectors"
            ),
        }
    }
}