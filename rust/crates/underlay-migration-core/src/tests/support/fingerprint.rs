use crate::DecisionFingerprintInput;

pub(in crate::tests) fn decision_fingerprint_for(input: &DecisionFingerprintInput) -> String {
    crate::decision_fingerprint(input).expect("decision fingerprint should encode")
}
