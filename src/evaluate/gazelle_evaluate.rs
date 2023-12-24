use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use crate::parser::ParsedLog;

use super::{DeductionCategory, DeductionField, DeductionData, Evaluator, Deduction};

pub mod red_evaluate;
pub mod ops_evaluate;

pub trait GazelleEvaluator: Evaluator {
    fn deduct(&mut self, data: dyn GazelleDeductionData) -> Deduction;
}

pub trait GazelleDeductionData {
    fn get_deduction_data(&self) -> DeductionData;
}

pub trait GazelleDeduction {
    // TODO: Do we need to send the ParsedLog for a single use case (drive not in DB check)?
    fn deduct(&self, parsed_log: &ParsedLog) -> Deduction;
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
    // FIXME: Unsecure T&C not handled (-40 points)
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
    fn get_deduction_data(&self) -> DeductionData {
        match &self {
            GazelleDeductionFail::UnknownEncoding => DeductionData::new(
                DeductionCategory::Release,
                DeductionField::Encoding,
                "Could not detect log encoding, log is corrupt"
            ),
            GazelleDeductionFail::UnknownRipper => DeductionData::new(
                DeductionCategory::Release,
                DeductionField::Ripper,
                "Unknown log file, could not determine ripper"
            ),
            GazelleDeductionFail::WhipperVersionLowerLimit => DeductionData::new(
                DeductionCategory::Release,
                DeductionField::RipperVersion,
                "Logs must be produced by whipper 0.7.3+"
            ),
            GazelleDeductionFail::CouldNotParseWhipper => DeductionData::new(
                DeductionCategory::Release,
                DeductionField::Ripper,
                "Could not parse whipper log"
            ),
        }
    }
}

