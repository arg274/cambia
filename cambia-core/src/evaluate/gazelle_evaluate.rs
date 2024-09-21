use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use crate::parser::ParsedLog;

use super::{EvaluationUnitScope, EvaluationUnitField, EvaluationUnitData, Evaluator, EvaluationUnit, EvaluationUnitClass};

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
                EvaluationUnitScope::Release,
                EvaluationUnitField::Encoding,
                "Could not detect log encoding, log is corrupt",
                EvaluationUnitClass::Critical
            ),
            GazelleDeductionFail::UnknownRipper => EvaluationUnitData::new(
                EvaluationUnitScope::Release,
                EvaluationUnitField::Ripper,
                "Unknown log file, could not determine ripper",
                EvaluationUnitClass::Critical
            ),
            GazelleDeductionFail::WhipperVersionLowerLimit => EvaluationUnitData::new(
                EvaluationUnitScope::Release,
                EvaluationUnitField::RipperVersion,
                "Logs must be produced by whipper 0.7.3+",
                EvaluationUnitClass::Critical
            ),
            GazelleDeductionFail::CouldNotParseWhipper => EvaluationUnitData::new(
                EvaluationUnitScope::Release,
                EvaluationUnitField::Ripper,
                "Could not parse whipper log",
                EvaluationUnitClass::Critical
            ),
        }
    }
}

