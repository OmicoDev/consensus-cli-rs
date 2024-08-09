use clap::builder::PossibleValue;
use clap::ValueEnum;
use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum VersionStage {
    Snapshot,
    Alpha,
    Beta,
    ReleaseCandidate,
    Stable,
}

impl VersionStage {
    pub(crate) fn value(&self) -> &str {
        match self {
            VersionStage::Snapshot => "SNAPSHOT",
            VersionStage::Alpha => "alpha",
            VersionStage::Beta => "beta",
            VersionStage::ReleaseCandidate => "rc",
            VersionStage::Stable => "",
        }
    }
}

impl fmt::Display for VersionStage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            VersionStage::Snapshot => VERSION_STAGE_SNAPSHOT,
            VersionStage::Alpha => VERSION_STAGE_ALPHA,
            VersionStage::Beta => VERSION_STAGE_BETA,
            VersionStage::ReleaseCandidate => VERSION_STAGE_RC,
            VersionStage::Stable => VERSION_STAGE_STABLE,
        };
        write!(f, "{}", value)
    }
}

impl TryFrom<String> for VersionStage {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            VERSION_STAGE_SNAPSHOT => Ok(VersionStage::Snapshot),
            VERSION_STAGE_ALPHA => Ok(VersionStage::Alpha),
            VERSION_STAGE_BETA => Ok(VersionStage::Beta),
            VERSION_STAGE_RC => Ok(VersionStage::ReleaseCandidate),
            VERSION_STAGE_STABLE => Ok(VersionStage::Stable),
            _ => Err(format!("Invalid version stage: {}", value)),
        }
    }
}

const VERSION_STAGE_VALUE_VARIANTS: [VersionStage; 5] = [
    VersionStage::Snapshot,
    VersionStage::Alpha,
    VersionStage::Beta,
    VersionStage::ReleaseCandidate,
    VersionStage::Stable,
];

impl ValueEnum for VersionStage {
    fn value_variants<'a>() -> &'a [Self] {
        &VERSION_STAGE_VALUE_VARIANTS
    }

    fn from_str(input: &str, ignore_case: bool) -> Result<Self, String> {
        let input = if ignore_case {
            input.to_lowercase()
        } else {
            input.to_string()
        };
        VersionStage::try_from(input)
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        match self {
            VersionStage::Snapshot => Some(PossibleValue::new(VERSION_STAGE_SNAPSHOT)),
            VersionStage::Alpha => Some(PossibleValue::new(VERSION_STAGE_ALPHA)),
            VersionStage::Beta => Some(PossibleValue::new(VERSION_STAGE_BETA)),
            VersionStage::ReleaseCandidate => Some(PossibleValue::new(VERSION_STAGE_RC)),
            VersionStage::Stable => Some(PossibleValue::new(VERSION_STAGE_STABLE)),
        }
    }
}

const VERSION_STAGE_SNAPSHOT: &str = "snapshot";
const VERSION_STAGE_ALPHA: &str = "alpha";
const VERSION_STAGE_BETA: &str = "beta";
const VERSION_STAGE_RC: &str = "rc";
const VERSION_STAGE_STABLE: &str = "stable";