impl GazelleDeductionData for GazelleDeductionRelease {
    fn get_deduction_data(&self) -> DeductionData {
        match &self {
            GazelleDeductionRelease::VirtualDrive => DeductionData::new(
                DeductionCategory::Release,
                DeductionField::Drive,
                "Virtual drive used"
            ),
            GazelleDeductionRelease::IncorrectReadOffset => DeductionData::new(
                DeductionCategory::Release,
                DeductionField::Offset,
                "Incorrect read offset for drive"
            ),
            GazelleDeductionRelease::DriveNotFoundDb => DeductionData::new(
                DeductionCategory::Release,
                DeductionField::Drive,
                "The drive was not found in the database"
            ),
            GazelleDeductionRelease::DefeatAudioCacheDisabled => DeductionData::new(
                DeductionCategory::Release,
                DeductionField::Cache,
                "\"Defeat audio cache\" should be Yes/true"
            ),
            GazelleDeductionRelease::EacVersionOld => DeductionData::new(
                DeductionCategory::Release,
                DeductionField::RipperVersion,
                "EAC version older than 0.99"
            ),
            GazelleDeductionRelease::XldNoChecksum => DeductionData::new(
                DeductionCategory::Release,
                DeductionField::Checksum,
                "No checksum with XLD 20121222 or newer"
            ),
            GazelleDeductionRelease::Mp3Log => DeductionData::new(
                DeductionCategory::Release,
                DeductionField::Encoder,
                "Invalid Log (MP3)"
            ),
            GazelleDeductionRelease::CouldNotVerifyDrive => DeductionData::new(
                DeductionCategory::Release,
                DeductionField::Drive,
                "Could not verify used drive"
            ),
            GazelleDeductionRelease::CouldNotVerifyMedia => DeductionData::new(
                DeductionCategory::Release,
                DeductionField::MediaType,
                "Could not verify media type"
            ),
            GazelleDeductionRelease::CouldNotVerifyReadMode => DeductionData::new(
                DeductionCategory::Release,
                DeductionField::ReadMode,
                "Could not verify read mode"
            ),
            GazelleDeductionRelease::CouldNotVerifyMaxRetry => DeductionData::new(
                DeductionCategory::Release,
                DeductionField::MaxRetryCount,
                "Could not verify max retry count"
            ),
            GazelleDeductionRelease::CouldNotVerifyAccurateStream => DeductionData::new(
                DeductionCategory::Release,
                DeductionField::AccurateStream,
                "Could not verify accurate stream"
            ),
            GazelleDeductionRelease::CouldNotVerifyDefeatAudioCache => DeductionData::new(
                DeductionCategory::Release,
                DeductionField::Cache,
                "Could not verify defeat audio cache"
            ),
            GazelleDeductionRelease::CouldNotVerifyC2 => DeductionData::new(
                DeductionCategory::Release,
                DeductionField::C2,
                "Could not verify C2 pointers"
            ),
            GazelleDeductionRelease::CouldNotVerifyReadOffset => DeductionData::new(
                DeductionCategory::Release,
                DeductionField::Offset,
                "Could not verify read offset"
            ),
            GazelleDeductionRelease::CombinedOffsetUnverifiable => DeductionData::new(
                DeductionCategory::Release,
                DeductionField::Offset,
                "Combined read/write offset cannot be verified"
            ),
            GazelleDeductionRelease::CouldNotVerifyMissingOffsetSamples => DeductionData::new(
                DeductionCategory::Release,
                DeductionField::Samples,
                "Could not verify missing offset samples"
            ),
            GazelleDeductionRelease::CouldNotVerifySilentBlocks => DeductionData::new(
                DeductionCategory::Release,
                DeductionField::SilentBlocks,
                "Could not verify silent blocks"
            ),
            GazelleDeductionRelease::CouldNotVerifyNullSamples => DeductionData::new(
                DeductionCategory::Release,
                DeductionField::NullSamples,
                "Could not verify null samples"
            ),
            GazelleDeductionRelease::CouldNotVerifyGapHandling => DeductionData::new(
                DeductionCategory::Release,
                DeductionField::MediaType,
                "Could not verify gap handling"
            ),
            GazelleDeductionRelease::CouldNotVerifyId3 => DeductionData::new(
                DeductionCategory::Release,
                DeductionField::Tag,
                "Could not verify id3 tag setting"
            ),
            GazelleDeductionRelease::CouldNotVerifyAlbumGain => DeductionData::new(
                DeductionCategory::Release,
                DeductionField::Gain,
                "Could not verify album gain"
            ),
            GazelleDeductionRelease::RippedWithCompressionOffset => DeductionData::new(
                DeductionCategory::Release,
                DeductionField::Offset,
                "Ripped with compression offset"
            ),
            GazelleDeductionRelease::RangeRip => DeductionData::new(
                DeductionCategory::Release,
                DeductionField::RangeSplit,
                "Range rip detected"
            ),
            GazelleDeductionRelease::TestAndCopyNotUsed => DeductionData::new(
                DeductionCategory::Release,
                DeductionField::TestAndCopy,
                "Test and copy was not used"
            ),
            GazelleDeductionRelease::RipModeNotSecure => DeductionData::new(
                DeductionCategory::Release,
                DeductionField::ReadMode,
                "Rip mode not secure"
            ),
            GazelleDeductionRelease::NotPressedCd => DeductionData::new(
                DeductionCategory::Release,
                DeductionField::MediaType,
                "Not a pressed cd"
            ),
            GazelleDeductionRelease::LowMaxRetryCount => DeductionData::new(
                DeductionCategory::Release,
                DeductionField::MaxRetryCount,
                "Low \"max retry count\" (potentially bad setting)"
            ),
            GazelleDeductionRelease::AccurateStreamNotUtilized => DeductionData::new(
                DeductionCategory::Release,
                DeductionField::AccurateStream,
                "\"Utilize accurate stream\" should be yes"
            ),
            GazelleDeductionRelease::UsedC2 => DeductionData::new(
                DeductionCategory::Release,
                DeductionField::C2,
                "C2 pointers were used"
            ),
            GazelleDeductionRelease::DoesNotFillMissingOffsetSamples => DeductionData::new(
                DeductionCategory::Release,
                DeductionField::Samples,
                "Does not fill up missing offset samples with silence"
            ),
            GazelleDeductionRelease::LeadingTrailingBlocksDeleted => DeductionData::new(
                DeductionCategory::Release,
                DeductionField::SilentBlocks,
                "Deletes leading and trailing silent blocks"
            ),
            GazelleDeductionRelease::NullSamplesNotUsed => DeductionData::new(
                DeductionCategory::Release,
                DeductionField::NullSamples,
                "Null samples should be used in CRC calculations"
            ),
            GazelleDeductionRelease::NormalizationUsed => DeductionData::new(
                DeductionCategory::Release,
                DeductionField::Normalization,
                "Normalization should be not be active"
            ),
            GazelleDeductionRelease::IncorrectGapHandling => DeductionData::new(
                DeductionCategory::Release,
                DeductionField::Gap,
                "Incorrect gap handling"
            ),
            GazelleDeductionRelease::NotSecureCrcMismatch => DeductionData::new(
                DeductionCategory::Release,
                DeductionField::TestAndCopy,
                "Rip was not done in Secure mode, and experienced CRC mismatches"
            ),
            GazelleDeductionRelease::Id3OnFlac => DeductionData::new(
                DeductionCategory::Release,
                DeductionField::Tag,
                "ID3 tags should not be added to FLAC files - they are mainly for MP3 files."
            ),
        }
    }
}

