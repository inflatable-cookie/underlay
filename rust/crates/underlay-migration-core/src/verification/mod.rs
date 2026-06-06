mod artifact;
mod checksum;
mod model;
mod stage;

pub use artifact::build_verification_artifact;
pub use checksum::transform_checksum;
pub use model::{
    VerificationArtifact, VerificationCheckResult, VerificationChecksumSection, VerificationInput,
    VerificationIntegrityGateSection, VerificationIssue, VerificationPromotionGate,
    VerificationReferentialIntegritySection, VerificationReport, VerificationRowCountSection,
    VerificationSeverity,
};
pub use stage::verify_stage;

#[cfg(test)]
#[path = "../tests/verification_tests.rs"]
mod tests;