impl GazelleDeductionData for GazelleDeductionRelease {
    fn get_deduction_data(&self) -> EvaluationUnitData {
        match &self {
            GazelleDeductionRelease::VirtualDrive => EvaluationUnitData::new(
                EvaluationUnitScope::Release,
                EvaluationUnitField::Drive,
                "Virtual drive used",
                EvaluationUnitClass::Critical
            ),
            GazelleDeductionRelease::NullDrive => EvaluationUnitData::new(
                EvaluationUnitScope::Release,
                EvaluationUnitField::Drive,
                "Null drive used",
                EvaluationUnitClass::Critical
            ),
            GazelleDeductionRelease::IncorrectReadOffset => EvaluationUnitData::new(
                EvaluationUnitScope::Release,
                EvaluationUnitField::Offset,
                "Incorrect read offset for drive",
                EvaluationUnitClass::Bad
            ),
            GazelleDeductionRelease::DriveNotFoundDb => EvaluationUnitData::new(
                EvaluationUnitScope::Release,
                EvaluationUnitField::Drive,
                "The drive was not found in the database",
                EvaluationUnitClass::Neutral
            ),
            GazelleDeductionRelease::DefeatAudioCacheDisabled => EvaluationUnitData::new(
                EvaluationUnitScope::Release,
                EvaluationUnitField::Cache,
                "\"Defeat audio cache\" should be Yes/true",
                EvaluationUnitClass::Bad
            ),
            GazelleDeductionRelease::EacVersionOld => EvaluationUnitData::new(
                EvaluationUnitScope::Release,
                EvaluationUnitField::RipperVersion,
                "EAC version older than 0.99",
                EvaluationUnitClass::Bad
            ),
            GazelleDeductionRelease::XldNoChecksum => EvaluationUnitData::new(
                EvaluationUnitScope::Release,
                EvaluationUnitField::Checksum,
                "No checksum with XLD 20121222 or newer",
                EvaluationUnitClass::Bad
            ),
            GazelleDeductionRelease::Mp3Log => EvaluationUnitData::new(
                EvaluationUnitScope::Release,
                EvaluationUnitField::Encoder,
                "Invalid Log (MP3)",
                EvaluationUnitClass::Critical
            ),
            GazelleDeductionRelease::CouldNotVerifyDrive => EvaluationUnitData::new(
                EvaluationUnitScope::Release,
                EvaluationUnitField::Drive,
                "Could not verify used drive",
                EvaluationUnitClass::Bad
            ),
            GazelleDeductionRelease::CouldNotVerifyMedia => EvaluationUnitData::new(
                EvaluationUnitScope::Release,
                EvaluationUnitField::MediaType,
                "Could not verify media type",
                EvaluationUnitClass::Bad
            ),
            GazelleDeductionRelease::CouldNotVerifyReadMode => EvaluationUnitData::new(
                EvaluationUnitScope::Release,
                EvaluationUnitField::ReadMode,
                "Could not verify read mode",
                EvaluationUnitClass::Bad
            ),
            GazelleDeductionRelease::CouldNotVerifyMaxRetry => EvaluationUnitData::new(
                EvaluationUnitScope::Release,
                EvaluationUnitField::MaxRetryCount,
                "Could not verify max retry count",
                EvaluationUnitClass::Neutral
            ),
            GazelleDeductionRelease::CouldNotVerifyAccurateStream => EvaluationUnitData::new(
                EvaluationUnitScope::Release,
                EvaluationUnitField::AccurateStream,
                "Could not verify accurate stream",
                EvaluationUnitClass::Critical
            ),
            GazelleDeductionRelease::CouldNotVerifyDefeatAudioCache => EvaluationUnitData::new(
                EvaluationUnitScope::Release,
                EvaluationUnitField::Cache,
                "Could not verify defeat audio cache",
                EvaluationUnitClass::Bad
            ),
            GazelleDeductionRelease::CouldNotVerifyC2 => EvaluationUnitData::new(
                EvaluationUnitScope::Release,
                EvaluationUnitField::C2,
                "Could not verify C2 pointers",
                EvaluationUnitClass::Bad
            ),
            GazelleDeductionRelease::CouldNotVerifyReadOffset => EvaluationUnitData::new(
                EvaluationUnitScope::Release,
                EvaluationUnitField::Offset,
                "Could not verify read offset",
                EvaluationUnitClass::Bad
            ),
            GazelleDeductionRelease::CombinedOffsetUnverifiable => EvaluationUnitData::new(
                EvaluationUnitScope::Release,
                EvaluationUnitField::Offset,
                "Combined read/write offset cannot be verified",
                EvaluationUnitClass::Bad
            ),
            GazelleDeductionRelease::CouldNotVerifyMissingOffsetSamples => EvaluationUnitData::new(
                EvaluationUnitScope::Release,
                EvaluationUnitField::Samples,
                "Could not verify missing offset samples",
                EvaluationUnitClass::Bad
            ),
            GazelleDeductionRelease::CouldNotVerifySilentBlocks => EvaluationUnitData::new(
                EvaluationUnitScope::Release,
                EvaluationUnitField::SilentBlocks,
                "Could not verify silent blocks",
                EvaluationUnitClass::Bad
            ),
            GazelleDeductionRelease::CouldNotVerifyNullSamples => EvaluationUnitData::new(
                EvaluationUnitScope::Release,
                EvaluationUnitField::NullSamples,
                "Could not verify null samples",
                EvaluationUnitClass::Neutral
            ),
            GazelleDeductionRelease::CouldNotVerifyGapHandling => EvaluationUnitData::new(
                EvaluationUnitScope::Release,
                EvaluationUnitField::Gap,
                "Could not verify gap handling",
                EvaluationUnitClass::Bad
            ),
            GazelleDeductionRelease::CouldNotVerifyId3 => EvaluationUnitData::new(
                EvaluationUnitScope::Release,
                EvaluationUnitField::Tag,
                "Could not verify id3 tag setting",
                EvaluationUnitClass::Neutral
            ),
            GazelleDeductionRelease::CouldNotVerifyAlbumGain => EvaluationUnitData::new(
                EvaluationUnitScope::Release,
                EvaluationUnitField::Gain,
                "Could not verify album gain",
                EvaluationUnitClass::Neutral
            ),
            GazelleDeductionRelease::RippedWithCompressionOffset => EvaluationUnitData::new(
                EvaluationUnitScope::Release,
                EvaluationUnitField::Offset,
                "Ripped with compression offset",
                EvaluationUnitClass::Neutral
            ),
            GazelleDeductionRelease::RangeRip => EvaluationUnitData::new(
                EvaluationUnitScope::Release,
                EvaluationUnitField::RangeSplit,
                "Range rip detected",
                EvaluationUnitClass::Bad
            ),
            GazelleDeductionRelease::TestAndCopyNotUsed => EvaluationUnitData::new(
                EvaluationUnitScope::Release,
                EvaluationUnitField::TestAndCopy,
                "Test and copy was not used",
                EvaluationUnitClass::Bad
            ),
            GazelleDeductionRelease::RipModeNotSecure => EvaluationUnitData::new(
                EvaluationUnitScope::Release,
                EvaluationUnitField::ReadMode,
                "Rip mode not secure",
                EvaluationUnitClass::Bad
            ),
            GazelleDeductionRelease::NotPressedCd => EvaluationUnitData::new(
                EvaluationUnitScope::Release,
                EvaluationUnitField::MediaType,
                "Not a pressed cd",
                EvaluationUnitClass::Neutral
            ),
            GazelleDeductionRelease::LowMaxRetryCount => EvaluationUnitData::new(
                EvaluationUnitScope::Release,
                EvaluationUnitField::MaxRetryCount,
                "Low \"max retry count\" (potentially bad setting)",
                EvaluationUnitClass::Neutral
            ),
            GazelleDeductionRelease::AccurateStreamNotUtilized => EvaluationUnitData::new(
                EvaluationUnitScope::Release,
                EvaluationUnitField::AccurateStream,
                "\"Utilize accurate stream\" should be yes",
                EvaluationUnitClass::Bad
            ),
            GazelleDeductionRelease::UsedC2 => EvaluationUnitData::new(
                EvaluationUnitScope::Release,
                EvaluationUnitField::C2,
                "C2 pointers were used",
                EvaluationUnitClass::Bad
            ),
            GazelleDeductionRelease::DoesNotFillMissingOffsetSamples => EvaluationUnitData::new(
                EvaluationUnitScope::Release,
                EvaluationUnitField::Samples,
                "Does not fill up missing offset samples with silence",
                EvaluationUnitClass::Bad
            ),
            GazelleDeductionRelease::LeadingTrailingBlocksDeleted => EvaluationUnitData::new(
                EvaluationUnitScope::Release,
                EvaluationUnitField::SilentBlocks,
                "Deletes leading and trailing silent blocks",
                EvaluationUnitClass::Bad
            ),
            GazelleDeductionRelease::NullSamplesNotUsed => EvaluationUnitData::new(
                EvaluationUnitScope::Release,
                EvaluationUnitField::NullSamples,
                "Null samples should be used in CRC calculations",
                EvaluationUnitClass::Bad
            ),
            GazelleDeductionRelease::NormalizationUsed => EvaluationUnitData::new(
                EvaluationUnitScope::Release,
                EvaluationUnitField::Normalization,
                "Normalization should be not be active",
                EvaluationUnitClass::Critical
            ),
            GazelleDeductionRelease::IncorrectGapHandling => EvaluationUnitData::new(
                EvaluationUnitScope::Release,
                EvaluationUnitField::Gap,
                "Incorrect gap handling",
                EvaluationUnitClass::Bad
            ),
            GazelleDeductionRelease::NotSecureCrcMismatch => EvaluationUnitData::new(
                EvaluationUnitScope::Release,
                EvaluationUnitField::TestAndCopy,
                "Rip was not done in Secure mode, and experienced CRC mismatches",
                EvaluationUnitClass::Bad
            ),
            GazelleDeductionRelease::NotSecureNoTC => EvaluationUnitData::new(
                EvaluationUnitScope::Release,
                EvaluationUnitField::TestAndCopy,
                "Rip was not done in Secure mode, and T+C was not used - as a result, we cannot verify the authenticity of the rip",
                EvaluationUnitClass::Critical
            ),
            GazelleDeductionRelease::Id3OnFlac => EvaluationUnitData::new(
                EvaluationUnitScope::Release,
                EvaluationUnitField::Tag,
                "ID3 tags should not be added to FLAC files - they are mainly for MP3 files.",
                EvaluationUnitClass::Neutral
            ),
        }
    }
}