impl GazelleDeductionData for GazelleDeductionTrack {
    fn get_deduction_data(&self) -> DeductionData {
        match &self {
            GazelleDeductionTrack::CouldNotVerifyFilenameTooLong => DeductionData::new(
                DeductionCategory::Track(None),
                DeductionField::Filename,
                "Could not verify filename, too long"
            ),
            GazelleDeductionTrack::CouldNotVerifyFilenameOrExt => DeductionData::new(
                DeductionCategory::Track(None),
                DeductionField::Filename,
                "Could not verify filename or file extension"
            ),
            GazelleDeductionTrack::CouldNotVerifyReadErrors => DeductionData::new(
                DeductionCategory::Track(None),
                DeductionField::Filename,
                "Could not verify read errors"
            ),
            GazelleDeductionTrack::CouldNotVerifySkippedErrors => DeductionData::new(
                DeductionCategory::Track(None),
                DeductionField::SkipError,
                "Could not verify skipped errors"
            ),
            GazelleDeductionTrack::CouldNotVerifyEdgeJitterErrors => DeductionData::new(
                DeductionCategory::Track(None),
                DeductionField::JitterEdgeError,
                "Could not verify edge jitter errors"
            ),
            GazelleDeductionTrack::CouldNotVerifyAtomJitterErrors => DeductionData::new(
                DeductionCategory::Track(None),
                DeductionField::JitterAtomError,
                "Could not verify atom jitter errors"
            ),
            GazelleDeductionTrack::CouldNotVerifyJitterErrors => DeductionData::new(
                DeductionCategory::Track(None),
                DeductionField::JitterGenericError,
                "Could not verify jitter errors"
            ),
            GazelleDeductionTrack::CouldNotVerifyRetrySectorCount => DeductionData::new(
                DeductionCategory::Track(None),
                DeductionField::MaxRetryCount,
                "Could not verify retry sector count"
            ),
            GazelleDeductionTrack::CouldNotVerifyDamagedSectorCount => DeductionData::new(
                DeductionCategory::Track(None),
                DeductionField::DamagedSector,
                "Could not verify damaged sector count"
            ),
            GazelleDeductionTrack::CouldNotVerifyDriftErrors => DeductionData::new(
                DeductionCategory::Track(None),
                DeductionField::DriftError,
                "Could not verify drift errors"
            ),
            GazelleDeductionTrack::CouldNotVerifyDroppedBytesErrors => DeductionData::new(
                DeductionCategory::Track(None),
                DeductionField::DroppedError,
                "Could not verify dropped bytes errors"
            ),
            GazelleDeductionTrack::CouldNotVerifyDuplicatedBytesErrors => DeductionData::new(
                DeductionCategory::Track(None),
                DeductionField::DuplicatedError,
                "Could not verify duplicated bytes errors"
            ),
            GazelleDeductionTrack::CouldNotVerifyInconsistentErrorSectors => DeductionData::new(
                DeductionCategory::Track(None),
                DeductionField::InconsistentErrorSectors,
                "Could not verify inconsistent error sectors"
            ),
            GazelleDeductionTrack::SusPositionsFound => DeductionData::new(
                DeductionCategory::Track(None),
                DeductionField::ReadError,
                "Suspicious position(s) found"
            ),
            GazelleDeductionTrack::TimingProblemsFound => DeductionData::new(
                DeductionCategory::Track(None),
                DeductionField::JitterGenericError,
                "Timing problem(s) found"
            ),
            GazelleDeductionTrack::MissingSamplesFound => DeductionData::new(
                DeductionCategory::Track(None),
                DeductionField::DamagedSector,
                "Missing sample(s) found"
            ),
            GazelleDeductionTrack::CopyAborted => DeductionData::new(
                DeductionCategory::Track(None),
                DeductionField::Abort,
                "Copy aborted"
            ),
            GazelleDeductionTrack::CrcMismatch => DeductionData::new(
                DeductionCategory::Track(None),
                DeductionField::TestAndCopy,
                "CRC mismatch"
            ),
            GazelleDeductionTrack::ReadErrors(_) => DeductionData::new(
                DeductionCategory::Track(None),
                DeductionField::ReadError,
                "Read error"
            ),
            GazelleDeductionTrack::SkippedErrors(_) => DeductionData::new(
                DeductionCategory::Track(None),
                DeductionField::SkipError,
                "Skipped error"
            ),
            GazelleDeductionTrack::InconsistenciesInErrorSectors(_) => DeductionData::new(
                DeductionCategory::Track(None),
                DeductionField::InconsistentErrorSectors,
                "Inconsistencies in error sectors detected"
            ),
            GazelleDeductionTrack::DamagedSectors(_) => DeductionData::new(
                DeductionCategory::Track(None),
                DeductionField::DamagedSector,
                "Damaged sectors"
            ),
        }
    }
}