impl GazelleDeductionData for GazelleDeductionTrack {
    fn get_deduction_data(&self) -> EvaluationUnitData {
        match &self {
            GazelleDeductionTrack::CouldNotVerifyFilenameTooLong => EvaluationUnitData::new(
                EvaluationUnitScope::Track(None),
                EvaluationUnitField::Filename,
                "Could not verify filename, too long",
                EvaluationUnitClass::Neutral
            ),
            GazelleDeductionTrack::CouldNotVerifyFilenameOrExt => EvaluationUnitData::new(
                EvaluationUnitScope::Track(None),
                EvaluationUnitField::Filename,
                "Could not verify filename or file extension",
                EvaluationUnitClass::Bad
            ),
            GazelleDeductionTrack::CouldNotVerifyReadErrors => EvaluationUnitData::new(
                EvaluationUnitScope::Track(None),
                EvaluationUnitField::Filename,
                "Could not verify read errors",
                EvaluationUnitClass::Neutral
            ),
            GazelleDeductionTrack::CouldNotVerifySkippedErrors => EvaluationUnitData::new(
                EvaluationUnitScope::Track(None),
                EvaluationUnitField::SkipError,
                "Could not verify skipped errors",
                EvaluationUnitClass::Neutral
            ),
            GazelleDeductionTrack::CouldNotVerifyEdgeJitterErrors => EvaluationUnitData::new(
                EvaluationUnitScope::Track(None),
                EvaluationUnitField::JitterEdgeError,
                "Could not verify edge jitter errors",
                EvaluationUnitClass::Neutral
            ),
            GazelleDeductionTrack::CouldNotVerifyAtomJitterErrors => EvaluationUnitData::new(
                EvaluationUnitScope::Track(None),
                EvaluationUnitField::JitterAtomError,
                "Could not verify atom jitter errors",
                EvaluationUnitClass::Neutral
            ),
            GazelleDeductionTrack::CouldNotVerifyJitterErrors => EvaluationUnitData::new(
                EvaluationUnitScope::Track(None),
                EvaluationUnitField::JitterGenericError,
                "Could not verify jitter errors",
                EvaluationUnitClass::Neutral
            ),
            GazelleDeductionTrack::CouldNotVerifyRetrySectorCount => EvaluationUnitData::new(
                EvaluationUnitScope::Track(None),
                EvaluationUnitField::MaxRetryCount,
                "Could not verify retry sector count",
                EvaluationUnitClass::Neutral
            ),
            GazelleDeductionTrack::CouldNotVerifyDamagedSectorCount => EvaluationUnitData::new(
                EvaluationUnitScope::Track(None),
                EvaluationUnitField::DamagedSector,
                "Could not verify damaged sector count",
                EvaluationUnitClass::Neutral
            ),
            GazelleDeductionTrack::CouldNotVerifyDriftErrors => EvaluationUnitData::new(
                EvaluationUnitScope::Track(None),
                EvaluationUnitField::DriftError,
                "Could not verify drift errors",
                EvaluationUnitClass::Neutral
            ),
            GazelleDeductionTrack::CouldNotVerifyDroppedBytesErrors => EvaluationUnitData::new(
                EvaluationUnitScope::Track(None),
                EvaluationUnitField::DroppedError,
                "Could not verify dropped bytes errors",
                EvaluationUnitClass::Neutral
            ),
            GazelleDeductionTrack::CouldNotVerifyDuplicatedBytesErrors => EvaluationUnitData::new(
                EvaluationUnitScope::Track(None),
                EvaluationUnitField::DuplicatedError,
                "Could not verify duplicated bytes errors",
                EvaluationUnitClass::Neutral
            ),
            GazelleDeductionTrack::CouldNotVerifyInconsistentErrorSectors => EvaluationUnitData::new(
                EvaluationUnitScope::Track(None),
                EvaluationUnitField::InconsistentErrorSectors,
                "Could not verify inconsistent error sectors",
                EvaluationUnitClass::Neutral
            ),
            GazelleDeductionTrack::SusPositionsFound => EvaluationUnitData::new(
                EvaluationUnitScope::Track(None),
                EvaluationUnitField::ReadError,
                "Suspicious position(s) found",
                EvaluationUnitClass::Bad
            ),
            GazelleDeductionTrack::TimingProblemsFound => EvaluationUnitData::new(
                EvaluationUnitScope::Track(None),
                EvaluationUnitField::JitterGenericError,
                "Timing problem(s) found",
                EvaluationUnitClass::Bad
            ),
            GazelleDeductionTrack::MissingSamplesFound => EvaluationUnitData::new(
                EvaluationUnitScope::Track(None),
                EvaluationUnitField::DamagedSector,
                "Missing sample(s) found",
                EvaluationUnitClass::Bad
            ),
            GazelleDeductionTrack::CopyAborted => EvaluationUnitData::new(
                EvaluationUnitScope::Track(None),
                EvaluationUnitField::Abort,
                "Copy aborted",
                EvaluationUnitClass::Critical
            ),
            GazelleDeductionTrack::CrcMismatch => EvaluationUnitData::new(
                EvaluationUnitScope::Track(None),
                EvaluationUnitField::TestAndCopy,
                "CRC mismatch",
                EvaluationUnitClass::Bad
            ),
            GazelleDeductionTrack::ReadErrors(_) => EvaluationUnitData::new(
                EvaluationUnitScope::Track(None),
                EvaluationUnitField::ReadError,
                "Read error",
                EvaluationUnitClass::Bad
            ),
            GazelleDeductionTrack::SkippedErrors(_) => EvaluationUnitData::new(
                EvaluationUnitScope::Track(None),
                EvaluationUnitField::SkipError,
                "Skipped error",
                EvaluationUnitClass::Bad
            ),
            GazelleDeductionTrack::InconsistenciesInErrorSectors(_) => EvaluationUnitData::new(
                EvaluationUnitScope::Track(None),
                EvaluationUnitField::InconsistentErrorSectors,
                "Inconsistencies in error sectors detected",
                EvaluationUnitClass::Bad
            ),
            GazelleDeductionTrack::DamagedSectors(_) => EvaluationUnitData::new(
                EvaluationUnitScope::Track(None),
                EvaluationUnitField::DamagedSector,
                "Damaged sectors",
                EvaluationUnitClass::Bad
            ),
        }
    }
}